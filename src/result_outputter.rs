use crate::station::Station;
use hashbrown::HashMap;

pub fn output_result(map: &mut HashMap<String, Station>) {
    // Get keys and sort them
    let mut keys: Vec<&String> = map.keys().collect();
    keys.sort_unstable();

    // Iterate over sorted keys and print corresponding values
    for key in keys {
        if let Some(value) = map.get(key) {
            println!("{}={}", key, value);
        }
    }
}
