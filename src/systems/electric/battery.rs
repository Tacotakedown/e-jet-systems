use super::busses::Busses;

#[derive(Debug, Clone)]
pub struct Battery {
    name: String,
    voltage: f64,
    voltage_min_max: (f64, f64), // Volts
    capacity: f64,               // Ampere-hours
    connected_busses: Vec<Busses>,
    state_of_charge: f64,                  // percent (0.0 - 1.0)
    charge_discharge_rate: f64, // Amps (negative to positive self explanatory what each do)
    max_charge_discharge_rate: (f64, f64), // Amps
    efficency: f64,             // Efficiency factor (0.0 - 1.0)
    health: f64,                // Health factor (0.0 - 1.0)
}
impl Battery {
    pub fn new(
        name: String,
        voltage: f64,
        voltage_min_max: (f64, f64),
        capacity: f64,
        connected_busses: Vec<Busses>,
        max_charge_discharge_rate: (f64, f64),
    ) -> Self {
        Self {
            name,
            voltage,
            voltage_min_max,
            capacity,
            connected_busses,
            state_of_charge: 1.0,
            charge_discharge_rate: 0.0,
            max_charge_discharge_rate,
            efficency: 1.0,
            health: 1.0,
        }
    }

    pub fn update_state(&mut self, time_delta: f64) {
        let charge_delta = self.charge_discharge_rate * time_delta;
        let state_of_charge_delta = charge_delta / self.capacity;

        self.state_of_charge = (self.state_of_charge + state_of_charge_delta)
            .min(1.0)
            .max(0.0);

        let voltage_range = self.voltage_min_max.1 - self.voltage_min_max.0;
        self.voltage = self.voltage_min_max.0 + self.state_of_charge * voltage_range;
    }

    pub fn set_charge_discharge_rate(&mut self, rate: f64) {
        self.charge_discharge_rate = rate.clamp(
            self.max_charge_discharge_rate.0,
            self.max_charge_discharge_rate.1,
        )
    }

    pub fn get_state_of_charge(&self) -> f64 {
        self.state_of_charge
    }

    pub fn get_efficency(&mut self) -> f64 {
        self.efficency
    }

    pub fn get_health(&mut self) -> f64 {
        self.health
    }
}
