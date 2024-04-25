use std::env::Vars;

use crate::mutex::{self, MutexVariables};

pub mod simvars;

#[derive(Debug)]
pub struct Simconnect {
    id: String,
}

impl Simconnect {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    fn write_simconnect(vars: Vec<(String, f64, String)>) {
        for (var, unit, value) in vars {
            // Simconnect.write(var, unit, value);
        }
    }
    pub async fn update(&self, values: MutexVariables) {
        loop {
            let parsed: Vec<(String, f64, String)> = mutex::get_values(values.clone()).await;
            // writes all the mutex vars to simconnect every tick
            Self::write_simconnect(parsed);

            //TODO: read from a list of simvars that are expected to change within the sim, these should all be user input vars like switches because all data is shared inside the simulation already
            tokio::time::sleep(tokio::time::Duration::from_millis(13)).await;
        }
    }
}
