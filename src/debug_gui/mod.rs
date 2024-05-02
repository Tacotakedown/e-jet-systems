use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::time::Instant;
use tokio::time::sleep;

use eframe::egui;

use crate::mutex::{HydraulicVars, MutexVariables, System1Vars};
use crate::systems::hydraulic::math::pa_to_psi;

#[derive(Debug)]
pub struct DebugGui {
    width: f32,
    height: f32,
    name: String,
    last_update_time: Instant,
    cached_data: Option<Vec<(String, f64, String)>>,
    update_interval: Duration,
}

impl DebugGui {
    pub fn new(width: f32, height: f32, name: String, update_interval_secs: u64) -> Self {
        Self {
            width,
            height,
            name,
            last_update_time: Instant::now(),
            cached_data: None,
            update_interval: Duration::from_secs(update_interval_secs),
        }
    }

    pub async fn render(&mut self, gui: Arc<Mutex<DebugGui>>) -> Result<(), eframe::Error> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([self.width, self.height]),
            ..Default::default()
        };

        eframe::run_simple_native(&self.name, options, move |ctx, _frame| {
            let gui = gui.lock().unwrap();
            let cached_data = gui.cached_data.clone();

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label(format!("System Vars: {:?}", cached_data));
            });

            ctx.request_repaint();
        })
        .expect("failed to start native GUI");

        Ok(())
    }
}
pub async fn get_values(mutex_vars: MutexVariables) -> Vec<(String, f64, String)> {
    let mut ret_vec: Vec<(String, f64, String)> = Vec::new();

    let locked_hydraulic_vars = mutex_vars.read_hydraulic_vars().await;
    let system1 = locked_hydraulic_vars.system1;

    ret_vec.push((
        "Reservoir Level".to_string(),
        system1.reservoir_level,
        "m".to_string(),
    ));
    ret_vec.push((
        "Engine Driven Pump RPM".to_string(),
        system1.engine_driven_pump_rpm,
        "rpm".to_string(),
    ));
    ret_vec.push((
        "AC Motor Pump State".to_string(),
        if system1.ac_motor_pump_state {
            1.0
        } else {
            0.0
        },
        "on/off".to_string(),
    ));
    ret_vec.push((
        "Pre-Manifold Pressure".to_string(),
        pa_to_psi(system1.pre_manifold_pressure),
        "psi".to_string(),
    ));
    ret_vec.push((
        "Post-Manifold Pressure".to_string(),
        pa_to_psi(system1.post_maifold_pressure),
        "psi".to_string(),
    ));
    ret_vec.push((
        "LH Thrust Reverser Position".to_string(),
        system1.lh_thrust_reverser_position,
        "percent".to_string(),
    ));

    ret_vec
}

pub async fn ui_updater(mutex_vars: MutexVariables, gui: Arc<Mutex<DebugGui>>) {
    loop {
        let fut = get_values(mutex_vars.clone()).await;
        if !fut.is_empty() {
            let mut gui = gui.lock().unwrap();
            gui.cached_data = Some(fut);
        }

        sleep(Duration::from_millis(100)).await;
    }
}
