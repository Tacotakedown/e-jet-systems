pub mod accumulator;
pub mod actuators;
pub mod components;
pub mod filter_manifold;
pub mod fluid;
pub mod hydraulic_line;
pub mod math;
pub mod pump;
pub mod reservoir;

use core::ptr::addr_of;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::process::Command;
use tokio::time::Instant;

use crate::mutex::HydraulicVars;
use crate::mutex::MutexVariables;
use crate::systems::hydraulic::filter_manifold::FilterManifold;
use crate::systems::shared::reduce_by_percentage;

use self::fluid::HydraulicFluid;
use self::hydraulic_line::HydraulicLineMaterial;
use self::pump::{ac_motor_pump::AcMotorPump, engine_driven_pump::EngineDrivenHydraulicPump};
use self::reservoir::Reservoir;

fn clear() {
    if cfg!(windows) {
        let _ = Command::new("cmd").arg("/c").arg("cls").status();
    } else {
        let _ = Command::new("clear").status();
    }
}

#[derive(Debug, Clone)]
pub enum ComponentType {
    Reservoir,
    Pump,
    ElecPump,
    FilterManifold,
    Accumulator,
    UnloaderValve,
    FlowLimitValve,
    GearManifold,
    Actuator,
    PTU,
    PTUSelecorValve,
    PriorityValve,
}

#[derive(Debug, Clone)]
struct Component {
    component_type: ComponentType,
}

#[derive(Debug)]
struct Connection {
    source_id: String,
    target_id: String,
    flow_direction: FlowDirection,
    length: f64,
    material: HydraulicLineMaterial,
}

#[derive(Debug)]
pub enum FlowDirection {
    InletToOutlet,
    OutletToInlet,
    Bidirectional, // ONLY FOR PTU!! right now there are no check valves so this enum is working as one (bidirectional is essentially disabling it)
}

pub struct HydraulicSystem {
    components: HashMap<String, Component>,
    connections: Vec<Connection>,
}

