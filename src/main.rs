use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::time::Instant;

struct Station {
    minimum: f64,
    maximum: f64,
    sum: f64,
    count: f64,
}

fn main() -> std::io::Result<()> {
    let filename = "measurements.txt";
    let file = File::open(filename).expect("error opening {filename}");
    let mut reader = BufReader::new(file);
    let mut string = String::new();
    let mut entries: HashMap<String, Station> = HashMap::new();

    let now = Instant::now();

    while reader.read_line(&mut string).unwrap() > 0 {
        let unpacked_line = &string.trim();
        let split = unpacked_line.split(";").to_owned();
        let collection: Vec<&str> = split.collect::<Vec<&str>>();
        let key = collection[0].to_owned();
        let val = collection[1].to_owned();
        let converted = val.parse::<f64>().expect("unable to convert string to float");
        let value_from_map = entries.get(&key);
        if value_from_map.is_none() {
            let station = Station{minimum: converted, maximum: converted, sum: converted, count: 1.0};
            entries.insert(key, station);
        } else {
            let old_station_val = value_from_map.expect("unable to get value from map");
            let mut minimum = old_station_val.minimum;
            let mut maximum = old_station_val.maximum;
            if converted < minimum {
                minimum = converted;
            }
            if converted > maximum {
                maximum = converted;
            }
            let sum = old_station_val.sum + converted;
            let count = old_station_val.count + 1.0;
            let new_station = Station{minimum, maximum, sum, count};
            entries.insert(key, new_station);
        }
        string.clear();
    }

    let mut sorted: Vec<_> = entries.iter().collect();
    sorted.sort_by_key(|a| a.0);

    for (key, value) in sorted.iter() {
        println!("{0}={1}/{2}/{3}", key, value.minimum, value.maximum, value.sum / value.count);
    }

    let elapsed_time = now.elapsed();
    println!("Running the program took {} milliseconds", elapsed_time.as_millis());

    Ok(())
}
