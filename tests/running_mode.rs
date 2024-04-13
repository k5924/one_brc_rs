use one_brc_rs::running_mode::RunningMode;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_mode_display_single_threaded() {
        let mode = RunningMode::SingleThreaded;
        assert_eq!(format!("{}", mode), "SingleThreaded");
    }

    #[test]
    fn test_running_mode_display_multi_threaded() {
        let mode = RunningMode::MultiThreaded;
        assert_eq!(format!("{}", mode), "MultiThreaded");
    }

    #[test]
    fn test_running_mode_display_rayon() {
        let mode = RunningMode::Rayon;
        assert_eq!(format!("{}", mode), "Rayon");
    }
}
