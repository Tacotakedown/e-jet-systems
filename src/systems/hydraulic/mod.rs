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

    fn simulate(&self) {
        static mut LAST_TIME: Lazy<Instant> = Lazy::new(|| Instant::now());
        static mut SYSTEM_1_PRESSURE: f64 = 0.;

        let hydraulic_fluid = HydraulicFluid::new();

        //  SYSTEM 1
        fn system_1(fluid: HydraulicFluid) {
            static mut RESERVOIR_LEVEL: f64 = 12.3; // TODO: mutex var
            static mut ENGINE_RPM: f64 = 4825.;
            static mut SYS1_AC_PUMP_CONTROLLER: bool = false;
            const FLUID_TEMP: f64 = 35.;

            let viscosity = fluid.get_viscosity(FLUID_TEMP);

            let mut reservoir = Reservoir::new();
            reservoir.set_fluid_level(unsafe { RESERVOIR_LEVEL });

            let mut engine_driven_pump = EngineDrivenHydraulicPump::new();
            engine_driven_pump.set_engine_rpm(unsafe { ENGINE_RPM });
            engine_driven_pump.enable_compensator();

            let mut elec_pump = AcMotorPump::new();
            elec_pump.set_power_state(unsafe { SYS1_AC_PUMP_CONTROLLER });

            let current_time: Instant = Instant::now();
            let last_time = unsafe { addr_of!(LAST_TIME) };
            let dt = current_time
                .duration_since(unsafe { **last_time })
                .as_secs_f64();

            let ac_pump_flow = elec_pump.get_output(dt);

            if unsafe { pa_to_psi(SYSTEM_1_PRESSURE) >= 3000. } {
                engine_driven_pump.disable_compensator();
            } else {
                engine_driven_pump.enable_compensator()
            }

            if unsafe { pa_to_psi(SYSTEM_1_PRESSURE) >= 2700. } {
                unsafe { SYS1_AC_PUMP_CONTROLLER = false }
            } else {
                unsafe { SYS1_AC_PUMP_CONTROLLER = true }
            }

            let mut flow_this_tick = engine_driven_pump.calculate_volume_flow(dt);

            if unsafe { RESERVOIR_LEVEL - flow_this_tick } <= 0. {
                flow_this_tick = unsafe { RESERVOIR_LEVEL }
            }

            unsafe { RESERVOIR_LEVEL -= flow_this_tick + ac_pump_flow }
            const CROSS_SECTIONAL_AREA: f64 = 0.009;

            let velocity = (flow_this_tick + ac_pump_flow) / (CROSS_SECTIONAL_AREA / 100.);
            let pressure_increase = 0.5 * fluid.density_kg_m_3_60f * velocity.powf(2.)
                + engine_driven_pump.get_leakback();

            fn pa_to_psi(pressure_pa: f64) -> f64 {
                const PSI_TO_PA: f64 = 6894.76;
                pressure_pa / PSI_TO_PA
            }
            clear();
            println!("pressure: {:.4}", pa_to_psi(unsafe { SYSTEM_1_PRESSURE }));

            println!(
                "\rlevel:{:.4} flow: {:.4}",
                unsafe { RESERVOIR_LEVEL },
                flow_this_tick + ac_pump_flow
            );

            unsafe { *LAST_TIME = current_time };
            unsafe { SYSTEM_1_PRESSURE += pressure_increase }
            //
        }

        system_1(hydraulic_fluid);
    }

    pub async fn simulate_system_async(&mut self) {
        self.simulate()
    }
}
