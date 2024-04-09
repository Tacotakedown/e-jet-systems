use self::{
    battery::Battery, bus::Bus, busses::Busses, components::Component, current_type::CurrentType,
    generator::Generator,
};

pub mod battery;
pub mod bus;
pub mod busses;
pub mod components;
pub mod consumption;
pub mod current_type;
pub mod generator;
pub mod transformer;

#[derive(Debug)]
pub struct ElectricalSystem {
    busses: Vec<Bus>,
    batteries: Vec<Battery>,
    generators: Vec<Generator>,
}

impl ElectricalSystem {
    pub fn new() -> Self {
        Self {
            busses: Vec::new(),
            batteries: Vec::new(),
            generators: Vec::new(),
        }
    }
    pub fn with_bus(
        mut self,
        name: Busses,
        current_type: CurrentType,
        voltage: f64,
        var: String,
    ) -> Self {
        let bus = Bus::new(name.to_string(), var, current_type, voltage);
        self.busses.push(bus);
        self
    }

    pub fn with_component(
        mut self,
        name: String,
        var: String,
        power_consumption: f64,
        required_voltage: f64,
    ) -> Self {
        if let Some(parent_bus) = self.busses.last_mut() {
            let component = Component::new(
                name,
                var,
                required_voltage,
                power_consumption,
                parent_bus.get_current_type(),
            );
            parent_bus.add_components(component);
        }
        self
    }

    pub fn with_component_switch(
        mut self,
        component_name: String,
        switch_name: String,
        switch_state: bool,
    ) -> Self {
        if let Some(last_bus) = self.busses.last_mut() {
            if let Some(component) = last_bus
                .components
                .iter_mut()
                .find(|c| c.name == component_name)
            {
                component.switch = Some((switch_name, switch_state));
            }
        }
        self
    }

    pub fn with_battery(
        mut self,
        name: String,
        voltage: f64,
        capacity: f64,
        connected_busses: Vec<Busses>,
    ) -> Self {
        let battery = Battery::new(name, voltage, capacity, connected_busses);
        self.batteries.push(battery);
        self
    }

    pub fn with_generator(
        mut self,
        name: String,
        voltage: f64,
        max_output: f64,
        connected_busses: Vec<Busses>,
    ) -> Self {
        let generator = Generator::new(name, voltage, max_output, connected_busses);
        self.generators.push(generator);
        self
    }

    pub fn build(self) -> Self {
        self
    }

    pub fn get_system_results(&self) {
        // need to create a type and return required voltages:f64 and electronics state:boolean for every component in the system on call
    }
}
