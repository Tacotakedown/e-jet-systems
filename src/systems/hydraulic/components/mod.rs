// Enums for deriving IDs for each component (easiest way to keep some sort of error checking using this method)

#[derive(Debug)]
pub enum E170HydraulicComponents {
    System1(E170System1),
    System2(E170System2),
    System3(E170System3),
}

#[derive(Debug)]
pub enum E170System1 {
    Reservoir,
    EngineDrivenPump,
    AcMotorPump,
    FilterManifold,
    ReturnPriorityValve,
    SystemAccumulator,
    ParkingBrakeAccumulator,
    LhThrustReverser,
    PTUValve,
    PTUPriorityValve,
    UpperRudder,
    OutboardBrakeSystem,
    LhOutboardElevator,
    LhRhMFS3,
    LhRhMFS4,
    LhRhGS2,
    ParkingBrake,
    PTU,
}

#[derive(Debug)]
pub enum E170System2 {
    Reservoir,
    EngineDrivenPump,
    AcMotorPump,
    FilterManifold,
    ReturnPriorityValve,
    SystemAccumulator,
    RhThrustReverser,
    LhRhMFS5,
    LhRhGS1,
    LhInboardAileron,
    RhInboardAileron,
    LhInboardElevator,
    RhInboardElevator,
    InboardBrakeSystem,
    ParkingBrakeAccumulator,
    ParkingBrake,
    GearPriorityValve,
    LandingGearManifold,
    NoseLandingGear,
    MainLandingGear,
    NoseWheelSteering, // only available while extended (duh)
}

#[derive(Debug)]
pub enum E170System3 {
    Reservoir,
    AcMotorPump1,
    AcMotorPump2,
    PumpUnloadValve,
    FilterManifold,
    FlowLimitValve,
    ReturnPriorityValve,
    SystemAccumulator,
    LhOutboardAileron,
    RhOutboardAileron,
    LowerRudder,
    RhOutboardElevator,
}

impl E170HydraulicComponents {
    pub fn id(&self) -> String {
        match self {
            E170HydraulicComponents::System1(component) => match component {
                E170System1::Reservoir => "Sys1Reservoir".to_string(),
                E170System1::EngineDrivenPump => "Sys1EngineDrivenPump".to_string(),
                E170System1::AcMotorPump => "Sys1AcMotorPump".to_string(),
                E170System1::FilterManifold => "Sys1FilterManifold".to_string(),
                E170System1::ReturnPriorityValve => "Sys1ReturnPriorityValve".to_string(),
                E170System1::SystemAccumulator => "Sys1SystemAccumulator".to_string(),
                E170System1::ParkingBrakeAccumulator => "Sys1ParkingBrakeAccumulator".to_string(),
                E170System1::LhThrustReverser => "Sys1LhThrustReverser".to_string(),
                E170System1::PTUValve => "Sys1PTUValve".to_string(),
                E170System1::PTUPriorityValve => "Sys1PTUPriorityValve".to_string(),
                E170System1::UpperRudder => "Sys1UpperRudder".to_string(),
                E170System1::OutboardBrakeSystem => "Sys1OutboardBrakeSystem".to_string(),
                E170System1::LhOutboardElevator => "Sys1LhOutboardElevator".to_string(),
                E170System1::LhRhMFS3 => "Sys1LhRhMFS3".to_string(),
                E170System1::LhRhMFS4 => "Sys1LhRhMFS4".to_string(),
                E170System1::LhRhGS2 => "Sys1LhRhGS2".to_string(),
                E170System1::ParkingBrake => "Sys1ParkingBrake".to_string(),
                E170System1::PTU => "Sys1PTU".to_string(),
            },
            E170HydraulicComponents::System2(component) => match component {
                E170System2::Reservoir => "Sys2Reservoir".to_string(),
                E170System2::EngineDrivenPump => "Sys2EngineDrivenPump".to_string(),
                E170System2::AcMotorPump => "Sys2AcMotorPump".to_string(),
                E170System2::FilterManifold => "Sys2FilterManifold".to_string(),
                E170System2::ReturnPriorityValve => "Sys2ReturnPriorityValve".to_string(),
                E170System2::SystemAccumulator => "Sys2SystemAccumulator".to_string(),
                E170System2::RhThrustReverser => "Sys2RhThrustReverser".to_string(),
                E170System2::LhRhMFS5 => "Sys2LhRhMFS5".to_string(),
                E170System2::LhRhGS1 => "Sys2LhRhGS1".to_string(),
                E170System2::LhInboardAileron => "Sys2LhInboardAileron".to_string(),
                E170System2::RhInboardAileron => "Sys2RhInboardAileron".to_string(),
                E170System2::LhInboardElevator => "Sys2LhInboardElevator".to_string(),
                E170System2::RhInboardElevator => "Sys2RhInboardElevator".to_string(),
                E170System2::InboardBrakeSystem => "Sys2InboardBrakeSystem".to_string(),
                E170System2::ParkingBrakeAccumulator => "Sys2ParkingBrakeAccumulator".to_string(),
                E170System2::ParkingBrake => "Sys2ParkingBrake".to_string(),
                E170System2::GearPriorityValve => "Sys2GearPriorityValve".to_string(),
                E170System2::LandingGearManifold => "Sys2LandingGearManifold".to_string(),
                E170System2::NoseLandingGear => "Sys2NoseLandingGear".to_string(),
                E170System2::MainLandingGear => "Sys2MainLandingGear".to_string(),
                E170System2::NoseWheelSteering => "Sys2NoseWheelSteering".to_string(),
            },
            E170HydraulicComponents::System3(component) => match component {
                E170System3::Reservoir => "Sys3Reservoir".to_string(),
                E170System3::AcMotorPump1 => "Sys3AcMotorPump1".to_string(),
                E170System3::AcMotorPump2 => "Sys3AcMotorPump2".to_string(),
                E170System3::PumpUnloadValve => "Sys3PumpUnloadValve".to_string(),
                E170System3::FilterManifold => "Sys3FilterManifold".to_string(),
                E170System3::FlowLimitValve => "Sys3FlowLimitValve".to_string(),
                E170System3::ReturnPriorityValve => "Sys3ReturnPriorityValve".to_string(),
                E170System3::SystemAccumulator => "Sys3SystemAccumulator".to_string(),
                E170System3::LhOutboardAileron => "Sys3LhOutboardAileron".to_string(),
                E170System3::RhOutboardAileron => "Sys3RhOutboardAileron".to_string(),
                E170System3::LowerRudder => "Sys3LowerRudder".to_string(),
                E170System3::RhOutboardElevator => "Sys3RhOutboardElevator".to_string(),
            },
        }
    }
}
