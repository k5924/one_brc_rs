use std::fs::File;
use std::io::{self};
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::ffi::c_void;

fn get_page_size() -> usize {
    unsafe { libc::sysconf(libc::_SC_PAGESIZE) as usize }
}

fn memory_map_chunk(file: &File, offset: usize, chunk_size: usize) -> io::Result<*mut c_void> {
    let mmap_ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            chunk_size,
            libc::PROT_READ,
            libc::MAP_PRIVATE,
            file.as_raw_fd(),
            offset as libc::off_t,
        )
    };

    if mmap_ptr == libc::MAP_FAILED {
        return Err(io::Error::last_os_error());
    }

    Ok(mmap_ptr)
}

fn unmap_memory(mmap_ptr: *mut c_void, chunk_size: usize) -> io::Result<()> {
    let result = unsafe { libc::munmap(mmap_ptr, chunk_size) };

    if result == -1 {
        return Err(io::Error::last_os_error());
    }

    Ok(())
}

fn split_and_print_line(line: &[u8]) {
    let mut field_start = 0;
    for (i, &byte) in line.iter().enumerate() {
        if byte == b';' {
            let field = &line[field_start..i];
            if let Ok(part1) = std::str::from_utf8(field) {
                if let Some(part2) = try_decode_utf8(&line[i + 1..]) {
                    println!("{}:{}", part1, part2);
                } else {
                    // Handle invalid UTF-8 bytes or unexpected characters
                    println!("{}:<invalid UTF-8>", part1);
                }
            }
            field_start = i + 1;
        }
    }
}

fn try_decode_utf8(bytes: &[u8]) -> Option<&str> {
    match std::str::from_utf8(bytes) {
        Ok(s) => Some(s),
        Err(_) => None,
    }
}

pub fn read_file_in_chunks(file: &File) -> io::Result<()> {
    let metadata = file.metadata()?;
    let file_size = metadata.len() as usize;
    let page_size = get_page_size();
    let mut offset = 0;
    let mut buffer = Vec::with_capacity(page_size);

    while offset < file_size {
        let chunk_size = page_size.min(file_size - offset);
        let mmap_ptr = memory_map_chunk(file, offset, chunk_size)?;

        let data = unsafe { std::slice::from_raw_parts(mmap_ptr as *const u8, chunk_size) };

        // Process the chunk
        let mut start = 0;
        for (i, &byte) in data.iter().enumerate() {
            if byte == b'\n' {
                let line = &data[start..i];
                split_and_print_line(line);
                start = i + 1;
            }
        }

        buffer.extend_from_slice(&data[start..]);

        unmap_memory(mmap_ptr, chunk_size)?;

        offset += chunk_size;
    }

    let mut last_new_line_idx = 0;
    for (i, &byte) in buffer.iter().enumerate() {
        if byte == b'\n' {
            let line = &buffer[last_new_line_idx..i];
            split_and_print_line(line);
            last_new_line_idx = i + 1;
        }
    }


    // Print the remaining content in the buffer if any
    if !last_new_line_idx < buffer.len() {
        split_and_print_line(&buffer[last_new_line_idx..]);
    }

    Ok(())
}

