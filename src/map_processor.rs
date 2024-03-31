use hashbrown::HashMap;
use crate::Station;

pub fn update_or_create(map: &mut HashMap<String, Station>, key: &str, value: f64) {
    match map.raw_entry_mut().from_key(key) {
        hashbrown::hash_map::RawEntryMut::Occupied(mut occupied) => {
            occupied.get_mut().update(value);
        }
        hashbrown::hash_map::RawEntryMut::Vacant(vacant) => {
            vacant.insert(key.to_string(), Station::new(value));
        }
    }
}

pub fn merge_or_create(map: &mut HashMap<String, Station>, key: &str, value: Station) {
    match map.raw_entry_mut().from_key(key) {
        hashbrown::hash_map::RawEntryMut::Occupied(mut occupied) => {
            occupied.get_mut().merge(value);
        }
        hashbrown::hash_map::RawEntryMut::Vacant(vacant) => {
            vacant.insert(key.to_string(), value);
        }
    }
}
