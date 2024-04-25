use std::time::Duration;

use self::{
    brakes::{brake_location::BrakePosition, brake_materials::BrakeMaterials, BrakeSystem},
    electric::{busses::Busses, current_type::CurrentType, ElectricalSystem},
    hydraulic::{FlowDirection, HydraulicSystem},
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

pub async fn brake_system() -> BrakeSystem {
    let brake_system = BrakeSystem::new()
        .with_assembly(BrakePosition::LeftMain)
        .with_pressure_plate(30.0, 5.0, 5.0, BrakeMaterials::Iron)
        .with_stator(30.0, 5.0, 5.0, BrakeMaterials::CarbonCeramic)
        .with_pressure_plate(30.0, 5.0, 5.0, BrakeMaterials::Iron)
        .with_stator(30.0, 5.0, 5.0, BrakeMaterials::CarbonCeramic)
        .with_pressure_plate(30.0, 5.0, 5.0, BrakeMaterials::Iron)
        .with_stator(30.0, 5.0, 5.0, BrakeMaterials::CarbonCeramic)
        .with_actuator(6.0, 5.0, 3.0)
        .with_assembly(BrakePosition::RightMain)
        .with_pressure_plate(30.0, 5.0, 5.0, BrakeMaterials::Iron)
        .with_stator(30.0, 5.0, 5.0, BrakeMaterials::CarbonCeramic)
        .with_pressure_plate(30.0, 5.0, 5.0, BrakeMaterials::Iron)
        .with_stator(30.0, 5.0, 5.0, BrakeMaterials::CarbonCeramic)
        .with_pressure_plate(30.0, 5.0, 5.0, BrakeMaterials::Iron)
        .with_stator(30.0, 5.0, 5.0, BrakeMaterials::CarbonCeramic)
        .with_actuator(6.0, 5.0, 3.0)
        .build();

    loop {
        // TODO: call the implemented brake_system.calculate() here and feed it the proper mutex vars
        brake_system.clone().calculate();
        // println!("brake: {:?}", brake_system);

        sleep(TICK_SLEEP_DURATION).await;
    }
}

pub async fn hydraulic_system() {
    let mut hydraulic = HydraulicSystem::new();

    hydraulic.add_component(1, hydraulic::ComponentType::Pump);
    hydraulic.add_component(2, hydraulic::ComponentType::Valve);
    hydraulic.add_component(3, hydraulic::ComponentType::Actuator);
    hydraulic.add_connection(1, 2, FlowDirection::InletToOutlet);
    hydraulic.add_connection(2, 3, FlowDirection::InletToOutlet);

    loop {
        hydraulic.simulate_system_async().await;
        sleep(TICK_SLEEP_DURATION).await;
    }
}
