use super::spoiler_position::SpoilerPosition;

#[derive(Debug)]
pub struct Spoiler {
    pcu_input: f64,
    spoiler_position: SpoilerPosition, //0 - 1
}

impl Spoiler {
    pub fn new(pcu_input: f64, spoiler_position: SpoilerPosition) -> Self {
        Self {
            pcu_input,
            spoiler_position,
        }
    }
}
