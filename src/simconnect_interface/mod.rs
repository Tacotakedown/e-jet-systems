pub mod simvars;

#[cfg(target_os = "windows")]
pub mod simconnect_interface_mod {
    use simconnect;
    use std::env::Vars;

    use crate::mutex::{self, MutexVariables};

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

    pub async fn simconnect_thread_fn(mutex_vars: MutexVariables) {
        let mut connection = simconnect::SimConnector::new();
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
        let mut value: u32 = 42; // Example value
        let error: *mut u32 = &mut value as *mut u32;
        loop {
            // todo!();

            std::thread::sleep(tokio::time::Duration::from_millis(13));
        }
    }
}

// No-op the entire file on Linux
#[cfg(not(target_os = "windows"))]
pub mod simconnect_interface_mod {
    use crate::MutexVariables;

    pub async fn simconnect_thread_fn(_mutex_vars: MutexVariables) {
        // No-op
    }
}
