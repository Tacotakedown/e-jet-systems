#[derive(Debug)]
pub struct FuelFilter {
    name: String,
    resistance: f64, // percentage 0.0 - 1.0
}
impl FuelFilter {
    pub fn new(name: String, resistance: f64) -> Self {
        Self { name, resistance }
    }
    pub fn set_resistance(&mut self, resistance: f64) {
        self.resistance = resistance;
    }

    pub fn get_resistance(&mut self) -> f64 {
        self.resistance
    }
}
