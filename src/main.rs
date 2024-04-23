use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tokio::task;
use tokio::time::sleep;

use crate::nav_server::api_factory;
use crate::simconnect::simvars::{SimVarBuilder, Simvars, Units};
use crate::systems::{brake_system, electrical};

mod nav_server;
mod simconnect;
mod systems;
mod terrain_radar;
mod weather_radar;

async fn log_time(name: &'static str) {
    loop {
        let start_time = Instant::now();
        sleep(Duration::from_millis(100)).await;
        let elapsed = start_time.elapsed().as_secs_f64();
        println!("{}: {:.2} seconds", name, elapsed);
    }
}

#[derive(Debug)]
struct BusVoltages {
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
struct ElecticState {
    // we will store state of all electronics power here, bool
}

#[derive(Debug, Clone)]
pub struct MutexVariables {
    bus_voltage: Arc<Mutex<BusVoltages>>,
}
impl MutexVariables {
    pub fn get_values(self) -> Vec<(String, f64)> {
        let ret_vec = Vec::new();

        ret_vec
    }
}

#[tokio::main]
async fn main() {
    let simvars =
        SimVarBuilder::simvar_builder().with_simvar(Simvars::AcBus1Voltage, Units::Number);

    let brake_thread = task::spawn(brake_system());
    let electrical_thread = task::spawn(electrical());

    let api_thread = task::spawn(api_factory());

    println!("REST API server running on port 3030");

    let _ = tokio::try_join!(brake_thread, electrical_thread, api_thread);
}
