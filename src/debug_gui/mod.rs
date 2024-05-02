use eframe::egui::{self, Id};
use egui::Color32;
use egui_plot::{Line, PlotPoints};
use std::collections::HashMap;
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
}

impl DebugGui {
    pub fn new(width: f32, height: f32, name: String) -> Self {
        Self {
            width,
            height,
            name,
            cached_data: None,
        }
    }

    pub async fn render(
        &mut self,
        gui: Arc<Mutex<DebugGui>>,
        data: Arc<Mutex<HashMap<String, Vec<f64>>>>,
        buttons: Arc<Mutex<HashMap<String, bool>>>,
    ) -> Result<(), eframe::Error> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([self.width, self.height]),
            ..Default::default()
        };
        let data_clone = data.clone();
        eframe::run_simple_native(&self.name, options, move |ctx, _frame| {
            let gui = gui.lock().unwrap();
            let cached_data = gui.cached_data.clone();

            let data_locked = data_clone.lock().unwrap();

            let points = data_locked.get("pressure").cloned().unwrap_or_default();
            let points_post = data_locked
                .get("post_manifold_pressure")
                .cloned()
                .unwrap_or_default();
            let points_res = data_locked
                .get("reservoir_level")
                .cloned()
                .unwrap_or_default();

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("Pumps: Red - pre manifold, Blue - post manifold");
                ui.end_row();
                plot_pumps(ui, points, points_post);
                ui.end_row();
                ui.label("Reservoir Level");
                plot_res_level(ui, points_res);
                ui.end_row();
                static mut ELEC_BUTTON_STATE: bool = false;
                if ui.button("Toggle Elec Pumps").clicked() {
                    unsafe { ELEC_BUTTON_STATE = !ELEC_BUTTON_STATE }
                    let mut elec_pump_mutex = buttons.lock().unwrap();
                    let entry = elec_pump_mutex
                        .entry("elec_pump_state".to_string())
                        .or_insert(false);
                    *entry = unsafe { ELEC_BUTTON_STATE };
                }
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

    ret_vec // TODO: return a struct from this instead of a vec
}

pub async fn ui_updater(
    mutex_vars: MutexVariables,
    gui: Arc<Mutex<DebugGui>>,
    plot_data: Arc<Mutex<HashMap<String, Vec<f64>>>>,
    button_vars: Arc<Mutex<HashMap<String, bool>>>,
) {
    loop {
        let fut = get_values(mutex_vars.clone()).await;
        let vars = mutex_vars.clone().read_hydraulic_vars().await;
        let button_vars_clone = button_vars.clone();
        let mut new_hydrauliuc_vars = HydraulicVars {
            system1: System1Vars {
                reservoir_level: vars.system1.reservoir_level,
                engine_driven_pump_rpm: vars.system1.engine_driven_pump_rpm,
                ac_motor_pump_state: false,
                pre_manifold_pressure: vars.system1.pre_manifold_pressure,
                post_maifold_pressure: vars.system1.post_maifold_pressure,
                lh_thrust_reverser_position: vars.system1.lh_thrust_reverser_position,
            },
        };
        if !fut.is_empty() {
            let mut gui = gui.lock().unwrap();
            let button_vars = button_vars_clone.lock().unwrap();
            gui.cached_data = Some(fut);

            let mut pressure_data_mutex = plot_data.lock().unwrap();
            let pressure_data_vec = pressure_data_mutex
                .entry("pressure".to_string())
                .or_insert_with(Vec::new);
            pressure_data_vec.push(pa_to_psi(vars.system1.pre_manifold_pressure));
            if pressure_data_vec.len() > 2000 {
                pressure_data_vec.remove(0);
            }

            let pressure_data_post_manifold_vec = pressure_data_mutex
                .entry("post_manifold_pressure".to_string())
                .or_insert_with(Vec::new);
            pressure_data_post_manifold_vec.push(pa_to_psi(vars.system1.post_maifold_pressure));
            if pressure_data_post_manifold_vec.len() > 2000 {
                pressure_data_post_manifold_vec.remove(0);
            }

            let reservoir_level_vec = pressure_data_mutex
                .entry("reservoir_level".to_string())
                .or_insert_with(Vec::new);
            reservoir_level_vec.push(vars.system1.reservoir_level);
            if reservoir_level_vec.len() > 2000 {
                reservoir_level_vec.remove(0);
            }

            let elec_pump_new_state = button_vars
                .get("elec_pump_state")
                .cloned()
                .unwrap_or_default();

            let engine_pump_new_state = button_vars
                .get("engine_pump_new_state")
                .cloned()
                .unwrap_or_default();

            new_hydrauliuc_vars = HydraulicVars {
                system1: System1Vars {
                    reservoir_level: vars.system1.reservoir_level,
                    engine_driven_pump_rpm: vars.system1.engine_driven_pump_rpm,
                    ac_motor_pump_state: elec_pump_new_state,
                    pre_manifold_pressure: vars.system1.pre_manifold_pressure,
                    post_maifold_pressure: vars.system1.post_maifold_pressure,
                    lh_thrust_reverser_position: vars.system1.lh_thrust_reverser_position,
                },
            };
        }

        mutex_vars.write_hydraulic_vars(new_hydrauliuc_vars).await;

        tokio::time::sleep(Duration::from_millis(60)).await;
    }
}

fn plot_pumps(
    ui: &mut egui::Ui,
    points: Vec<f64>,
    points_post_manifold: Vec<f64>,
) -> egui::Response {
    let spacing_factor = 20.0;

    let line_points: PlotPoints = points
        .iter()
        .enumerate()
        .map(|(i, &value)| [i as f64 * spacing_factor, value])
        .collect();

    let line_points_post_manifold: PlotPoints = points_post_manifold
        .iter()
        .enumerate()
        .map(|(i, &value)| [i as f64 * spacing_factor, value])
        .collect();

    let line1 = Line::new(line_points).color(Color32::RED);
    let line2 = Line::new(line_points_post_manifold).color(Color32::BLUE);

    egui_plot::Plot::new("example_plot")
        .height(500.0)
        .show_axes(true)
        .data_aspect(1.0)
        .show(ui, |plot_ui| {
            plot_ui.line(line1);
            plot_ui.line(line2)
        })
        .response
}

fn plot_res_level(ui: &mut egui::Ui, points: Vec<f64>) -> egui::Response {
    let spacing_factor = 1.0;

    let line_points: PlotPoints = points
        .iter()
        .enumerate()
        .map(|(i, &value)| [i as f64 * spacing_factor, value])
        .collect();

    let line1 = Line::new(line_points).color(Color32::GOLD);

    egui_plot::Plot::new("example_plot")
        .height(500.0)
        .show_axes(true)
        .data_aspect(1.0)
        .id(Id::new(32131))
        .show(ui, |plot_ui| {
            plot_ui.line(line1);
        })
        .response
}
