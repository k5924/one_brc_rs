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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_or_create_existing_key() {
        let mut map = HashMap::new();
        map.insert(String::from("key1"), Station::new(10.0));

        update_or_create(&mut map, "key1", 20.0);

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
    fn test_update_or_create_new_key() {
        let mut map = HashMap::new();

        update_or_create(&mut map, "key2", 30.0);

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
    fn test_merge_or_create_existing_key() {
        let mut map = HashMap::new();
        map.insert(
            String::from("key1"),
            Station {
                min: 10.0,
                max: 20.0,
                sum: 30.0,
                count: 2,
            },
        );

        merge_or_create(
            &mut map,
            "key1",
            Station {
                min: 5.0,
                max: 25.0,
                sum: 30.0,
                count: 3,
            },
        );

        assert_eq!(
            map.get("key1"),
            Some(&Station {
                min: 5.0,
                max: 25.0,
                sum: 60.0,
                count: 5,
            })
        );
    }

    #[test]
    fn test_merge_or_create_new_key() {
        let mut map = HashMap::new();

        merge_or_create(
            &mut map,
            "key2",
            Station {
                min: 5.0,
                max: 25.0,
                sum: 30.0,
                count: 3,
            },
        );

        assert_eq!(
            map.get("key2"),
            Some(&Station {
                min: 5.0,
                max: 25.0,
                sum: 30.0,
                count: 3,
            })
        );
    }
}
