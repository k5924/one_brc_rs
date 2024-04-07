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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    
    #[test]
    fn test_process_chunk_single_line() -> io::Result<()> {
        let mut map = HashMap::new();
        let chunk = b"key1;10.0\n";
        process_chunk(chunk, &mut map)?;

        assert_eq!(
            map.get("key1"),
            Some(&Station {
                min: 10.0,
                max: 10.0,
                sum: 10.0,
                count: 1,
            })
        );

        Ok(())
    }

    #[test]
    fn test_process_chunk_multiple_lines() -> io::Result<()> {
        let mut map = HashMap::new();
        let chunk = b"key1;10.0\nkey2;20.0\nkey1;30.0\n";
        process_chunk(chunk, &mut map)?;

        assert_eq!(
            map.get("key1"),
            Some(&Station {
                min: 10.0,
                max: 30.0,
                sum: 40.0,
                count: 2,
            })
        );
        assert_eq!(
            map.get("key2"),
            Some(&Station {
                min: 20.0,
                max: 20.0,
                sum: 20.0,
                count: 1,
            })
        );

        Ok(())
    }

    #[test]
    fn test_process_chunk_empty_input() -> io::Result<()> {
        let mut map = HashMap::new();
        let chunk = b"";
        process_chunk(chunk, &mut map)?;

        assert!(map.is_empty());

        Ok(())
    }

    #[test]
    fn test_process_chunk_newline_only() -> io::Result<()> {
        let mut map = HashMap::new();
        let chunk = b"\n\n";
        process_chunk(chunk, &mut map)?;

        assert!(map.is_empty());

        Ok(())
    }
}
