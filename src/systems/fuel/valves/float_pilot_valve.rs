#[derive(Debug)]
pub struct FloatPilotValve {
    id: String,
}

impl FloatPilotValve {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
