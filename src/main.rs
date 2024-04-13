mod station;
mod utils;
mod file_processor;
mod float_parser;
mod line_processor;
mod result_outputter;
mod chunk_processor;
mod config_loader;
mod map_processor;
mod running_mode;

use hashbrown::HashMap;
use result_outputter::output_result;
use config_loader::{load_config, get_mode};
use crate::running_mode::RunningMode;
use log4rs;
use log::info;

use std::fs::File;
use std::io::{self};

#[cfg(not(feature = "rayon"))]
use std::io::{ErrorKind, Error};
use std::time::Instant;
use station::Station;
use file_processor::{process_file_single_thread, process_file_multiple_threads};

#[cfg(feature = "rayon")]
use file_processor::process_file_rayon;

const FILENAME: &str = "measurements.txt";

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn execute_program(mode: &RunningMode, file: &File, map: &mut HashMap<String, Station>) -> io::Result<()> {
    return match mode {
        RunningMode::SingleThreaded => Ok(process_file_single_thread(&file, map)?),
        RunningMode::MultiThreaded => Ok(process_file_multiple_threads(&file, map)?),
        RunningMode::Rayon => {
            #[cfg(not(feature = "rayon"))]
            {
                return Err(Error::new(ErrorKind::Other, "Rayon feature is not enabled"));
            }
            #[cfg(feature = "rayon")]
            {
                Ok(process_file_rayon(&file, map)?)
            }
        }
    }
}

fn main() -> io::Result<()> {
    log4rs::init_file("config/log/log4rs.yaml", Default::default()).expect("unable to load log config");

    let config = load_config().expect("unable to load config");
    let mode = get_mode(&config).expect("unable to find env variable");

    let now = Instant::now();

    let file = File::open(FILENAME)?;
    let mut map = HashMap::new();

    execute_program(&mode, &file, &mut map)?;

    output_result(&mut map);

    let elapsed_time = now.elapsed();
    info!("Running the program took {} milliseconds in {} mode", elapsed_time.as_millis(), mode);

    Ok(())
}
