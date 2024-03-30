use std::fs::File;
use std::io::{self};
use std::slice;
use std::os::unix::io::AsRawFd;
use std::ptr;
use libc::{sysconf, _SC_PAGESIZE};
use core::ops::Range;
use crate::chunk_processor::process_chunk;
use crate::utils::is_newline_char;
use crate::Station;
use hashbrown::HashMap;

unsafe fn get_page_size() -> io::Result<usize> {
    let page_size = sysconf(_SC_PAGESIZE);
    if page_size == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(page_size as usize)
    }
}

fn find_chunk_ranges(slice: &[u8]) -> io::Result<Vec<Range<usize>>> {
    let total_len = slice.len();
    let page_size = unsafe { get_page_size()? };
    let mut chunks = Vec::with_capacity((total_len + page_size - 1) / page_size);
    let mut start = 0;

    while start < total_len {
        let mut end = start + page_size;
        if end > total_len {
            end = total_len;
        } else {
            // Ensure the chunk ends at a newline character if possible
            while end < total_len && !is_newline_char(slice[end]) {
                end += 1;
            }
        }
        chunks.push(start..end);
        start = end;
    }

    Ok(chunks)
}

fn mmap_file(file: &std::fs::File) -> io::Result<(&[u8], Vec<Range<usize>>)> {
    let metadata = file.metadata()?;
    let file_size = metadata.len() as usize;

    let mmap_ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            file_size,
            libc::PROT_READ,
            libc::MAP_PRIVATE,
            file.as_raw_fd(),
            0,
        )
    };

    if mmap_ptr == libc::MAP_FAILED {
        return Err(io::Error::last_os_error());
    }

    let mmap_slice = unsafe { slice::from_raw_parts(mmap_ptr as *const u8, file_size) };
    let chunks = find_chunk_ranges(mmap_slice)?;
    Ok((mmap_slice, chunks))
}

pub fn process_file(file: &File, map: &mut HashMap<String, Station>) -> io::Result<()> {

    let (mmap, chunks) = mmap_file(&file)?;
    
    for chunk in chunks {
        process_chunk(&mmap[chunk], map)?;
    }
    
    Ok(())
}

