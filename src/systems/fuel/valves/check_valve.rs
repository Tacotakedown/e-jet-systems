#[derive(Debug)]
pub struct CheckValve {
    id: String,
}

impl CheckValve {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
