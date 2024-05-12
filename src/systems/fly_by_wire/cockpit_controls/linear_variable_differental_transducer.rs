#[derive(Debug)]
pub enum LVDTControllers {
    ElevatorAxis,
    RudderPedals,
    SpoilerHandle,
} // aileron input is handled with actual cables, thus we omit it

#[derive(Debug)]
pub struct LinearVariableDifferentalTransducer {
    mechanical_position: f64,
    input_volts: f64,
    output_volts: f64,
    sensitivity: f64, // volts per unit moved
    offset: f64, // starting voltage offset (should be 0 for the most part, but ima implement this for debugging easily)
}

impl LinearVariableDifferentalTransducer {
    pub fn new(sensitivity: f64, offset: f64) -> Self {
        Self {
            mechanical_position: 0.0,
            input_volts: 0.0,
            output_volts: 0.0,
            sensitivity,
            offset,
        }
    }

    pub fn update_position(&mut self, position: f64) {
        self.mechanical_position = position;
        self.output_volts =
            self.sensitivity * self.mechanical_position + self.offset + self.input_volts;
    }

    pub fn set_input_volts(&mut self, input_volts: f64) {
        self.input_volts = input_volts;
    }

    pub fn get_output_volts(&self) -> f64 {
        self.output_volts
    }
}
