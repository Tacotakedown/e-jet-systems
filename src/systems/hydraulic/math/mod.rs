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

    fn darcy_law(
        flow_rate_l_min: f64,
        permability: f64,
        cross_sectional_area: f64,
        dynamic_viscocity: f64,
        length_of_filter: f64,
    ) -> f64;

    fn pressure_from_temperature(density: f64, temperature: f64, molar_mass: f64) -> f64;

    fn pressrue_pa_estimated(
        density: f64,
        volume: f64,
        cross_sectional_areas: Vec<f64>,
        flow_rate: f64,
    ) -> f64;

    fn pressure_pa_increase(
        pressure_pa: f64,
        density: f64,
        volume: f64,
        cross_sectional_areas: Vec<f64>,
        flow_rate: f64,
    ) -> f64;
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
    fn darcy_law(
        flow_rate_l_min: f64,
        permability: f64,
        cross_sectional_area: f64,
        dynamic_viscocity: f64,
        length_of_filter: f64,
    ) -> f64 {
        let flow_rate_l_sec = flow_rate_l_min / 60.;
        (flow_rate_l_sec * dynamic_viscocity * length_of_filter)
            / (permability * cross_sectional_area)
    }
    fn pressure_from_temperature(density: f64, temperature: f64, molar_mass: f64) -> f64 {
        (density * 8.314 * temperature) / molar_mass
    }

    fn pressrue_pa_estimated(
        density: f64,
        volume: f64,
        cross_sectional_areas: Vec<f64>,
        flow_rate: f64,
    ) -> f64 {
        let mut net_cross_sectional_area: f64 = 0.;
        for val in cross_sectional_areas {
            net_cross_sectional_area += val
        }
        0.5 * density * (flow_rate / net_cross_sectional_area).powf(2.)
    }

    fn pressure_pa_increase(
        pressure_pa: f64,
        density: f64,
        volume: f64,
        cross_sectional_areas: Vec<f64>,
        flow_rate: f64,
    ) -> f64 {
        let mut net_cross_sectional_area: f64 = 0.;
        for val in cross_sectional_areas {
            net_cross_sectional_area += val
        }
        pressure_pa + 0.5 * density * (flow_rate / net_cross_sectional_area).powf(2.)
    }
}
