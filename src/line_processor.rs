use crate::float_parser::parse_float;
use crate::Station;
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
    let entry = map.raw_entry_mut().from_key(key);

    // Handle the entry
    match entry {
        // If entry exists, update it
        hashbrown::hash_map::RawEntryMut::Occupied(mut occupied) => {
            occupied.get_mut().update(part2_float);
        }
        // If entry doesn't exist, insert a new one
        hashbrown::hash_map::RawEntryMut::Vacant(vacant) => {
            vacant.insert(key.to_string(), Station::new(part2_float));
        }
    }
}
