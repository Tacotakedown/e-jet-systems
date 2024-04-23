pub enum Units {
    Bool,
    Number,
    Degree,
    Percent,
    Feet,
    Knots,
}

impl Units {
    pub fn get_unit(self) -> String {
        match self {
            Self::Bool => {
                return "bool".to_string();
            }
            Self::Degree => {
                return "degree".to_string();
            }
            Self::Feet => {
                return "feet".to_string();
            }
            Self::Knots => {
                return "knots".to_string();
            }
            Self::Number => {
                return "number".to_string();
            }
            Self::Percent => {
                return "percent".to_string();
            }
        }
    }
}

struct Simvar {
    name: String,
    unit: String,
}

#[derive(Debug)]
pub enum Simvars {
    AcBus1Voltage,
    AcBus2Voltage,
    AcEssBusVoltage,
    AcGndSvcBusVoltage,
    AcStbyBusVoltage,
    DcGndSvcBusVoltage,
    DcBus1Voltage,
    DcBus2Voltage,
    DcEssBus1Voltage,
    DcEssBus2Voltage,
    DcEssBus3Voltage,
    HotBatBus1Voltage,
    HotBatBus2Voltage,
    ApuStartBusVoltage,
}
impl Simvars {
    pub fn name(self) -> String {
        match self {
            Self::AcBus1Voltage => "L:OBJ_SYS_AC_BUS_1_VOLTAGE".to_string(),
            Self::AcBus2Voltage => "L:OBJ_SYS_AC_BUS_2_VOLTAGE".to_string(),
            Self::AcEssBusVoltage => "L:OBJ_SYS_AC_ESSENTIAL_BUS_VOLTAGE".to_string(),
            Self::AcGndSvcBusVoltage => "L:OBJ_SYS_AC_GROUND_SERVICE_BUS_VOLTAGE".to_string(),
            Self::AcStbyBusVoltage => "L:OBJ_SYS_AC_STANDBY_BUS_VOLTAGE".to_string(),
            Self::DcGndSvcBusVoltage => "L:OBJ_SYS_DC_GROUND_SERVICE_BUS_VOLTAGE".to_string(),
            Self::DcBus1Voltage => "L:OBJ_SYS_DC_BUS_1_VOLTAGE".to_string(),
            Self::DcBus2Voltage => "L:OBJ_SYS_DC_BUS_2_VOLTAGE".to_string(),
            Self::DcEssBus1Voltage => "L:OBJ_SYS_DC_ESSENTIAL_BUS_1_VOLTAGE".to_string(),
            Self::DcEssBus2Voltage => "L:OBJ_SYS_DC_ESSENTIAL_BUS_2_VOLTAGE".to_string(),
            Self::DcEssBus3Voltage => "L:OBJ_SYS_DC_ESSENTIAL_BUS_3_VOLTAGE".to_string(),
            Self::HotBatBus1Voltage => "L:OBJ_SYS_HOT_BATTERY_BUS_1_VOLTAGE".to_string(),
            Self::HotBatBus2Voltage => "L:OBJ_SYS_HOT_BATTERY_BUS_2_VOLTAGE".to_string(),
            Self::ApuStartBusVoltage => "L:OBJ_SYS_APU_STARTER_BUS_VOLTAGE".to_string(),
        }
    }
}

pub struct SimVarBuilder {
    simvars: Vec<(Simvars, Units)>,
}

impl SimVarBuilder {
    pub fn simvar_builder() -> Self {
        Self {
            simvars: Vec::new(),
        }
    }

    pub fn with_simvar(&mut self, var: Simvars, unit: Units) {
        self.simvars.push((var, unit));
    }

    pub fn build(self) -> Self {
        self
    }

    pub fn deserialize_simvars(self) -> Vec<(String, String)> {
        let mut ds_vars = Vec::new();
        for (simvar, unit) in self.simvars {
            ds_vars.push((simvar.name(), unit.get_unit()))
        }
        ds_vars
    }
}
