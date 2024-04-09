use self::{fuel_filters::FuelFilter, fuel_pump::FuelPump};

pub mod fuel_filters;
pub mod fuel_flow_sensor;
pub mod fuel_junction;
pub mod fuel_pump;
pub mod fuel_tank;
pub mod fuel_valve;
pub mod fuel_vent;

#[derive(Debug)]
pub struct FuelSystem {
    pumps: Vec<FuelPump>,
    filters: Vec<FuelFilter>,
}
