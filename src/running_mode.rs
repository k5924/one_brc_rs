use std::fmt;

#[derive(Debug, PartialEq)]
pub enum RunningMode {
    SingleThreaded,
    MultiThreaded,
    Rayon,
}

impl fmt::Display for RunningMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RunningMode::SingleThreaded => write!(f, "SingleThreaded"),
            RunningMode::MultiThreaded => write!(f, "MultiThreaded"),
            RunningMode::Rayon => write!(f, "Rayon"),
        }
    }
}

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
