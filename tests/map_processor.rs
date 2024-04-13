use one_brc_rs::station::Station;
use one_brc_rs::map_processor::{update_or_create, merge_or_create};
use hashbrown::HashMap;

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
