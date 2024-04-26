use std::f64::consts::PI;

pub trait HydraulicMath {
    fn flow_rate_through_orifice(
        discharge_coefficient: f64,
        area_of_orifice: f64,
        gravity_acceleration: f64,
        height_diff_pressure_drop: f64,
    ) -> f64;

    fn flow_rate_through_pipe(
        pipe_radius: f64,
        delta_pressure: f64,
        viscocity_of_fluid_pa: f64,
        length: f64,
    ) -> f64;

    fn pascals_law_p(force: f64, area: f64) -> f64;

    fn pascals_law_f(pressure_pa: f64, area: f64) -> f64;

    fn hydraulic_power(flow_rate: f64, delta_pressure: f64, efficency: f64) -> f64;

    fn density(mass: f64, volume: f64) -> f64;

    fn dynamic_viscocity(shear_stress: f64, shear_rate: f64) -> f64;
}

pub struct HydraulicCalculator {}

impl HydraulicMath for HydraulicCalculator {
    fn flow_rate_through_orifice(
        discharge_coefficent: f64,
        area_of_orfice: f64,
        gravity_acceleration: f64,
        height_dif_pressure_drop: f64,
    ) -> f64 {
        discharge_coefficent
            * area_of_orfice
            * (2. * gravity_acceleration * height_dif_pressure_drop).sqrt()
    }

    fn flow_rate_through_pipe(
        pipe_radius: f64,
        delta_pressure: f64,
        viscocity_of_fluid_pa: f64,
        length: f64,
    ) -> f64 {
        (PI * pipe_radius.powf(4.) * delta_pressure) / (8. * viscocity_of_fluid_pa * length)
    }

    fn pascals_law_p(force: f64, area: f64) -> f64 {
        force / area
    }

    fn pascals_law_f(pressure_pa: f64, area: f64) -> f64 {
        pressure_pa * area
    }

    fn hydraulic_power(flow_rate: f64, delta_pressure: f64, efficency: f64) -> f64 {
        flow_rate * delta_pressure * efficency
    }

    fn density(mass: f64, volume: f64) -> f64 {
        mass / volume
    }

    fn dynamic_viscocity(shear_stress: f64, shear_rate: f64) -> f64 {
        shear_stress / shear_rate
    }
}