impl HydraulicSystem {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            connections: Vec::new(),
        }
    }
    fn get_component_by_id(&self, id: String) -> Component {
        let component = self.components.clone();
        let component = component.get(&id).unwrap();
        component.clone()
    }

    pub fn add_component(&mut self, id: String, component_type: ComponentType) {
        let component = Component { component_type };
        self.components.insert(id, component);
    }

    pub fn add_connection(
        &mut self,
        source_id: String,
        target_id: String,
        flow_direction: FlowDirection,
        length: f64,
        material: HydraulicLineMaterial,
    ) {
        let connection = Connection {
            source_id,
            target_id,
            flow_direction,
            length,
            material,
        };
        self.connections.push(connection);
    }

    pub async fn simulate(&self, mutex_vars: MutexVariables) {
        let hydraulic_fluid = HydraulicFluid::new();

        //  SYSTEM 1
        async fn system_1(fluid: HydraulicFluid, mutex_vars: MutexVariables) {
            static mut LAST_TIME: Lazy<Instant> = Lazy::new(|| Instant::now());
            let mut read_mutex_vars: HydraulicVars = mutex_vars.read_hydraulic_vars().await;
            const ENGINE_RPM: f64 = 4825.;
            static mut SYS1_AC_PUMP_CONTROLLER: bool = false;

            const FLUID_TEMP: f64 = 35.;

            let viscosity = fluid.get_viscosity(FLUID_TEMP);

            let mut reservoir = Reservoir::new();
            reservoir.set_fluid_level(read_mutex_vars.system1.reservoir_level);

            let mut engine_driven_pump = EngineDrivenHydraulicPump::new();
            engine_driven_pump.set_engine_rpm(ENGINE_RPM);

            let mut elec_pump = AcMotorPump::new();
            elec_pump.set_power_state(read_mutex_vars.system1.ac_motor_pump_state);

            let current_time: Instant = Instant::now();
            let last_time = unsafe { addr_of!(LAST_TIME) };
            let dt = current_time
                .duration_since(unsafe { **last_time })
                .as_secs_f64();

            if pa_to_psi(read_mutex_vars.system1.pre_manifold_pressure) >= 3000. {
                engine_driven_pump.disable_compensator();
            } else {
                engine_driven_pump.enable_compensator();
                // engine_driven_pump.disable_compensator();
            }

            if pa_to_psi(read_mutex_vars.system1.pre_manifold_pressure) >= 2700. {
                elec_pump.set_pressure_override(true);
            } else {
                elec_pump.set_pressure_override(false);
            }

            let mut ac_pump_flow =
                elec_pump.get_output(dt, pa_to_psi(read_mutex_vars.system1.pre_manifold_pressure));

            let mut flow_this_tick = engine_driven_pump.calculate_volume_flow(
                dt,
                pa_to_psi(read_mutex_vars.system1.pre_manifold_pressure),
            );

            if read_mutex_vars.system1.reservoir_level - flow_this_tick <= 0. {
                flow_this_tick = read_mutex_vars.system1.reservoir_level
            }
            if read_mutex_vars.system1.reservoir_level - ac_pump_flow <= 0. {
                ac_pump_flow = read_mutex_vars.system1.reservoir_level
            }

            read_mutex_vars.system1.reservoir_level -= flow_this_tick + ac_pump_flow;
            const CROSS_SECTIONAL_AREA: f64 = 0.009;

            let velocity = (flow_this_tick + ac_pump_flow) / (CROSS_SECTIONAL_AREA / 100.);
            let pressure_increase = 0.5 * fluid.density_kg_m_3_60f * velocity.powf(2.)
                + engine_driven_pump.get_leakback();

            read_mutex_vars.system1.pre_manifold_pressure += pressure_increase;

            fn pa_to_psi(pressure_pa: f64) -> f64 {
                const PSI_TO_PA: f64 = 6894.76;
                pressure_pa / PSI_TO_PA
            }
            //clear();

            // println!(
            //     "\rPressure pre manifold: {:.4} \nPressure at actuators: {:.4}\nReservoir level:{:.4} \nFlow: {:.4}",
            //    pa_to_psi(read_mutex_vars.system1.pre_manifold_pressure),
            //     pa_to_psi(read_mutex_vars.system1.post_maifold_pressure),
            //     read_mutex_vars.system1.reservoir_level,
            //     flow_this_tick + ac_pump_flow,
            // );

            let filter_manifold = FilterManifold::new(15.0);
            let pressure_drop =
                filter_manifold.calculate_pressure_drop(flow_this_tick / 60., viscosity);

            read_mutex_vars.system1.post_maifold_pressure =
                read_mutex_vars.system1.pre_manifold_pressure - pressure_drop * 6894.76;

            let percentage_compressability = fluid.get_volume_change_from_compression_percent(
                pa_to_psi(read_mutex_vars.system1.pre_manifold_pressure),
                15.,
            );

            let returned_to_res = reduce_by_percentage(flow_this_tick, percentage_compressability);

            read_mutex_vars.system1.reservoir_level += returned_to_res;

            mutex_vars.write_hydraulic_vars(read_mutex_vars).await;

            unsafe { *LAST_TIME = current_time };
        }

        async fn system_2(fluid: HydraulicFluid, mutex_vars: MutexVariables) {
            static mut LAST_TIME: Lazy<Instant> = Lazy::new(|| Instant::now());
            let mut read_mutex_vars: HydraulicVars = mutex_vars.read_hydraulic_vars().await;
            const FLUID_TEMP: f64 = 35.; //TODO: make into mutex
            let viscosity = fluid.get_viscosity(FLUID_TEMP);
            // TODO: fn
            let current_time: Instant = Instant::now();

            mutex_vars.write_hydraulic_vars(read_mutex_vars).await;
            unsafe { *LAST_TIME = current_time };
        }

        async fn system_3(fluid: HydraulicFluid, mutex_vars: MutexVariables) {
            static mut LAST_TIME: Lazy<Instant> = Lazy::new(|| Instant::now());
            let mut read_mutex_vars: HydraulicVars = mutex_vars.read_hydraulic_vars().await;
            const FLUID_TEMP: f64 = 35.; // TODO: make into mutex
            let viscosity = fluid.get_viscosity(FLUID_TEMP);
            // TODO: fn
            let current_time: Instant = Instant::now();

            mutex_vars.write_hydraulic_vars(read_mutex_vars).await;
            unsafe { *LAST_TIME = current_time };
        }

        system_1(hydraulic_fluid.clone(), mutex_vars.clone()).await;
        system_2(hydraulic_fluid.clone(), mutex_vars.clone()).await;
        system_3(hydraulic_fluid.clone(), mutex_vars.clone()).await;
    }
}
