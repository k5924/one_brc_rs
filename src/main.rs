mod station;
mod utils;
mod file_processor;
mod float_parser;
mod line_processor;
mod result_outputter;
mod chunk_processor;

use std::fs::File;
use std::io::{self};
use std::time::Instant;
use station::Station;
use file_processor::process_file;
use hashbrown::HashMap;
use result_outputter::output_result;

fn main() -> io::Result<()> {
    let filename = "measurements.txt";
    let file = File::open(filename)?;
    let mut map = HashMap::new();
    let now = Instant::now();

    process_file(&file, &mut map)?;

    output_result(&mut map);

    let elapsed_time = now.elapsed();
    println!("Running the program took {} milliseconds", elapsed_time.as_millis());

    Ok(())
}
