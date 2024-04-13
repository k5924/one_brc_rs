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

