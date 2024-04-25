#[derive(Debug)]
pub struct ShutoffValve {
    id: String,
}

impl ShutoffValve {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
