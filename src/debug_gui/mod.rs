use eframe::egui;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

pub mod plotter;

use crate::mutex::{HydraulicVars, MutexVariables, System1Vars};
use crate::systems::hydraulic::math::pa_to_psi;
use plotter::PressureData;

#[derive(Debug)]
pub struct DebugGui {
    width: f32,
    height: f32,
    name: String,
    cached_data: Option<Vec<(String, f64, String)>>,
    pressure_data: Vec<f64>,
}

impl DebugGui {
    pub fn new(width: f32, height: f32, name: String) -> Self {
        Self {
            width,
            height,
            name,
            cached_data: None,
            pressure_data: Vec::new(),
        }
    }

    pub async fn render(
        &mut self,
        gui: Arc<Mutex<DebugGui>>,
        pressure_data: Arc<Mutex<Vec<f64>>>,
    ) -> Result<(), eframe::Error> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([self.width, self.height]),
            ..Default::default()
        };
        let pressure_data_clone = pressure_data.clone();
        eframe::run_simple_native(&self.name, options, move |ctx, _frame| {
            let gui = gui.lock().unwrap();
            let cached_data = gui.cached_data.clone();

            let pressure_data = pressure_data_clone.lock().unwrap();
            let points = pressure_data.clone();

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label(format!("System Vars: {:?}", cached_data));
                ui.end_row();
                example_plot(ui, points);
                ui.end_row();
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

pub async fn ui_updater(
    mutex_vars: MutexVariables,
    gui: Arc<Mutex<DebugGui>>,
    pressure_data: Arc<Mutex<Vec<f64>>>,
) {
    let mut pressure_data_struct = PressureData::new(1000);
    loop {
        let fut = get_values(mutex_vars.clone()).await;
        let pressure = mutex_vars.clone().read_hydraulic_vars().await;

        if !fut.is_empty() {
            let mut gui = gui.lock().unwrap();
            gui.cached_data = Some(fut);
            pressure_data_struct.add_value(pa_to_psi(pressure.system1.pre_manifold_pressure));
            let mut pressure_data_mutex = pressure_data.lock().unwrap();
            *pressure_data_mutex = pressure_data_struct.get_values();
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

fn example_plot(ui: &mut egui::Ui, points: Vec<f64>) -> egui::Response {
    use egui_plot::{Line, PlotPoints};

    let spacing_factor = 50.0;

    let line_points: PlotPoints = points
        .iter()
        .enumerate()
        .map(|(i, &value)| [i as f64 * spacing_factor, value])
        .collect();

    let line = Line::new(line_points);
    egui_plot::Plot::new("example_plot")
        .height(500.0)
        .show_axes(true)
        .data_aspect(1.0)
        .show(ui, |plot_ui| plot_ui.line(line))
        .response
}
