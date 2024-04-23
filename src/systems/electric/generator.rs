use super::busses::Busses;

#[derive(Debug, Clone)]
pub struct Generator {
    name: String,
    voltage: f64,    // Volts
    max_output: f64, // Watts
    connected_busses: Vec<Busses>,
}

impl Generator {
    pub fn new(name: String, voltage: f64, max_output: f64, connected_busses: Vec<Busses>) -> Self {
        Self {
            name,
            voltage,
            max_output,
            connected_busses,
        }
    }

    // need more info on voltage regulator, this will determine output based on RPM
}
