#[derive(Debug)]
pub struct Reservoir {
    low_pressure_chamber: LowPressureChamber,
    bootstrap_cylinder: BootstrapCylinder,
    indicators: Indicators,
}
#[derive(Debug)]
struct LowPressureChamber {
    max_storage_volume_l: f64,
    current_fluid_level_l: f64,
    chamber_pressure_kpa: f64,
}

#[derive(Debug)]
struct BootstrapCylinder {
    system_pressure_kpa: f64,
}

#[derive(Debug)]
struct Indicators {
    mechenical_level_indicator: f64,
    electrical_level_indicator: f64,
    temperature_transducer: f64,
}

impl Reservoir {
    pub fn new() -> Self {
        Self {
            low_pressure_chamber: LowPressureChamber {
                max_storage_volume_l: 12.3,
                current_fluid_level_l: 0.,
                chamber_pressure_kpa: 0.,
            },
            bootstrap_cylinder: BootstrapCylinder {
                system_pressure_kpa: 0.,
            },
            indicators: Indicators {
                mechenical_level_indicator: 0.,
                electrical_level_indicator: 0.,
                temperature_transducer: 0.,
            },
        }
    }
    pub fn set_system_pressure(&mut self, pressure_kpa: f64) {
        self.bootstrap_cylinder.system_pressure_kpa = pressure_kpa
    }

    pub fn set_chamber_pressure(&mut self, pressure_kpa: f64) {
        self.low_pressure_chamber.chamber_pressure_kpa = pressure_kpa
    }

    pub fn set_fluid_level(&mut self, fluid_level_l: f64) {
        self.low_pressure_chamber.current_fluid_level_l = fluid_level_l
    }

    pub fn decrease_fluid_level(&mut self, decrease_by_l: f64) {
        self.low_pressure_chamber.current_fluid_level_l -= decrease_by_l
    }

    pub fn increase_fluid_level(&mut self, increase_by_l: f64) {
        self.low_pressure_chamber.current_fluid_level_l += increase_by_l
    }
    pub fn get_level(&self) -> f64 {
        self.low_pressure_chamber.current_fluid_level_l
    }
}
