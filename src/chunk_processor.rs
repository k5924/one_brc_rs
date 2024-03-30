use std::io;
use crate::line_processor::process_line;
use crate::utils::is_newline_char;
use crate::Station;
use hashbrown::HashMap;

pub fn process_chunk(slice: &[u8], map: &mut HashMap<String, Station>) -> io::Result<()> {
    let mut start = 0;
    let mut end = 0;

    // Find and process each line in the chunk
    while end < slice.len() {
        // Skip leading newline characters
        while start < slice.len() && is_newline_char(unsafe { *slice.get_unchecked(start) }) {
            start += 1;
        }

        // Find the end of the current line
        end = start;
        while end < slice.len() && !is_newline_char(unsafe { *slice.get_unchecked(end) }) {
            end += 1;
        }

        // Process the current line
        if start != end {
            let line = &slice[start..end];
            process_line(line, map);
        }

        // Move to the next line
        start = end + 1;
    }

    Ok(())
}
