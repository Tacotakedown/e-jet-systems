#[derive(Debug)]
pub struct AcMotorPump {
    rated_output_flow_l_min: f64,
    powered: bool,
}
impl AcMotorPump {
    pub fn new(rated_output_flow_l_min: f64) -> Self {
        Self {
            rated_output_flow_l_min,
            powered: false,
        }
    }
    pub fn set_power_state(&mut self, state: bool) {
        self.powered = state
    }

    pub fn get_output(&self) -> f64 {
        if self.powered {
            self.rated_output_flow_l_min
        } else {
            0.0
        }
    }
}
