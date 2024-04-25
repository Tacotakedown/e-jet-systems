// anti slosh valve, prevents movement of fuel due to aircraft roll

#[derive(Debug)]
pub struct FlapValve {
    id: String,
}

impl FlapValve {
    pub fn new(id: String) -> Self {
        Self { id }
    }
}
