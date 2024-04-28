#[derive(Debug)]
pub struct AcMotorPump {
    rated_output_flow_l_min: f64,
    powered: bool,
}
impl AcMotorPump {
    pub fn new() -> Self {
        Self {
            rated_output_flow_l_min: 71.92, // L/min // 19 gal/min
            powered: false,
        }
    }
    pub fn set_power_state(&mut self, state: bool) {
        self.powered = state
    }

    pub fn get_output(&self, dt: f64) -> f64 {
        if self.powered {
            let volume_flow_rate_m3_s = self.rated_output_flow_l_min * 0.001;
            volume_flow_rate_m3_s * dt
        } else {
            0.0
        }
    }
}
