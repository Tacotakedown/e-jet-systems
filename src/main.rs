use tokio::task;

use crate::mutex::{
    BusVoltages, ElectricState, HydraulicVars, MutexVariables, SimulatorVariables, System1Vars,
};
use crate::server::api_factory;
use crate::simconnect::Simconnect;
use crate::systems::{brake_system, electrical, hydraulic_system};

mod mutex;
mod server;
mod simconnect;
mod systems;

#[tokio::main]
async fn main() {
    let simconnect = Simconnect::new("OBJ_SIMCONNECT".to_string());

    let mutex_vars = MutexVariables::new(
        BusVoltages {
            ac_bus1: 0.0,
            ac_bus2: 0.0,
            ac_ess_bus: 0.0,
            ac_gnd_svc_bus: 0.0,
            ac_stby_bus: 0.0,
            dc_gnd_svc_bus: 0.0,
            dc_bus1: 0.0,
            dc_bus2: 0.0,
            dc_ess_bus1: 0.0,
            dc_ess_bus2: 0.0,
            dc_ess_bus3: 0.0,
            hot_bat_bus1: 0.0,
            hot_bat_bus2: 0.0,
            apu_start_bus: 0.0,
        },
        SimulatorVariables {
            aileron_controls_position: 0.0,
            elevator_controls_position: 0.0,
            rudder_controls_position: 0.0,
        },
        HydraulicVars {
            system1: System1Vars {
                reservoir_level: 12.3,
                engine_driven_pump_rpm: 0.0,
                ac_motor_pump_state: false,
                pre_manifold_pressure: 0.0,
                post_maifold_pressure: 0.0,
                lh_thrust_reverser_position: 0.0,
            },
        },
    );

    let brake_thread = task::spawn(brake_system());
    let electrical_thread = task::spawn(electrical());
    let hydraulic_thread = task::spawn(hydraulic_system(mutex_vars));

    let api_thread = task::spawn(api_factory());

    // let simvar_update_thread = task::spawn(simconnect.update(mutex_vars));

    println!("REST API server running on port 3030");

    let _ = tokio::try_join!(
        brake_thread,
        electrical_thread,
        hydraulic_thread,
        api_thread,
        //  simvar_update_thread
    );
}
