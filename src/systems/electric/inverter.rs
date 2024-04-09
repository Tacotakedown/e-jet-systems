#[derive(Debug)]
pub struct Inverter {
    name: String,
    input_voltage: f64,
    output_voltage: f64,
    efficency: f64,
    max_power_output: f64,
}

impl Inverter {
    pub fn new(
        name: String,
        input_voltage: f64,
        output_voltage: f64,
        efficency: f64,
        max_power_output: f64,
    ) -> Self {
        Self {
            name,
            input_voltage,
            output_voltage,
            efficency,
            max_power_output,
        }
    }
}
