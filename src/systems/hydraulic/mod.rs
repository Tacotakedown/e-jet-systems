pub mod components;
pub mod fluid;
pub mod hydraulic_line;
pub mod math;
pub mod pump;

use std::collections::HashMap;
use tokio::time::Instant;

use self::hydraulic_line::HydraulicLineMaterial;

#[derive(Debug)]
pub enum ComponentType {
    Reservoir,
    Pump,
    ElecPump,
    FilterManifold,
    Accumulator,
    Valve,
    Actuator,
    PTU,
    PTUSelecorValve,
    PriorityValve,
}

#[derive(Debug)]
struct Component {
    id: String,
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

    pub fn add_component(&mut self, id: String, component_type: ComponentType) {
        let component = Component {
            id: id.clone(),
            component_type,
        };
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
        static mut ENGINE_RPM: f64 = 0.0;
        static mut FLUID_MOVED: f64 = 0.0;
        static mut LAST_TIME: Option<Instant> = None;

        // TODO: instead of looping through all the possible connections, we will hard code all the connections so we can configure the proper logic

        // some variables can just be static so we dont have to pass 50 mutex vars into each system, the only variables that need to be mutex are ones that pass between threads obviously

        //  SYSTEM 1
        fn system_1() {}

        system_1();

        // for connection in &self.connections {
        //     let source_component = self.components.get(&connection.source_id);
        //     let target_component = self.components.get(&connection.target_id);

        //     if let (Some(source), Some(target)) = (source_component, target_component) {
        //         match &source.component_type {
        //             ComponentType::Pump => {
        //                 let mut pump = EngineDrivenHydraulicPump::new();
        //                 let max_rpm = 4825.0;
        //                 let rpm_step = max_rpm / 10.0; // Adjust the step size as needed
        //                 let current_time = Instant::now();
        //                 let dt = if let Some(last_time) = unsafe { &LAST_TIME } {
        //                     current_time.duration_since(*last_time).as_secs_f64()
        //                 } else {
        //                     0.0
        //                 };
        //                 unsafe { LAST_TIME = Some(current_time) };
        //                 for rpm in (0..=max_rpm as i32).step_by(rpm_step as usize) {
        //                     unsafe { ENGINE_RPM = rpm as f64 };
        //                     pump.set_engine_rpm(unsafe { ENGINE_RPM });
        //                     pump.enable_compensator();
        //                     let flow_this_tick = pump.calculate_volume_flow(dt);
        //                     unsafe {
        //                         FLUID_MOVED += flow_this_tick;
        //                     }
        //                     println!(
        //                         "\r Pump RPM: {:.2}, Flow (L this tick): {}",
        //                         unsafe { ENGINE_RPM },
        //                         flow_this_tick
        //                     );
        //                 }
        //             }
        //             _ => {}
        //         }
        //         match &target.component_type {
        //             ComponentType::Valve => {}
        //             _ => {}
        //         }
        //     }
        // }
    }

    pub async fn simulate_system_async(&mut self) {
        self.simulate()
    }
}
