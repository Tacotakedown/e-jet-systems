// this component will need interconnectivity with hydraulic system, we will pass an arc Mutex to share data between systems

#[derive(Debug, Clone)]
pub struct Actuator {
    piston_count: f64,
    piston_radius: f64,
    piston_extension: f64,
    // hydraulic arc mutex
}

impl Actuator {
    pub fn new(piston_count: f64, piston_radius: f64, piston_extension: f64) -> Self {
        Self {
            piston_count,
            piston_radius,
            piston_extension,
        }
    }
}
