pub mod fluid;
pub mod pump;

use std::collections::HashMap;
use tokio::time::{sleep, Duration, Instant};

use self::pump::engine_driven_pump::EngineDrivenHydraulicPump;

// Define enums and structs for components and connections
#[derive(Debug)]
pub enum ComponentType {
    Pump,
    Valve,
    Actuator,
}

#[derive(Debug)]
struct Component {
    id: usize,
    component_type: ComponentType,
    // Other component properties
}

#[derive(Debug)]
struct Connection {
    source_id: usize,
    target_id: usize,
    flow_direction: FlowDirection,
    // Other connection properties
}

#[derive(Debug)]

pub enum FlowDirection {
    InletToOutlet,
    OutletToInlet,
}

// Define the HydraulicSystem struct to manage components and connections
pub struct HydraulicSystem {
    components: HashMap<usize, Component>,
    connections: Vec<Connection>,
    // Other system properties and methods
}

impl HydraulicSystem {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            connections: Vec::new(),
        }
    }

    pub fn add_component(&mut self, id: usize, component_type: ComponentType) {
        let component = Component { id, component_type };
        self.components.insert(id, component);
    }

    pub fn add_connection(
        &mut self,
        source_id: usize,
        target_id: usize,
        flow_direction: FlowDirection,
    ) {
        let connection = Connection {
            source_id,
            target_id,
            flow_direction,
        };
        self.connections.push(connection);
    }

    fn simulate(&self) {
        static mut ENGINE_RPM: f64 = 0.0;
        static mut FLUID_MOVED: f64 = 0.0;
        static mut LAST_TIME: Option<Instant> = None;

        for connection in &self.connections {
            let source_component = self.components.get(&connection.source_id);
            let target_component = self.components.get(&connection.target_id);

            if let (Some(source), Some(target)) = (source_component, target_component) {
                match &source.component_type {
                    ComponentType::Pump => {
                        let mut pump = EngineDrivenHydraulicPump::new();
                        let max_rpm = 4825.0;
                        let rpm_step = max_rpm / 10.0; // Adjust the step size as needed
                        let current_time = Instant::now();
                        let dt = if let Some(last_time) = unsafe { &LAST_TIME } {
                            current_time.duration_since(*last_time).as_secs_f64()
                        } else {
                            0.0
                        };
                        unsafe { LAST_TIME = Some(current_time) };
                        for rpm in (0..=max_rpm as i32).step_by(rpm_step as usize) {
                            unsafe { ENGINE_RPM = rpm as f64 };
                            pump.set_engine_rpm(unsafe { ENGINE_RPM });
                            pump.enable_compensator();
                            let flow_this_tick = pump.calculate_volume_flow(dt);
                            unsafe {
                                FLUID_MOVED += flow_this_tick;
                            }
                            println!(
                                "\r Pump RPM: {:.2}, Flow (L this tick): {}",
                                unsafe { ENGINE_RPM },
                                flow_this_tick
                            );
                        }
                    }
                    _ => {}
                }
                match &target.component_type {
                    ComponentType::Valve => {}
                    _ => {}
                }
            }
        }
    }

    pub async fn simulate_system_async(&mut self) {
        self.simulate()
    }
}
