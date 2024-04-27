use super::math::{HydraulicCalculator, HydraulicMath};

#[derive(Debug)]
pub struct FolderManifold {
    resistance: f64,
    filter_size_microns: f64,
}
impl FolderManifold {
    pub fn new(filter_size_microns: f64) -> Self {
        Self {
            resistance: 0.,
            filter_size_microns,
        }
    }
    pub fn calculate_pressure_drop(flow_rate_l_min: f64, dynamic_viscocity: f64) -> f64 {
        let permability: f64 = 1.5e-12;
        let cross_sectional_area: f64 = 0.0001;
        let length_filter: f64 = 1.0;
        HydraulicCalculator::darcy_law(
            flow_rate_l_min,
            permability,
            cross_sectional_area,
            dynamic_viscocity,
            length_filter,
        )
    }
}
