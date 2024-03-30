use core::fmt;

pub struct Station {
    pub min: f64,
    pub max: f64,
    pub sum: f64,
    pub count: u64,
}

impl Default for Station {
    fn default() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            sum: 0.0,
            count: 0,
        }
    }
}

impl Station {
    #[inline]
    pub fn new(value: f64) -> Self {
        Station {
            min: value,
            max: value,
            sum: value,
            count: 1,
        }
    }

    #[inline]
    pub fn update(&mut self, value: f64) {
        self.min = self.min.min(value);
        self.max = self.max.max(value);
        self.count += 1;
        self.sum += value;
    }

    #[inline]
    pub fn merge(&mut self, other: Station) {
        self.min = self.min.min(other.min);
        self.max = self.max.max(other.max);
        self.count += other.count;
        self.sum += other.sum;
    }
}

impl fmt::Display for Station {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let min_str = format!("{:.1}", self.min);
        let max_str = format!("{:.1}", self.max);
        let avg_str = if self.count == 0 { "0.0".to_string() } else { format!("{:.1}", self.sum / self.count as f64) };
        write!(f, "{}/{}/{}", min_str, max_str, avg_str)
    }
}
