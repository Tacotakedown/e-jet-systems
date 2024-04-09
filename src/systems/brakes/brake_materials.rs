// materials that are used acoss stators and pressure plates

#[derive(Debug, Copy, Clone)]
pub enum BrakeMaterials {
    CarbonCeramic,
    Iron,
    // idk what these shits are made of
}

impl BrakeMaterials {
    pub fn get_coef_of_friction(&self) -> f64 {
        match *self {
            BrakeMaterials::CarbonCeramic => 0.4, // change this once we can have some sort of idea idfk
            BrakeMaterials::Iron => 0.3,
        }
    }
}
