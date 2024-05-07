use once_cell::sync::Lazy;
use std::time::Duration;
use std::time::Instant;

use crate::mutex::MutexVariables;

use self::{
    brakes::{brake_location::BrakePosition, brake_materials::BrakeMaterials, BrakeSystem},
    electric::{busses::Busses, current_type::CurrentType, ElectricalSystem},
    flight_controls::{spoilers::spoiler_logic::SpoilerController, FlightControls},
    hydraulic::{
        components::{E170HydraulicComponents, E170System1, E170System2, E170System3},
        hydraulic_line::HydraulicLineMaterial,
        ComponentType, FlowDirection, HydraulicSystem,
    },
};
use tokio::time::sleep;

pub mod air_conditioning;
pub mod brakes;
pub mod electric;
pub mod flight_controls;
pub mod fuel;
pub mod hydraulic;
pub mod pneumatic;
pub mod reverse_thrust;
pub mod shared;

const TICK_SLEEP_DURATION: Duration = Duration::from_millis(16);

pub async fn electrical() -> ElectricalSystem {
    let electrical_system = ElectricalSystem::new()
        .with_bus(Busses::AcBus1, CurrentType::AC, 120.0)
        .with_component(
            "Component 1".to_string(),
            "L:COMPONENT_1_VOLTAGE".to_string(),
            100.0,
            120.0,
        )
        .with_component(
            "Component 2".to_string(),
            "L:COMPONENT_2_VOLTAGE".to_string(),
            100.0,
            120.0,
        )
        .with_component_switch(
            "Component 1".to_string(),
            "Component 1 Switch".to_string(),
            true,
        )
        .with_battery(
            "BATT1".to_string(),
            30.,
            (0.0, 28.0),
            6000.0,
            vec![Busses::HotBatBus1, Busses::HotBatBus2],
            (-10.0, 10.0),
        )
        .with_generator(
            "ENG1".to_string(),
            120.0,
            5000.0,
            vec![Busses::AcBus1, Busses::AcBus2],
        )
        .build();

    loop {
        // TODO: call the implemented electrical_system.calculate() here and feed it the proper mutex vars
        electrical_system.calculate();
        // println!("electrical system: {:?}", electrical_system);

        sleep(TICK_SLEEP_DURATION).await;
    }
}

pub async fn brake_system(mutex_vars: MutexVariables) -> BrakeSystem {
    let brake_system = BrakeSystem::new()
        .with_assembly(BrakePosition::LeftMainOB)
        .with_pressure_plate(30.0, 5.0, 5.0, BrakeMaterials::Iron)
        .with_stator(30.0, 5.0, 5.0, BrakeMaterials::CarbonCeramic)
        .with_pressure_plate(30.0, 5.0, 5.0, BrakeMaterials::Iron)
        .with_stator(30.0, 5.0, 5.0, BrakeMaterials::CarbonCeramic)
        .with_pressure_plate(30.0, 5.0, 5.0, BrakeMaterials::Iron)
        .with_stator(30.0, 5.0, 5.0, BrakeMaterials::CarbonCeramic)
        .with_actuator(6.0, 5.0, 3.0)
        .with_assembly(BrakePosition::LeftMainIB)
        .with_pressure_plate(30.0, 5.0, 5.0, BrakeMaterials::Iron)
        .with_stator(30.0, 5.0, 5.0, BrakeMaterials::CarbonCeramic)
        .with_pressure_plate(30.0, 5.0, 5.0, BrakeMaterials::Iron)
        .with_stator(30.0, 5.0, 5.0, BrakeMaterials::CarbonCeramic)
        .with_pressure_plate(30.0, 5.0, 5.0, BrakeMaterials::Iron)
        .with_stator(30.0, 5.0, 5.0, BrakeMaterials::CarbonCeramic)
        .with_actuator(6.0, 5.0, 3.0)
        .with_assembly(BrakePosition::RightMainOB)
        .with_pressure_plate(30.0, 5.0, 5.0, BrakeMaterials::Iron)
        .with_stator(30.0, 5.0, 5.0, BrakeMaterials::CarbonCeramic)
        .with_pressure_plate(30.0, 5.0, 5.0, BrakeMaterials::Iron)
        .with_stator(30.0, 5.0, 5.0, BrakeMaterials::CarbonCeramic)
        .with_pressure_plate(30.0, 5.0, 5.0, BrakeMaterials::Iron)
        .with_stator(30.0, 5.0, 5.0, BrakeMaterials::CarbonCeramic)
        .with_actuator(6.0, 5.0, 3.0)
        .with_assembly(BrakePosition::RightMainIB)
        .with_pressure_plate(30.0, 5.0, 5.0, BrakeMaterials::Iron)
        .with_stator(30.0, 5.0, 5.0, BrakeMaterials::CarbonCeramic)
        .with_pressure_plate(30.0, 5.0, 5.0, BrakeMaterials::Iron)
        .with_stator(30.0, 5.0, 5.0, BrakeMaterials::CarbonCeramic)
        .with_pressure_plate(30.0, 5.0, 5.0, BrakeMaterials::Iron)
        .with_stator(30.0, 5.0, 5.0, BrakeMaterials::CarbonCeramic)
        .with_actuator(6.0, 5.0, 3.0)
        .build();

    // system 2 controls inboard brake system
    // system 1 controls outboard brake system

    loop {
        // TODO: call the implemented brake_system.calculate() here and feed it the proper mutex vars
        brake_system
            .clone()
            .calculate(mutex_vars.clone(), 0.0, 0.0, 0.0);
        // println!("brake: {:?}", brake_system);

        sleep(TICK_SLEEP_DURATION).await;
    }
}

