use std::time::Duration;

use super::spoiler_position::ESpoilerPosition;

#[derive(Debug)]
pub struct Spoiler {
    demanded_position: f64,
    spoiler_position: ESpoilerPosition,
    deflection: f64, // 0-60 degrees
}

impl Spoiler {
    pub fn new(demanded_position: f64, spoiler_position: ESpoilerPosition) -> Self {
        Self {
            demanded_position,
            spoiler_position,
            deflection: 0.0,
        }
    }

    pub fn update(&self, pressure_psi: f64, dt: Duration) {}
}

fn get_actuator_speed(pressure: f64, density: f64, area: f64, coefficent: f64) -> f64 {
    (2.0 * pressure) / (density * area * coefficent)
}
