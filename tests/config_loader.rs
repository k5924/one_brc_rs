use one_brc_rs::running_mode::RunningMode;
use one_brc_rs::config_loader::get_mode;

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

