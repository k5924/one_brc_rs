use config::Config;
use std::io;

pub fn load_config() -> io::Result<Config> {
    let mut config_builder = Config::builder();
    config_builder = config_builder.add_source(config::File::with_name("config"));
    config_builder.build().map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

pub fn get_enable_multithreading(config: &Config) -> io::Result<bool> {
    config.get_bool("myapp.enable_multithreading").map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}
