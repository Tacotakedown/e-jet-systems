// Power Control Unit
use core::fmt;

#[derive(Debug)]
pub enum AILERON_PCU_TYPES {
    LhOb,
    LhIb,
    RhOb,
    RhIb,
}

impl fmt::Display for AILERON_PCU_TYPES {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AILERON_PCU_TYPES::LhIb => write!(f, "LH_I/B"),
            AILERON_PCU_TYPES::LhOb => write!(f, "LH_O/B"),
            AILERON_PCU_TYPES::RhIb => write!(f, "RH_I/B"),
            AILERON_PCU_TYPES::RhOb => write!(f, "RH_O/B"),
        }
    }
}

#[derive(Debug)]
pub struct PCU {
    hydraulic_input_pressure: f64,
    piston_diameter: f64,
    piston_stroke: f64,
    id: String,
    position: AILERON_PCU_TYPES,
}

impl PCU {
    pub fn new(
        hydraulic_input_pressure: f64,
        piston_diameter: f64,
        piston_stroke: f64,
        id: String,
        position: AILERON_PCU_TYPES,
    ) -> Self {
        Self {
            hydraulic_input_pressure,
            piston_diameter,
            piston_stroke,
            id,
            position,
        }
    }

    pub fn get_id(self) -> String {
        self.id
    }

    pub fn set_hydraulic_input_pressure(&mut self, pressure: f64) {
        self.hydraulic_input_pressure = pressure
    }
}
