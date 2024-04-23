use core::fmt;
use std::collections::HashMap;

use super::{bus::Bus, transformer::Transformer};

#[derive(Debug)]
pub enum EControlCenters {
    LICC, // Left Integrated Control Center
    RICC, // Right Integrated Control Center
    EICC, // Emergency Integrated Control Center
    AICC, // A Integrated Control Center
}

impl fmt::Display for EControlCenters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EControlCenters::EICC => write!(f, "EICC"),
            EControlCenters::LICC => write!(f, "LICC"),
            EControlCenters::RICC => write!(f, "RICC"),
            EControlCenters::AICC => write!(f, "AICC"),
        }
    }
}

impl EControlCenters {
    pub fn get_full_name(&self) -> String {
        match self {
            EControlCenters::EICC => String::from("Emergency Integrated Control Center"),
            EControlCenters::LICC => String::from("Left Integrated Control Center"),
            EControlCenters::RICC => String::from("Right Integrated Control Center"),
            EControlCenters::AICC => String::from("A Integrated Control Center"),
        }
    }
}

#[derive(Debug)]
pub struct ControlCenters {
    busses: Vec<Bus>,
    connections: HashMap<String, String>,
    transformer: Vec<Transformer>,
}
