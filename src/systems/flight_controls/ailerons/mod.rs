use self::aileron_position::AileronPosition;

pub mod aileron_deflection;
pub mod aileron_position;
pub mod aileron_surface_position_sensor_lvdt;

#[derive(Debug)]
pub struct Ailerons {
    pcu_input: f64, // 0-1
    position: AileronPosition,
}

impl Ailerons {
    pub fn new(pcu_input: f64, position: AileronPosition) -> Self {
        Self {
            pcu_input,
            position,
        }
    }
    pub fn get_pcu_input(self) -> f64 {
        self.pcu_input
    }
}
