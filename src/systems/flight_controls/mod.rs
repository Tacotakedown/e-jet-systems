use self::ailerons::Ailerons;
use self::spoilers::modes::SpoilerModes;
use self::spoilers::spoiler::Spoiler;
use self::spoilers::spoiler_position::SpoilerPosition;

pub mod ailerons;
pub mod elevator;
pub mod rudder;
pub mod shared;
pub mod spoilers;
pub mod trim;

#[derive(Debug)]
pub struct FlightControls {
    spoilers: Vec<Spoiler>,
    spoiler_mode: SpoilerModes,
    ailerons: Vec<Ailerons>,
}

impl FlightControls {
    pub fn new(spoiler_mode: SpoilerModes) -> Self {
        Self {
            spoilers: Vec::new(),
            spoiler_mode,
            ailerons: Vec::new(),
        }
    }
    pub fn with_spoiler(&mut self, pcu_input: f64, spoiler_position: SpoilerPosition) {
        let spoiler = Spoiler::new(pcu_input, spoiler_position);
        self.spoilers.push(spoiler);
    }
    // vec pcu input from hydraulic system has to be 10 long
    pub fn update(aircraft_on_ground: bool, pcu_input: Vec<f64>, yoke_input: f64) {}

    // yoke input will be from -40 to 40
    pub fn spoileron_actuation(yoke_input: f64) {
        ()
    }
    pub fn get_spoiler_mode(self) -> SpoilerModes {
        self.spoiler_mode
    }

    pub fn set_spoiler_mode(&mut self, mode: SpoilerModes) {
        self.spoiler_mode = mode
    }

    // IB on landing on ground > 60 knots at touchdown, retract automatically, handle can be in any position
    // Ob full def on touchdown, in air work as spoileron
    //

    pub fn build(self) -> Self {
        self
    }
}
