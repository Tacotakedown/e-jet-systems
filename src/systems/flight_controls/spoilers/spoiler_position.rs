use core::fmt;

#[derive(Debug)]
pub enum ESpoilerPosition {
    R1,
    R2,
    R3,
    R4,
    R5,
    L1,
    L2,
    L3,
    L4,
    L5,
}

impl fmt::Display for ESpoilerPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::L1 => write!(f, "L1"),
            Self::L2 => write!(f, "L2"),
            Self::L3 => write!(f, "L3"),
            Self::L4 => write!(f, "L4"),
            Self::L5 => write!(f, "L5"),
            Self::R1 => write!(f, "R1"),
            Self::R2 => write!(f, "R2"),
            Self::R3 => write!(f, "R3"),
            Self::R4 => write!(f, "R4"),
            Self::R5 => write!(f, "R5"),
        }
    }
}
impl ESpoilerPosition {
    pub fn key(&self) -> String {
        match self {
            Self::R1 => String::from("R1"),
            Self::R2 => String::from("R2"),
            Self::R3 => String::from("R3"),
            Self::R4 => String::from("R4"),
            Self::R5 => String::from("R5"),
            Self::L1 => String::from("L1"),
            Self::L2 => String::from("L2"),
            Self::L3 => String::from("L3"),
            Self::L4 => String::from("L4"),
            Self::L5 => String::from("L5"),
        }
    }
}
