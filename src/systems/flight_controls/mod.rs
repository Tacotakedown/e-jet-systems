use self::ailerons::Ailerons;
use self::spoilers::modes::SpoilerModes;
use self::spoilers::spoiler::Spoiler;
use self::spoilers::spoiler_logic::SpoilerController;
use self::spoilers::spoiler_position::ESpoilerPosition;

use std::collections::HashMap;
use std::time::Duration;

pub mod ailerons;
pub mod elevator;
pub mod nosewheel_steering;
pub mod rudder;
pub mod shared;
pub mod spoilers;
pub mod trim;

#[derive(Debug)]
pub struct FlightControls {
    spoilers: HashMap<String, Spoiler>,
    ailerons: Vec<Ailerons>,
}

impl FlightControls {
    pub fn new() -> Self {
        Self {
            spoilers: HashMap::new(),

            ailerons: Vec::new(),
        }
    }
    pub fn with_spoilers(&mut self) {
        let l1_spoiler = Spoiler::new(0.0, ESpoilerPosition::L1);
        let l2_spoiler = Spoiler::new(0.0, ESpoilerPosition::L2);
        let l3_spoiler = Spoiler::new(0.0, ESpoilerPosition::L3);
        let l4_spoiler = Spoiler::new(0.0, ESpoilerPosition::L4);
        let l5_spoiler = Spoiler::new(0.0, ESpoilerPosition::L5);
        let r1_spoiler = Spoiler::new(0.0, ESpoilerPosition::R1);
        let r2_spoiler = Spoiler::new(0.0, ESpoilerPosition::R2);
        let r3_spoiler = Spoiler::new(0.0, ESpoilerPosition::R3);
        let r4_spoiler = Spoiler::new(0.0, ESpoilerPosition::R4);
        let r5_spoiler = Spoiler::new(0.0, ESpoilerPosition::R5);

        self.spoilers.insert(ESpoilerPosition::R1.key(), r1_spoiler);
        self.spoilers.insert(ESpoilerPosition::R2.key(), r2_spoiler);
        self.spoilers.insert(ESpoilerPosition::R3.key(), r3_spoiler);
        self.spoilers.insert(ESpoilerPosition::R4.key(), r4_spoiler);
        self.spoilers.insert(ESpoilerPosition::R5.key(), r5_spoiler);
        self.spoilers.insert(ESpoilerPosition::L1.key(), l1_spoiler);
        self.spoilers.insert(ESpoilerPosition::L2.key(), l2_spoiler);
        self.spoilers.insert(ESpoilerPosition::L3.key(), l3_spoiler);
        self.spoilers.insert(ESpoilerPosition::L4.key(), l4_spoiler);
        self.spoilers.insert(ESpoilerPosition::L5.key(), l5_spoiler);
    }

    pub fn update(
        &self,
        aircraft_on_ground: bool,
        yoke_input: f64,
        pressure: f64,
        speed_brake_handle: f64,
        in_landing_config: bool,
        spoiler_controller: &mut SpoilerController,
        dt: Duration,
    ) {
        spoiler_controller.update_input(yoke_input, aircraft_on_ground, speed_brake_handle);
    }

    // yoke input will be from -40 to 40
    pub fn spoileron_actuation(&self, yoke_input: f64) {
        ()
    }

    // IB on landing on ground > 60 knots at touchdown, retract automatically, handle can be in any position
    // Ob full def on touchdown, in air work as spoileron
    //

    pub fn build(self) -> Self {
        self
    }
}
fn get_actuator_speed(pressure: f64, density: f64, area: f64, coefficent: f64) -> f64 {
    (2.0 * pressure) / (density * area * coefficent)
}
