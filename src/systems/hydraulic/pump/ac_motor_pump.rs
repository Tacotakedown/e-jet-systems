#[derive(Debug)]
pub struct AcMotorPump {
    rated_output_flow_l_min: f64,
    powered: bool,
    pressure_override: bool,
}
impl AcMotorPump {
    pub fn new() -> Self {
        Self {
            rated_output_flow_l_min: 71.92, // L/min // 19 gal/min
            powered: false,
            pressure_override: false,
        }
    }
    pub fn set_power_state(&mut self, state: bool) {
        self.powered = state
    }

    pub fn get_output(&self, dt: f64, pressure: f64) -> f64 {
        if self.powered {
            if !self.pressure_override {
                const RATED_PRESSURE: f64 = 2700.0;
                let pressure_factor = calculate_pressure_factor(pressure, RATED_PRESSURE);
                let volume_flow_rate = self.rated_output_flow_l_min * pressure_factor;
                let volume_flow_rate_m3_s = volume_flow_rate * 0.001;
                volume_flow_rate_m3_s * dt
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    pub fn set_pressure_override(&mut self, state: bool) {
        self.pressure_override = state
    }
}
fn calculate_pressure_factor(pressure: f64, rated_pressure: f64) -> f64 {
    // This is an example function that increases the factor non-linearly as pressure decreases
    // You can modify this function to suit your specific requirements
    let pressure_ratio = pressure / rated_pressure;
    let factor = 1.0 + (1.0 - pressure_ratio).powf(2.0);
    factor
}
