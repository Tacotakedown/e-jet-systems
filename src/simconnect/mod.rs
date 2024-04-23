use std::env::Vars;

use crate::MutexVariables;

pub mod simvars;

#[derive(Debug)]
pub struct Simconnect {
    id: String,
}

impl Simconnect {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    fn write_simconnect(vars: MutexVariables) {
        let simvars = vec!["L:", "L:"];
        // match mutex variables to simvar

        for var in vars {
            // simconnect.write()
        }
    }
    pub fn update() {
        loop {
            Self::write_simconnect();
            tokio::time::sleep(tokio::time::Duration::from_millis(13));
        }
    }
}
