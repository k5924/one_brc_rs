use one_brc_rs::chunk_processor::process_chunk;
use one_brc_rs::station::Station;
use hashbrown::HashMap;

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
