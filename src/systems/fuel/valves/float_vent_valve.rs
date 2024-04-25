#[derive(Debug)]
pub struct FloatVentValve {
    id: String,
}

impl FloatVentValve {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
