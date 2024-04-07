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

#[cfg(test)]
mod tests {
    use super::*;
    use config::{Config, File, FileFormat};

    #[test]
    fn test_get_mode_valid_modes() {
        // Create a test configuration
        let config = Config::builder()
            .add_source(File::from_str("myapp.mode = 'single_thread'", FileFormat::Toml))
            .build()
            .expect("Failed to build config");

        // Test single_thread mode
        assert_eq!(get_mode(&config).unwrap(), RunningMode::SingleThreaded);

        // Test multi_thread mode
        let config = Config::builder()
            .add_source(File::from_str("myapp.mode = 'multi_thread'", FileFormat::Toml))
            .build()
            .expect("Failed to build config");
        assert_eq!(get_mode(&config).unwrap(), RunningMode::MultiThreaded);

        // Test rayon mode
        let config = Config::builder()
            .add_source(File::from_str("myapp.mode = 'rayon'", FileFormat::Toml))
            .build()
            .expect("Failed to build config");
        assert_eq!(get_mode(&config).unwrap(), RunningMode::Rayon);
    }

    #[test]
    fn test_get_mode_invalid_mode() {
        // Create a test configuration with invalid mode
        let config = Config::builder()
            .add_source(File::from_str("myapp.mode = 'invalid_mode'", FileFormat::Toml))
            .build()
            .expect("Failed to build config");

        // Test invalid mode
        assert!(get_mode(&config).is_err());
    }

    #[test]
    fn test_get_mode_missing_mode() {
        // Create an empty configuration
        let config = Config::builder().build().expect("Failed to build config");

        // Test missing mode
        assert!(get_mode(&config).is_err());
    }
}

