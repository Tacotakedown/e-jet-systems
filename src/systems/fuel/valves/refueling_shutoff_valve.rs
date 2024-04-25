#[derive(Debug)]
pub struct RefuelingShutoffValve {
    id: String,
}

impl RefuelingShutoffValve {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
