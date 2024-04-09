use super::current_type::{self, CurrentType};

#[derive(Debug)]
pub struct Component {
    pub name: String,
    var: String,
    required_voltage: f64,  // volts
    power_consumption: f64, // watts
    current_type: CurrentType,
    pub switch: Option<(String, bool)>,
}

impl Component {
    pub fn new(
        name: String,
        var: String,
        required_voltage: f64,
        power_consumption: f64,
        current_type: CurrentType,
    ) -> Self {
        Self {
            name,
            var,
            required_voltage,
            power_consumption,
            current_type,
            switch: None,
        }
    }

    pub fn calculate_amps(&self, voltage: f64) -> f64 {
        self.power_consumption / voltage
    }

    pub fn is_component_on(&self, voltage: f64) -> bool {
        if let Some((_, switch_value)) = self.switch {
            return switch_value && (self.required_voltage < voltage);
        }
        false
    }
}
