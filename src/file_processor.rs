use std::fs::File;
use std::io::{self};
use std::slice;
use std::os::unix::io::AsRawFd;
use std::ptr;
use libc::{sysconf, _SC_PAGESIZE, _SC_NPROCESSORS_CONF};
use core::ops::Range;
use crate::chunk_processor::process_chunk;
use crate::utils::is_newline_char;
use crate::Station;
use hashbrown::HashMap;
use rayon::prelude::*;

unsafe fn get_page_size() -> io::Result<usize> {
    let page_size = sysconf(_SC_PAGESIZE);
    if page_size == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(page_size as usize)
    }
}

fn find_chunk_ranges(slice: &[u8], min_chunks: usize) -> io::Result<Vec<Range<usize>>> {
    let total_len = slice.len();
    let chunk_size = total_len / min_chunks;
    let mut chunks = Vec::with_capacity(min_chunks);
    let mut start = 0;

    while start < total_len {
        let mut end = start + chunk_size;
        if end > total_len {
            end = total_len;
        } else {
            // Ensure the chunk ends at a newline character if possible
            while end < total_len && !is_newline_char(unsafe {*slice.get_unchecked(end)}) {
                end += 1;
            }
        }
        chunks.push(start..end);
        start = end;
    }

    Ok(chunks)
}

fn mmap_file(file: &std::fs::File, min_chunks: usize) -> io::Result<(&[u8], Vec<Range<usize>>)> {
    let metadata = file.metadata()?;
    let file_size = metadata.len() as usize;

    let mmap_ptr = unsafe {
        libc::mmap(
            ptr::null_mut(),
            file_size,
            libc::PROT_READ,
            libc::MAP_PRIVATE | libc::MAP_POPULATE,
            file.as_raw_fd(),
            0,
        )
    };

    if mmap_ptr == libc::MAP_FAILED {
        return Err(io::Error::last_os_error());
    }

    let mmap_slice = unsafe { slice::from_raw_parts(mmap_ptr as *const u8, file_size) };
    let chunks = find_chunk_ranges(mmap_slice, min_chunks)?;
    Ok((mmap_slice, chunks))
}

pub fn process_file_single_thread(file: &File, map: &mut HashMap<String, Station>) -> io::Result<()> {
    let page_size = unsafe { get_page_size()? };

    let (mmap, chunks) = mmap_file(&file, page_size)?;
    
    for chunk in chunks {
        process_chunk(&mmap[chunk], map)?;
    }
    
    Ok(())
}

pub fn process_file_multiple_threads(file: &File, map: &mut HashMap<String, Station>) -> io::Result<()> {
    let num_threads = unsafe {sysconf(_SC_NPROCESSORS_CONF)} as usize;
    let (mmap, chunks) = mmap_file(&file, num_threads)?;

    let results: Vec<_> = chunks
        .into_par_iter()
        .map(|chunk| {
            let start = chunk.start;
            let end = chunk.end;
            let mut local_map = HashMap::new();
            process_chunk(&mmap[start..end], &mut local_map)?;
            Ok(local_map)
        })
        .collect::<io::Result<Vec<_>>>()?;

    for local_map in results {
        for (key, value) in local_map {
            let entry = map.raw_entry_mut().from_key(&key);

            // Handle the entry
            match entry {
                // If entry exists, update it
                hashbrown::hash_map::RawEntryMut::Occupied(mut occupied) => {
                    occupied.get_mut().merge(value);
                }
                // If entry doesn't exist, insert a new one
                hashbrown::hash_map::RawEntryMut::Vacant(vacant) => {
                    vacant.insert(key.to_string(), Station::new(0.0));
                }
            }
        }
    }

    Ok(())
}

