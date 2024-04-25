#[derive(Debug)]
pub struct DrainValve {
    id: String,
}

impl DrainValve {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
