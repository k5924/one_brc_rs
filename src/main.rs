mod station;
mod file_processor;

use std::fs::File;
use std::io::{self};
use std::collections::HashMap;
use std::time::Instant;
use station::Station;
use file_processor::read_file_in_chunks;

fn process_line(map: &mut HashMap<Box<str>, Station>, buffer: &[u8]) {
    let line = String::from_utf8_lossy(&buffer);
    let unpacked_line = &line.trim();
    let (key, val) = unpacked_line.split_once(';').expect("failed to split string");
    let converted = val.parse::<f64>().expect("failed to parse float");
    map.entry(key.into()).or_default().update(converted);
}

fn output_result(map: &mut HashMap<Box<str>, Station>) {
    let mut sorted: Vec<_> = map.iter().collect();
    sorted.sort_unstable_by_key(|a| a.0);

    for (key, value) in sorted.iter() {
        println!("{0}={1}/{2}/{3}", key, value.min, value.max, value.sum / value.count as f64);
    }
}

fn main() -> io::Result<()> {
    let filename = "measurements.txt";
    let file = File::open(filename)?;
    let now = Instant::now();

    read_file_in_chunks(&file)?;

    let elapsed_time = now.elapsed();
    println!("Running the program took {} milliseconds", elapsed_time.as_millis());

    Ok(())
}
