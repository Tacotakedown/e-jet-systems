use super::{
    actuator::Actuator,
    pressure_plate::{self, PressurePlate},
    stator::Stator,
};

#[derive(Debug, Clone)]
pub struct BrakeAssembly {
    pressure_plates: Vec<PressurePlate>,
    stators: Vec<Stator>,
    actuator: Vec<Actuator>,
    name: String,
}

impl BrakeAssembly {
    pub fn new(name: String) -> Self {
        Self {
            pressure_plates: Vec::new(),
            stators: Vec::new(),
            actuator: Vec::new(),
            name,
        }
    }
    pub fn with_pressure_plate(&mut self, pressure_plate: PressurePlate) {
        self.pressure_plates.push(pressure_plate);
    }
    pub fn with_stator(&mut self, stator: Stator) {
        self.stators.push(stator);
    }
    pub fn with_actuator(&mut self, actuator: Actuator) {
        self.actuator.push(actuator);
    }
}
