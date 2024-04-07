use crate::float_parser::parse_float;
use crate::Station;
use crate::map_processor::update_or_create;
use hashbrown::HashMap;

pub fn process_line(line: &[u8], map: &mut HashMap<String, Station>) {
    // Find the semicolon index without bounds checking
    let mut ptr = line.as_ptr();
    let end_ptr = unsafe { ptr.add(line.len()) };
    while unsafe { *ptr } != b';' {
        ptr = unsafe { ptr.add(1) };
        if ptr == end_ptr {
            return;
        }
    }

    // Convert byte slices to strings without bounds checking
    let semicolon_index = unsafe { ptr.offset_from(line.as_ptr()) as usize };
    let part2_float = unsafe { parse_float(&line[semicolon_index + 1..]) };
    let key = unsafe { std::str::from_utf8_unchecked(&line[..semicolon_index]) };
    update_or_create(map, key, part2_float);
}

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
