#[derive(Debug)]
pub struct Resistor {
    resistance: f64,
    input: f64,
    output: f64,
}

impl Resistor {
    pub fn new(resistance: f64, input: f64, output: f64) -> Self {
        Self {
            resistance,
            input,
            output,
        }
    }

    pub fn set_input(&mut self, input: f64) {
        self.input = input
    }
    pub fn calculate_output(&mut self) {
        self.output = self.input * (self.resistance / (self.resistance + self.input))
    }
    pub fn get_output(&self) -> f64 {
        self.output
    }
}
