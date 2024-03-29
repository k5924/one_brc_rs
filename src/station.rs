pub struct Station {
    pub min: f64,
    pub max: f64,
    pub sum: f64,
    pub count: u64,
}

impl Default for Station {
    fn default() -> Self {
        Self {
            min: f64::MAX,
            max: f64::MIN,
            sum: 0.0,
            count: 0,
        }
    }
}

impl Station {
    pub fn update(&mut self, value: f64) {
        self.min = self.min.min(value);
        self.max = self.max.max(value);
        self.count += 1;
        self.sum += value;
    }
}
