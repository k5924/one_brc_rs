mod station;
mod file_processor;
mod float_parser;
mod line_parser;
mod result_outputter;

use std::fs::File;
use std::io::{self};
use hashbrown::HashMap;
use std::time::Instant;
use station::Station;
use file_processor::read_file_in_chunks;
use result_outputter::output_result;

fn main() -> io::Result<()> {
    let filename = "measurements.txt";
    let file = File::open(filename)?;
    let now = Instant::now();
    let mut entries: HashMap<String, Station> = HashMap::new();

    read_file_in_chunks(&file, &mut entries)?;

    output_result(&mut entries);

    let elapsed_time = now.elapsed();
    println!("Running the program took {} milliseconds", elapsed_time.as_millis());

    Ok(())
}
