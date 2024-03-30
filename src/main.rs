mod station;
mod utils;
mod file_processor;
mod float_parser;
mod line_processor;
mod result_outputter;
mod chunk_processor;
mod config_loader;

use std::fs::File;
use std::io::{self};
use std::time::Instant;
use station::Station;
use file_processor::{process_file_single_thread, process_file_multiple_threads};
use hashbrown::HashMap;
use result_outputter::output_result;
use config_loader::{load_config, get_enable_multithreading};

const FILENAME: &str = "measurements.txt";

fn main() -> io::Result<()> {
    let config = load_config().expect("unable to load config");
    let enable_multithreading = get_enable_multithreading(&config).expect("unable to find env variable");

    let now = Instant::now();

    let file = File::open(FILENAME)?;
    let mut map = HashMap::new();

    if enable_multithreading {
        process_file_multiple_threads(&file, &mut map)?;
    } else {
        process_file_single_thread(&file, &mut map)?;
    }

    output_result(&mut map);

    let elapsed_time = now.elapsed();
    println!("Running the program took {} milliseconds", elapsed_time.as_millis());

    Ok(())
}
