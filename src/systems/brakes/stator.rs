use std::f64::consts::PI;

use super::brake_materials::BrakeMaterials;

#[derive(Debug, Clone)]
pub struct Stator {
    radius: f64,      // CM
    axle_radius: f64, // inner radius CM
    thickness: f64,   // CM
    material: BrakeMaterials,
}

impl Stator {
    pub fn new(radius: f64, axle_radius: f64, thickness: f64, material: BrakeMaterials) -> Self {
        Self {
            radius,
            axle_radius,
            thickness,
            material,
        }
    }

    pub fn get_material_friction(&self) -> f64 {
        self.material.get_coef_of_friction()
    }

    pub fn get_area(&self) -> f64 {
        let total_area = PI * self.radius.powi(2);
        let unused_area = PI * self.axle_radius.powi(2);
        total_area - unused_area
    }

    pub fn get_volume(&self) -> f64 {
        let total_area = PI * self.radius.powi(2);
        let unused_area = PI * self.axle_radius.powi(2);
        (total_area - unused_area) * self.thickness
    }
}
