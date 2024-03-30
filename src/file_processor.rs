use std::fs::File;
use std::io::{self};
use std::os::unix::io::AsRawFd;
use std::ptr;
use std::ffi::c_void;
use crate::line_parser::process_line;
use crate::Station;
use hashbrown::HashMap;

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

pub fn read_file_in_chunks(file: &File, map: &mut HashMap<String, Station>) -> io::Result<()> {
     // Determine page size
     let page_size = get_page_size();

     // Open file and get its metadata
     let metadata = file.metadata()?;
     let file_size = metadata.len() as usize;

     // Allocate memory for buffers
     let mut buffer = Vec::with_capacity(page_size);
     let mut line = Vec::with_capacity(page_size); // Reusable line buffer

     // Iterate through file in chunks
     let mut offset = 0;
     while offset < file_size {
         // Calculate chunk size for this iteration
         let chunk_size = page_size.min(file_size - offset);

         // Memory map the chunk
         let mmap_ptr = memory_map_chunk(file, offset, chunk_size)?;

         // Get a slice view of the memory mapped chunk
         let data = unsafe { std::slice::from_raw_parts(mmap_ptr as *const u8, chunk_size) };

         // Process the chunk
        for &byte in data {
            if byte == b'\n' {
                // Extend line with buffer content
                line.extend_from_slice(&buffer);
                
                // Split and print the line
                process_line(&line, map);
                
                // Clear the line buffer for reuse
                line.clear();
                
                // Clear the buffer after processing a line
                buffer.clear();
            } else {
                // Add byte to buffer
                buffer.push(byte);
            }
        }

        // Unmap memory after processing
        unmap_memory(mmap_ptr, chunk_size)?;

        // Move offset to the next chunk
        offset += chunk_size;
    }

    // Process any remaining content in the buffer
    if !buffer.is_empty() {
        // Extend line with remaining buffer content
        line.extend_from_slice(&buffer);
        
        // Split and print the line
        process_line(&line, map);
    }

    Ok(())
}

