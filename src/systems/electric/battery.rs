use super::busses::Busses;

#[derive(Debug)]
pub struct Battery {
    name: String,
    voltage: f64,  // Volts
    capacity: f64, // Ampere-hours
    connected_busses: Vec<Busses>,
}
impl Battery {
    pub fn new(name: String, voltage: f64, capacity: f64, connected_busses: Vec<Busses>) -> Self {
        Self {
            name,
            voltage,
            capacity,
            connected_busses,
        }
    }
}
