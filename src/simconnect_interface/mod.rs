// rip linux development with this implementation

use simconnect;
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

pub async fn sinconnect_interface(mutex_vars: MutexVariables) {
    let connection = simconnect::SimConnector::new();
    connection.connect("Ouroboros Simconnect");

    // data defs for all the simvars:
    connection.add_data_definition(
        0,
        "PLANE HEADING",
        "Degrees",
        simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64,
        u32::MAX,
        0.0,
    );

    loop {
        unimplemented!();
    }
    tokio::time::sleep(tokio::time::Duration::from_millis(16));
}