pub async fn hydraulic_system(mutex_vars: MutexVariables) {
    let mut hydraulic = HydraulicSystem::new();

    // building system 1 components
    hydraulic.add_component(
        E170HydraulicComponents::System1(E170System1::Reservoir).id(),
        ComponentType::Reservoir,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System1(E170System1::AcMotorPump).id(),
        ComponentType::ElecPump,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System1(E170System1::EngineDrivenPump).id(),
        ComponentType::Pump,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System1(E170System1::FilterManifold).id(),
        ComponentType::FilterManifold,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System1(E170System1::ReturnPriorityValve).id(),
        ComponentType::PriorityValve,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System1(E170System1::SystemAccumulator).id(),
        ComponentType::Accumulator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System1(E170System1::LhThrustReverser).id(),
        ComponentType::Actuator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System1(E170System1::LhRhGS2).id(),
        ComponentType::Actuator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System1(E170System1::LhRhMFS3).id(),
        ComponentType::Actuator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System1(E170System1::LhRhMFS4).id(),
        ComponentType::Actuator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System1(E170System1::LhOutboardElevator).id(),
        ComponentType::Actuator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System1(E170System1::OutboardBrakeSystem).id(),
        ComponentType::Actuator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System1(E170System1::UpperRudder).id(),
        ComponentType::Actuator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System1(E170System1::ParkingBrakeAccumulator).id(),
        ComponentType::Accumulator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System1(E170System1::ParkingBrake).id(),
        ComponentType::Actuator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System1(E170System1::PTUValve).id(),
        ComponentType::PTUSelecorValve,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System1(E170System1::PTU).id(),
        ComponentType::PTU,
    );

    // connecting system 1 components in proper order
    //TODO: Find reasonable lengths for hydraulic connections
    hydraulic.add_connection(
        E170HydraulicComponents::System1(E170System1::Reservoir).id(),
        E170HydraulicComponents::System1(E170System1::EngineDrivenPump).id(),
        FlowDirection::InletToOutlet,
        1.0,
        HydraulicLineMaterial::Solid,
    );
    hydraulic.add_connection(
        E170HydraulicComponents::System1(E170System1::Reservoir).id(),
        E170HydraulicComponents::System1(E170System1::AcMotorPump).id(),
        FlowDirection::InletToOutlet,
        1.0,
        HydraulicLineMaterial::Solid,
    );

    hydraulic.add_connection(
        E170HydraulicComponents::System1(E170System1::EngineDrivenPump).id(),
        E170HydraulicComponents::System1(E170System1::FilterManifold).id(),
        FlowDirection::InletToOutlet,
        1.0,
        HydraulicLineMaterial::Solid,
    );
    hydraulic.add_connection(
        E170HydraulicComponents::System1(E170System1::AcMotorPump).id(),
        E170HydraulicComponents::System1(E170System1::FilterManifold).id(),
        FlowDirection::InletToOutlet,
        1.0,
        HydraulicLineMaterial::Solid,
    );

    hydraulic.add_connection(
        E170HydraulicComponents::System1(E170System1::FilterManifold).id(),
        E170HydraulicComponents::System1(E170System1::ReturnPriorityValve).id(),
        FlowDirection::InletToOutlet,
        1.0,
        HydraulicLineMaterial::Solid,
    );

    hydraulic.add_connection(
        E170HydraulicComponents::System1(E170System1::ReturnPriorityValve).id(),
        E170HydraulicComponents::System1(E170System1::SystemAccumulator).id(),
        FlowDirection::InletToOutlet,
        1.0,
        HydraulicLineMaterial::Solid,
    );

    hydraulic.add_connection(
        E170HydraulicComponents::System1(E170System1::SystemAccumulator).id(),
        E170HydraulicComponents::System1(E170System1::Reservoir).id(),
        FlowDirection::InletToOutlet,
        1.0,
        HydraulicLineMaterial::Solid,
    );

    hydraulic.add_connection(
        E170HydraulicComponents::System1(E170System1::FilterManifold).id(),
        E170HydraulicComponents::System1(E170System1::LhThrustReverser).id(),
        FlowDirection::InletToOutlet,
        1.0,
        HydraulicLineMaterial::Solid,
    );

    hydraulic.add_connection(
        E170HydraulicComponents::System1(E170System1::FilterManifold).id(),
        E170HydraulicComponents::System1(E170System1::LhRhGS2).id(),
        FlowDirection::InletToOutlet,
        1.0,
        HydraulicLineMaterial::Solid,
    );

    hydraulic.add_connection(
        E170HydraulicComponents::System1(E170System1::FilterManifold).id(),
        E170HydraulicComponents::System1(E170System1::LhRhMFS3).id(),
        FlowDirection::InletToOutlet,
        1.0,
        HydraulicLineMaterial::Solid,
    );

    hydraulic.add_connection(
        E170HydraulicComponents::System1(E170System1::FilterManifold).id(),
        E170HydraulicComponents::System1(E170System1::LhRhMFS4).id(),
        FlowDirection::InletToOutlet,
        1.0,
        HydraulicLineMaterial::Solid,
    );

    hydraulic.add_connection(
        E170HydraulicComponents::System1(E170System1::FilterManifold).id(),
        E170HydraulicComponents::System1(E170System1::LhOutboardElevator).id(),
        FlowDirection::InletToOutlet,
        1.0,
        HydraulicLineMaterial::Solid,
    );

    hydraulic.add_connection(
        E170HydraulicComponents::System1(E170System1::FilterManifold).id(),
        E170HydraulicComponents::System1(E170System1::OutboardBrakeSystem).id(),
        FlowDirection::InletToOutlet,
        1.0,
        HydraulicLineMaterial::Solid,
    );

    hydraulic.add_connection(
        E170HydraulicComponents::System1(E170System1::FilterManifold).id(),
        E170HydraulicComponents::System1(E170System1::UpperRudder).id(),
        FlowDirection::InletToOutlet,
        1.0,
        HydraulicLineMaterial::Solid,
    );

    hydraulic.add_connection(
        E170HydraulicComponents::System1(E170System1::FilterManifold).id(),
        E170HydraulicComponents::System1(E170System1::ParkingBrakeAccumulator).id(),
        FlowDirection::InletToOutlet,
        1.0,
        HydraulicLineMaterial::Solid,
    );

    hydraulic.add_connection(
        E170HydraulicComponents::System1(E170System1::FilterManifold).id(),
        E170HydraulicComponents::System1(E170System1::PTUValve).id(),
        FlowDirection::InletToOutlet,
        1.0,
        HydraulicLineMaterial::Solid,
    );
    hydraulic.add_connection(
        E170HydraulicComponents::System1(E170System1::PTUValve).id(),
        E170HydraulicComponents::System1(E170System1::PTUPriorityValve).id(),
        FlowDirection::InletToOutlet,
        1.0,
        HydraulicLineMaterial::Solid,
    );
    hydraulic.add_connection(
        E170HydraulicComponents::System1(E170System1::PTUPriorityValve).id(),
        E170HydraulicComponents::System1(E170System1::PTU).id(),
        FlowDirection::InletToOutlet,
        1.0,
        HydraulicLineMaterial::Solid,
    );

    // building system 2 components
    hydraulic.add_component(
        E170HydraulicComponents::System2(E170System2::Reservoir).id(),
        ComponentType::Reservoir,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System2(E170System2::EngineDrivenPump).id(),
        ComponentType::Pump,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System2(E170System2::AcMotorPump).id(),
        ComponentType::ElecPump,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System2(E170System2::FilterManifold).id(),
        ComponentType::FilterManifold,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System2(E170System2::ReturnPriorityValve).id(),
        ComponentType::PriorityValve,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System2(E170System2::SystemAccumulator).id(),
        ComponentType::Accumulator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System2(E170System2::RhThrustReverser).id(),
        ComponentType::Actuator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System2(E170System2::LhRhMFS5).id(),
        ComponentType::Actuator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System2(E170System2::LhRhGS1).id(),
        ComponentType::Actuator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System2(E170System2::LhInboardAileron).id(),
        ComponentType::Actuator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System2(E170System2::RhInboardAileron).id(),
        ComponentType::Actuator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System2(E170System2::LhInboardElevator).id(),
        ComponentType::Actuator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System2(E170System2::RhInboardElevator).id(),
        ComponentType::Actuator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System2(E170System2::InboardBrakeSystem).id(),
        ComponentType::Actuator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System2(E170System2::ParkingBrakeAccumulator).id(),
        ComponentType::Accumulator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System2(E170System2::ParkingBrake).id(),
        ComponentType::Actuator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System2(E170System2::GearPriorityValve).id(),
        ComponentType::PriorityValve,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System2(E170System2::LandingGearManifold).id(),
        ComponentType::GearManifold,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System2(E170System2::NoseLandingGear).id(),
        ComponentType::Actuator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System2(E170System2::MainLandingGear).id(),
        ComponentType::Actuator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System2(E170System2::NoseWheelSteering).id(),
        ComponentType::Actuator,
    );

    //connecting system 2 components in proper order

    // building system 3 components
    hydraulic.add_component(
        E170HydraulicComponents::System3(E170System3::Reservoir).id(),
        ComponentType::Reservoir,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System3(E170System3::AcMotorPump1).id(),
        ComponentType::ElecPump,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System3(E170System3::AcMotorPump2).id(),
        ComponentType::ElecPump,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System3(E170System3::PumpUnloadValve).id(),
        ComponentType::UnloaderValve,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System3(E170System3::FilterManifold).id(),
        ComponentType::FilterManifold,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System3(E170System3::FlowLimitValve).id(),
        ComponentType::FlowLimitValve,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System3(E170System3::ReturnPriorityValve).id(),
        ComponentType::PriorityValve,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System3(E170System3::SystemAccumulator).id(),
        ComponentType::Accumulator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System3(E170System3::LhOutboardAileron).id(),
        ComponentType::Actuator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System3(E170System3::RhOutboardAileron).id(),
        ComponentType::Actuator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System3(E170System3::LowerRudder).id(),
        ComponentType::Actuator,
    );
    hydraulic.add_component(
        E170HydraulicComponents::System3(E170System3::RhOutboardElevator).id(),
        ComponentType::Actuator,
    );

    //connecting system 3 components in proper order

    // system 1 and 2 connections (PTU)

    loop {
        hydraulic.simulate(mutex_vars.clone()).await;
        sleep(TICK_SLEEP_DURATION).await;
    }
}

