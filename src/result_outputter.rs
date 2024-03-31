use crate::station::Station;
use hashbrown::HashMap;

pub fn output_result(map: &mut HashMap<String, Station>) {
    let mut sorted: Vec<_> = map.iter().collect();
    sorted.sort_unstable_by_key(|a| a.0);

    for (key, value) in sorted.iter() {
        println!("{0}={1}", key, value);
    }
}
