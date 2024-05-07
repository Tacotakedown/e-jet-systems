use core::fmt;

#[derive(Debug, Clone, Copy)]
pub enum BrakePosition {
    LeftMainOB,
    LeftMainIB,
    RightMainOB,
    RightMainIB,
}

impl fmt::Display for BrakePosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BrakePosition::LeftMainOB => write!(f, "LeftMainOB"),
            BrakePosition::RightMainOB => write!(f, "RightMainOB"),
            BrakePosition::LeftMainIB => write!(f, "LeftMainIB"),
            BrakePosition::RightMainIB => write!(f, "RightMainIB"),
        }
    }
}