pub async fn flight_controls(mutex_vars: MutexVariables) {
    let mut flt_controls = FlightControls::new();
    flt_controls.with_spoilers();
    let simulator_vars_building_lifetime = mutex_vars.read_simulator_variables().await;
    static mut ON_GROUND: bool = false;
    static mut LANDING_CONFIG: bool = false;
    let mut spoiler_controller = SpoilerController::new(
        simulator_vars_building_lifetime.aileron_controls_position,
        unsafe { ON_GROUND },
        simulator_vars_building_lifetime.spoiler_handle_0_to_100,
        unsafe { LANDING_CONFIG },
    );

    static mut LAST_TIME: Lazy<Instant> = Lazy::new(|| Instant::now());
    loop {
        let current_time = Instant::now();
        let dt = unsafe { current_time.duration_since(unsafe { *LAST_TIME }) };
        let mut simulator_vars = mutex_vars.read_simulator_variables().await;
        let mut hydraulic_vars = mutex_vars.read_hydraulic_vars().await;
        flt_controls.update(
            unsafe { ON_GROUND },
            simulator_vars.aileron_controls_position,
            hydraulic_vars.system1.post_maifold_pressure,
            simulator_vars.spoiler_handle_0_to_100,
            unsafe { LANDING_CONFIG },
            &mut spoiler_controller,
            dt,
        );

        mutex_vars.write_simulator_variables(simulator_vars);

        unsafe { *LAST_TIME = current_time };
    }
}
