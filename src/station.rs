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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_station_new() {
        let station = Station::new(10.0);
        assert_eq!(station.min, 10.0);
        assert_eq!(station.max, 10.0);
        assert_eq!(station.sum, 10.0);
        assert_eq!(station.count, 1);
    }

    #[test]
    fn test_station_update() {
        let mut station = Station::new(10.0);
        station.update(20.0);
        assert_eq!(station.min, 10.0);
        assert_eq!(station.max, 20.0);
        assert_eq!(station.sum, 30.0);
        assert_eq!(station.count, 2);
    }

    #[test]
    fn test_station_merge() {
        let mut station1 = Station::new(10.0);
        let station2 = Station::new(20.0);
        station1.merge(station2);
        assert_eq!(station1.min, 10.0);
        assert_eq!(station1.max, 20.0);
        assert_eq!(station1.sum, 30.0);
        assert_eq!(station1.count, 2);
    }

    #[test]
    fn test_station_display() {
        let station = Station {
            min: 10.0,
            max: 20.0,
            sum: 30.0,
            count: 2,
        };
        assert_eq!(format!("{}", station), "10.0/20.0/15.0");
    }
}
