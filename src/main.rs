use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::time::Instant;

struct Station {
    min: f64,
    max: f64,
    sum: f64,
    count: u64,
}

impl Default for Station {
    fn default() -> Self {
        Self {
            min: f64::MAX,
            max: f64::MIN,
            sum: 0.0,
            count: 0,
        }
    }
}

impl Station {
    fn update(&mut self, value: f64) {
        self.min = self.min.min(value);
        self.max = self.max.max(value);
        self.count += 1;
        self.sum += value;
    }
}

fn main() -> std::io::Result<()> {
    let filename = "measurements.txt";
    let file = File::open(filename).expect("error opening {filename}");
    let mut reader = BufReader::new(file);
    let mut string = String::new();
    let mut entries: HashMap<Box<str>, Station> = HashMap::new();

    let now = Instant::now();

    while reader.read_line(&mut string).expect("failed to get string") > 0 {
        let unpacked_line = &string.trim();
        let (key, val) = unpacked_line.split_once(";").expect("failed to split on line");
        let converted = val.parse::<f64>().expect("unable to convert string to float");
        entries.entry(key.into()).or_default().update(converted);
        string.clear();
    }

    let mut sorted: Vec<_> = entries.iter().collect();
    sorted.sort_by_key(|a| a.0);

    for (key, value) in sorted.iter() {
        println!("{0}={1}/{2}/{3}", key, value.min, value.max, value.sum / value.count as f64);
    }

    let elapsed_time = now.elapsed();
    println!("Running the program took {} milliseconds", elapsed_time.as_millis());

    Ok(())
}
