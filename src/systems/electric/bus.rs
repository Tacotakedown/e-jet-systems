use super::{components::Component, current_type::CurrentType};

#[derive(Debug)]
pub struct Bus {
    name: String,
    components: Vec<Component>,
    var: String,
    current_type: CurrentType,
    voltage: f64,
}

impl Bus {
    pub fn new(name: String, var: String, current_type: CurrentType, voltage: f64) -> Self {
        Self {
            name,
            components: Vec::new(),
            var,
            current_type,
            voltage,
        }
    }

    pub fn add_components(&mut self, component: Component) {
        self.components.push(component);
    }

    pub fn set_voltage(&mut self, voltage: f64) {
        self.voltage = voltage
    }

    pub fn get_voltage(&self) -> f64 {
        self.voltage
    }
    pub fn get_current_type(&self) -> CurrentType {
        self.current_type
    }
}
