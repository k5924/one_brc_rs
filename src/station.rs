use core::fmt;

#[derive(Debug, PartialEq)]
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
        write!(f, 
        "{:.1}/{:.1}/{:.1}", 
        self.min, 
        self.max, 
        if self.count == 0 {
        0.0
        } else {
        self.sum / self.count as f64
        })
    }
}

