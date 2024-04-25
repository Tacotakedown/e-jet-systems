// this valve allows flow inboard but not outboard, located on the outboard side of the rib

#[derive(Debug)]
pub struct BaffleOvalCheckValve {
    id: String,
}

impl BaffleOvalCheckValve {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
