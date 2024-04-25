#[derive(Debug)]
pub struct PressureSwitch {
    id: String,
}

impl PressureSwitch {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
