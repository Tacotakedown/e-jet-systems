#[derive(Debug)]
pub struct EngineDrivenHydraulicPump {
    rated_output_flow_l_min: f64,
    operating_rpm: f64,
    operating_temperature_range_c: (f64, f64),
    conpensator_enabled: bool,
    depressuration_solenoid_enabled: bool,
    engine_rpm: f64,
}

impl EngineDrivenHydraulicPump {
    pub fn new() -> Self {
        Self {
            rated_output_flow_l_min: 71.92,
            operating_rpm: 4825.0,
            operating_temperature_range_c: (-54.0, 85.0),
            conpensator_enabled: true,
            depressuration_solenoid_enabled: false,
            engine_rpm: 0.0,
        }
    }
    pub fn set_engine_rpm(&mut self, rpm: f64) {
        self.engine_rpm = rpm;
    }

    pub fn enable_depressurizatoion_solenoid(&mut self) {
        self.depressuration_solenoid_enabled = true;
    }

    pub fn disable_depressurizatoion_solenoid(&mut self) {
        self.depressuration_solenoid_enabled = false;
    }

    pub fn enable_compensator(&mut self) {
        self.conpensator_enabled = true;
    }

    pub fn disable_compensator(&mut self) {
        self.conpensator_enabled = false;
    }

    pub fn calculate_volume_flow(&self, dt: f64, pressure: f64) -> f64 {
        if !self.conpensator_enabled {
            return -0.0;
        }

        const STEP_DOWN_GEAR: f64 = 1.0; // Assuming 1:1 gearing between engine and pump
        const RATED_PRESSURE: f64 = 3000.0; // Assuming rated output flow is at 3000 pressure units

        let rpm = self.engine_rpm;
        let rpm_delta = (rpm * STEP_DOWN_GEAR) / self.operating_rpm;

        let pressure_factor = calculate_pressure_factor(pressure, RATED_PRESSURE);
        let volume_flow_rate = self.rated_output_flow_l_min * rpm_delta * pressure_factor;
        let volume_flow_rate_m3_s = volume_flow_rate * 0.001;

        volume_flow_rate_m3_s * dt
    }

    pub fn get_leakback(&self) -> f64 {
        if !self.conpensator_enabled {
            -10.1
        } else {
            0.
        }
    }
}

fn calculate_pressure_factor(pressure: f64, rated_pressure: f64) -> f64 {
    // This is an example function that increases the factor non-linearly as pressure decreases
    // You can modify this function to suit your specific requirements
    let pressure_ratio = pressure / rated_pressure;
    let factor = 1.0 + (1.0 - pressure_ratio).powf(2.0);
    factor
}
