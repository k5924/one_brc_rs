use one_brc_rs::station::Station;
use one_brc_rs::line_processor::process_line;
use hashbrown::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_line_existing_key() {
        let mut map = HashMap::new();
        map.insert(String::from("key1"), Station::new(10.0));

        let line = b"key1;20.0";
        process_line(line, &mut map);

        assert_eq!(
            map.get("key1"),
            Some(&Station {
                min: 10.0,
                max: 20.0,
                sum: 30.0,
                count: 2,
            })
        );
    }

    #[test]
    fn test_process_line_new_key() {
        let mut map = HashMap::new();

        let line = b"key2;30.0";
        process_line(line, &mut map);

        assert_eq!(
            map.get("key2"),
            Some(&Station {
                min: 30.0,
                max: 30.0,
                sum: 30.0,
                count: 1,
            })
        );
    }

    #[test]
    fn test_process_line_no_semicolon() {
        let mut map = HashMap::new();

        let line = b"no_semicolon";
        process_line(line, &mut map);

        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_process_line_empty_value() {
        let mut map = HashMap::new();

        let line = b"empty_value;";
        process_line(line, &mut map);

        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_process_line_empty_key() {
        let mut map = HashMap::new();

        let line = b";30.0";
        process_line(line, &mut map);

        assert_eq!(map.len(), 1);
    }

    #[test]
    fn test_process_line_invalid_value() {
        let mut map = HashMap::new();

        let line = b"key3;invalid_value";
        process_line(line, &mut map);

        assert_eq!(map.len(), 1);
    }
}
