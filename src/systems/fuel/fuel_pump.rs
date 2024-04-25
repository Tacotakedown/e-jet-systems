#[derive(Debug)]
pub enum FuelPumpType {
    AcAuxBoostPump,
    DcApuEngineStartPump,
    EjectorPump,
}

#[derive(Debug)]
pub struct FuelPump {
    name: String,
    pump_type: FuelPumpType,
    connection: Vec<String>, // connect to fuel lines, string placeholder
    max_output: f64,
    electric_component_id: Option<String>,
}
impl FuelPump {
    pub fn new(
        name: String,
        pump_type: FuelPumpType,
        max_output: f64,
        electric_component_id: Option<String>,
    ) -> Self {
        Self {
            name,
            pump_type,
            connection: Vec::new(),
            max_output,
            electric_component_id,
        }
    }

    pub fn with_connections(&mut self, connections: Vec<String>) {
        self.connection = connections;
    }
}
