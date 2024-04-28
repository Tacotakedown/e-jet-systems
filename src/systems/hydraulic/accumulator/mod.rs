#[derive(Debug)]
pub struct Accumulator {
    nitrogen_pressure_kpa: f64,
    max_capacity: f64,
    current_capacity: f64,
}
impl Accumulator {
    pub fn new(nitrogen_pressure_kpa: f64, max_capacity: f64, current_capacity: f64) -> Self {
        Self {
            nitrogen_pressure_kpa,
            max_capacity,
            current_capacity,
        }
    }
    pub fn get_current_capacity(&self) -> f64 {
        self.current_capacity
    }
    // the way the accumulator will work will be, if there is no pressure in the system from the pumps, the accumulators will displace the proper amount of fluid until system pressure = nitrogen pressure until its empty, then when the sytem is on, we will fill the accumulator (increasing the pressure of the nitrogen because its compressed)
}
