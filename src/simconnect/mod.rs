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

    fn write_simconnect(vars: Vec<(String, String, f64)>) {
        for (var, unit, value) in vars {
            // Simconnect.write(var, unit, value);
        }
    }
    pub async fn update(self, vars: Vec<(String, String)>, values: MutexVariables) {
        loop {
            // TODO: lock mutex values and build a Vec with proper name, unit and value for writing
            let vars = vars.clone();
            // Self::write_simconnect(vars);

            //TODO: read from a list of simvars that are expected to change within the sim, these should all be user input vars like switches because all data is shared inside the simulation already
            tokio::time::sleep(tokio::time::Duration::from_millis(13));
        }
    }
}
