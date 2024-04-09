// this component will need interconnectivity with hydraulic system, we will pass an arc Mutex to share data between systems

#[derive(Debug, Clone)]
pub struct Actuator {
    pistonCount: f64,
    pistonRadius: f64,
    pistonExtension: f64,
    // hydraulic arc mutex
}

impl Actuator {
    pub fn new(pistonCount: f64, pistonRadius: f64, pistonExtension: f64) -> Self {
        Self {
            pistonCount,
            pistonRadius,
            pistonExtension,
        }
    }
}
