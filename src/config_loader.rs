use std::io;
use config::{Config, File as ConfigFile};
use crate::running_mode::RunningMode;

pub fn load_config() -> io::Result<Config> {
    let mut config_builder = Config::builder();
    config_builder = config_builder.add_source(ConfigFile::with_name("config/app/config.toml"));
    config_builder.build().map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

pub fn get_mode(config: &Config) -> io::Result<RunningMode> {
    let mode_str = match config.get_string("myapp.mode") {
        Ok(mode) => mode,
        Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
    };

    let mode_enum = match mode_str.as_str() {
        "single_thread" => RunningMode::SingleThreaded,
        "multi_thread" => RunningMode::MultiThreaded,
        "rayon" => RunningMode::Rayon,
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid running mode")),
    };

    Ok(mode_enum)
}

