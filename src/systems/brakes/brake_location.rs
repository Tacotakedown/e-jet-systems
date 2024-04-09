use core::fmt;

#[derive(Debug, Clone, Copy)]
pub enum BrakePosition {
    LeftMain,
    RightMain,
}

impl fmt::Display for BrakePosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BrakePosition::LeftMain => write!(f, "LeftMain"),
            BrakePosition::RightMain => write!(f, "RightMain"),
        }
    }
}
