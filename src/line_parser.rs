use crate::float_parser::parse_float;
use crate::Station;
use hashbrown::HashMap;

pub fn process_line(line: &[u8], map: &mut HashMap<String, Station>) {
    if let Some(index) = line.iter().position(|&byte| byte == b';') {
        let (part1, part2) = line.split_at(index);

        // Unsafe: Convert part1 to a String without checking for UTF-8 validity

        if let Some(part2_float) = unsafe {parse_float(&part2[1..]) } {
            // Convert part1_str to the appropriate key type if necessary
            let key = unsafe { std::str::from_utf8_unchecked(part1).to_owned() };

            // Get a mutable reference to the entry's value if it exists
            let entry = map.raw_entry_mut().from_key(&key);

            // Handle the entry
            match entry {
                // If entry exists, update it
                hashbrown::hash_map::RawEntryMut::Occupied(mut occupied) => {
                    occupied.get_mut().update(part2_float);
                }
                // If entry doesn't exist, insert a new one
                hashbrown::hash_map::RawEntryMut::Vacant(vacant) => {
                    vacant.insert(key, Station::new(part2_float));
                }
            }
        } else {
            eprintln!("Error parsing part2 as float");
        }
    } else {
        eprintln!("Error: No semicolon found in the line");
        eprintln!("Error: Invalid data: {:?}", line);
    }
}

