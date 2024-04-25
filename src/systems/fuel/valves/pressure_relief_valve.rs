#[derive(Debug)]
pub enum PressureReliefValveTypes {
    FuelTankReliefValves,
}
impl PressureReliefValveTypes {
    pub fn get_max_pressure(&self) -> f64 {
        match self {
            PressureReliefValveTypes::FuelTankReliefValves => 3.0,
        }
    }
}

#[derive(Debug)]
pub struct PressureReliefValve {
    id: String,
    max_pressure_dif: f64, // in PSI
}

impl PressureReliefValve {
    pub fn new(id: String, location: PressureReliefValveTypes) -> Self {
        Self {
            id,
            max_pressure_dif: location.get_max_pressure(),
        }
    }
}
