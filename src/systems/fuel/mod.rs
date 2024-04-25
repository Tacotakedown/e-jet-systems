use self::{fuel_filters::FuelFilter, fuel_pump::FuelPump};

pub mod fuel_filters;
pub mod fuel_pump;
pub mod fuel_tank;
pub mod pressure_switch;
pub mod valves;
pub mod vents;

#[derive(Debug)]
pub struct FuelSystem {
    pumps: Vec<FuelPump>,
    filters: Vec<FuelFilter>,
}
