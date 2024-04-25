#[derive(Debug)]
pub struct FlameArrestorSurgeValve {
    id: String,
}

impl FlameArrestorSurgeValve {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
