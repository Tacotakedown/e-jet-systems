#[derive(Debug)]
pub struct PressureData {
    values: Vec<f64>,
    max_ticks: usize,
}

impl PressureData {
    pub fn new(max_ticks: usize) -> Self {
        Self {
            values: Vec::new(),
            max_ticks,
        }
    }

    pub fn add_value(&mut self, value: f64) {
        self.values.push(value);
        if self.values.len() > self.max_ticks {
            self.values.remove(0);
        }
    }

    pub fn get_values(&self) -> Vec<f64> {
        self.values.clone()
    }
}
