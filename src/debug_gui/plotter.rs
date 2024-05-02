#[derive(Debug)]
pub struct PressureData {
    values: Vec<f64>,
    values_post_manifold: Vec<f64>,
    max_ticks: usize,
}

impl PressureData {
    pub fn new(max_ticks: usize) -> Self {
        Self {
            values: Vec::new(),
            values_post_manifold: Vec::new(),
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

    pub fn add_value2(&mut self, value: f64) {
        self.values_post_manifold.push(value);
        if self.values_post_manifold.len() > self.max_ticks {
            self.values_post_manifold.remove(0);
        }
    }

    pub fn get_values_post_manifold(&self) -> Vec<f64> {
        self.values_post_manifold.clone()
    }
}
