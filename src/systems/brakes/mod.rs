use self::{
    actuator::Actuator, brake_assembly::BrakeAssembly, brake_location::BrakePosition,
    brake_materials::BrakeMaterials, pressure_plate::PressurePlate, stator::Stator,
};

pub mod actuator;
pub mod brake_assembly;
pub mod brake_location;
pub mod brake_materials;
pub mod friction;
pub mod pressure_plate;
pub mod stator;

#[derive(Debug)]
pub struct BrakeSystem {
    assemblies: Vec<BrakeAssembly>,
}

impl BrakeSystem {
    pub fn new() -> Self {
        Self {
            assemblies: Vec::new(),
        }
    }
    pub fn with_assembly(mut self, name: BrakePosition) -> Self {
        let new_assembly = BrakeAssembly::new(name.to_string());
        self.assemblies.push(new_assembly);
        self
    }
    pub fn with_pressure_plate(
        mut self,
        radius: f64,
        axle_radius: f64,
        thickness: f64,
        material: BrakeMaterials,
    ) -> Self {
        if let Some(parent_assembly) = self.assemblies.last_mut() {
            let pressure_plate = PressurePlate::new(radius, axle_radius, thickness, material);
            parent_assembly.with_pressure_plate(pressure_plate);
        }
        self
    }
    pub fn with_stator(
        mut self,
        radius: f64,
        axle_radius: f64,
        thickness: f64,
        material: BrakeMaterials,
    ) -> Self {
        if let Some(parent_assembly) = self.assemblies.last_mut() {
            let stator = Stator::new(radius, axle_radius, thickness, material);
            parent_assembly.with_stator(stator);
        }
        self
    }

    pub fn with_actuator(
        mut self,
        pistonCount: f64,
        pistonRadius: f64,
        pistonExtension: f64,
    ) -> Self {
        if let Some(parent_assembly) = self.assemblies.last_mut() {
            let actuator = Actuator::new(pistonCount, pistonRadius, pistonExtension);
            parent_assembly.with_actuator(actuator)
        }
        self
    }

    pub fn build(self) -> Self {
        self
    }
}
