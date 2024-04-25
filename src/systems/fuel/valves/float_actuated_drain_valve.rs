#[derive(Debug)]
pub struct FloatActuatedDrainValve {
    id: String,
}

impl FloatActuatedDrainValve {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
