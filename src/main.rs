use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::debug_gui::{ui_updater, DebugGui};
use crate::mutex::{
    BusVoltages, ElectricState, HydraulicVars, MutexVariables, SimulatorVariables, System1Vars,
};
use crate::server::api_factory;
use crate::simconnect_interface::simconnect_interface_mod::simconnect_thread_fn;
use crate::systems::{brake_system, electrical, flight_controls, hydraulic_system};

mod debug_gui;
mod mutex;
mod server;
mod simconnect_interface;
mod systems;

#[tokio::main]
async fn main() {
    #[cfg(feature = "gui")]
    let gui = Arc::new(Mutex::new(DebugGui::new(
        400.0,
        300.0,
        "Systems".to_string(),
    )));

    #[cfg(feature = "gui")]
    let data_mutex = Arc::new(Mutex::new(HashMap::new()));
    #[cfg(feature = "gui")]
    let button_mutex = Arc::new(Mutex::new(HashMap::new()));
    #[cfg(feature = "gui")]
    let gui_simvar_mutex = Arc::new(Mutex::new(HashMap::new()));

    #[cfg(feature = "gui")]
    let mut render_gui = DebugGui::new(1800.0, 1000.0, "Systems".to_string());

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
            spoiler_handle_0_to_100: 0.0,
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

    let brake_thread = tokio::spawn(brake_system(mutex_vars.clone()));
    let electrical_thread = tokio::spawn(electrical());
    let hydraulic_thread = tokio::spawn(hydraulic_system(mutex_vars.clone()));
    let flight_control_thread = tokio::spawn(flight_controls(mutex_vars.clone()));

    let api_thread = tokio::spawn(api_factory());
    #[cfg(target_os = "windows")]
    let simvar_update_thread = tokio::spawn(simconnect_thread_fn(mutex_vars.clone()));

    #[cfg(not(target_os = "windows"))]
    let simvar_update_thread = tokio::spawn(simconnect_thread_fn(mutex_vars.clone()));

    println!("REST API server running on port 3030");

    #[cfg(feature = "gui")]
    let ui_updater_thread = tokio::spawn(ui_updater(
        mutex_vars.clone(),
        gui.clone(),
        data_mutex.clone(),
        button_mutex.clone(),
        gui_simvar_mutex.clone(),
    ));

    #[cfg(not(feature = "gui"))]
    let ui_updater_thread = tokio::spawn(async move {});

    #[cfg(feature = "gui")]
    println!("Spawned all system threads, Blocking on GUI");
    #[cfg(feature = "gui")]
    if let Err(err) = render_gui
        .render(
            gui.clone(),
            data_mutex.clone(),
            button_mutex.clone(),
            gui_simvar_mutex.clone(),
        )
        .await
    {
        eprintln!("Error rendering GUI: {:?}", err);
    }
    //we can block here because each thread is a infinite loop so the following code is essentially useless

    println!("spawned all threads, No blocking");

    let join_res = tokio::try_join!(
        brake_thread,
        electrical_thread,
        hydraulic_thread,
        api_thread,
        flight_control_thread,
        simvar_update_thread,
        ui_updater_thread
    );

    match join_res {
        Ok(_) => {}
        Err(e) => {
            eprintln!("thread join error, its amazing that the code got this far, you really broke it bad: {}",e)
        }
    }
}
