/*
* structs for all the mutex variables
*/
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::simconnect::simvars::{Simvars, Units};

#[derive(Debug, Clone)]
pub struct MutexVariables {
    bus_voltage: Arc<Mutex<BusVoltages>>,
    simulator_variables: Arc<Mutex<SimulatorVariables>>,
}
impl MutexVariables {
    pub fn new(bus_voltage: BusVoltages, simulator_vars: SimulatorVariables) -> Self {
        Self {
            bus_voltage: Arc::new(Mutex::new(bus_voltage)),
            simulator_variables: Arc::new(Mutex::new(simulator_vars)),
        }
    }
}

pub async fn get_values(vars: MutexVariables) -> Vec<(String, f64, String)> {
    let mut ret_vec: Vec<(String, f64, String)> = Vec::new();

    let locked_bus_voltages = vars.bus_voltage.lock().await;
    ret_vec.push((
        Simvars::AcBus1Voltage.name(),
        locked_bus_voltages.ac_bus1,
        Units::Number.get_unit(),
    ));
    ret_vec.push((
        Simvars::AcBus2Voltage.name(),
        locked_bus_voltages.ac_bus2,
        Units::Number.get_unit(),
    ));
    ret_vec.push((
        Simvars::AcEssBusVoltage.name(),
        locked_bus_voltages.ac_ess_bus,
        Units::Number.get_unit(),
    ));
    ret_vec.push((
        Simvars::AcGndSvcBusVoltage.name(),
        locked_bus_voltages.ac_gnd_svc_bus,
        Units::Number.get_unit(),
    ));
    ret_vec.push((
        Simvars::AcStbyBusVoltage.name(),
        locked_bus_voltages.ac_stby_bus,
        Units::Number.get_unit(),
    ));
    ret_vec.push((
        Simvars::ApuStartBusVoltage.name(),
        locked_bus_voltages.apu_start_bus,
        Units::Number.get_unit(),
    ));
    ret_vec.push((
        Simvars::DcBus1Voltage.name(),
        locked_bus_voltages.dc_bus1,
        Units::Number.get_unit(),
    ));
    ret_vec.push((
        Simvars::DcBus2Voltage.name(),
        locked_bus_voltages.dc_bus2,
        Units::Number.get_unit(),
    ));
    ret_vec.push((
        Simvars::DcEssBus1Voltage.name(),
        locked_bus_voltages.dc_ess_bus1,
        Units::Number.get_unit(),
    ));
    ret_vec.push((
        Simvars::DcEssBus2Voltage.name(),
        locked_bus_voltages.dc_ess_bus2,
        Units::Number.get_unit(),
    ));
    ret_vec.push((
        Simvars::DcEssBus3Voltage.name(),
        locked_bus_voltages.dc_ess_bus3,
        Units::Number.get_unit(),
    ));
    ret_vec.push((
        Simvars::DcGndSvcBusVoltage.name(),
        locked_bus_voltages.dc_gnd_svc_bus,
        Units::Number.get_unit(),
    ));
    ret_vec.push((
        Simvars::HotBatBus1Voltage.name(),
        locked_bus_voltages.hot_bat_bus1,
        Units::Number.get_unit(),
    ));
    ret_vec.push((
        Simvars::HotBatBus2Voltage.name(),
        locked_bus_voltages.hot_bat_bus2,
        Units::Number.get_unit(),
    ));

    ret_vec
}

#[derive(Debug)]
pub struct SimulatorVariables {
    pub aileron_controls_position: f64,
    pub elevator_controls_position: f64,
    pub rudder_controls_position: f64,
}

pub async fn set_vars(mutex: MutexVariables, simulator_vars: SimulatorVariables) {
    let mut mutex_vars = mutex.simulator_variables.lock().await;

    mutex_vars.aileron_controls_position = simulator_vars.aileron_controls_position;
    mutex_vars.elevator_controls_position = simulator_vars.elevator_controls_position;
    mutex_vars.rudder_controls_position = simulator_vars.rudder_controls_position;
}

#[derive(Debug)]
pub struct BusVoltages {
    pub ac_bus1: f64,
    pub ac_bus2: f64,
    pub ac_ess_bus: f64,
    pub ac_gnd_svc_bus: f64,
    pub ac_stby_bus: f64,
    pub dc_gnd_svc_bus: f64,
    pub dc_bus1: f64,
    pub dc_bus2: f64,
    pub dc_ess_bus1: f64,
    pub dc_ess_bus2: f64,
    pub dc_ess_bus3: f64,
    pub hot_bat_bus1: f64,
    pub hot_bat_bus2: f64,
    pub apu_start_bus: f64,
}

#[derive(Debug)]
pub struct ElectricState {
    // we will store state of all electronics power here, bool
}
