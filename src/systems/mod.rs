use self::electric::{busses::Busses, current_type::CurrentType, ElectricalSystem};

pub mod brakes;
pub mod electric;
pub mod hydraulic;
pub mod shared;

pub async fn systems() {
    let electrical_system = ElectricalSystem::new()
        .with_bus(
            Busses::AcBus1,
            CurrentType::AC,
            120.0,
            "L:OBJ_E170_AC_BUS_1_V".to_string(),
        )
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
        .with_battery(
            "BATT1".to_string(),
            28.0,
            6000.0,
            vec![Busses::HotBatBus1, Busses::HotBatBus2],
        )
        .with_generator(
            "ENG1".to_string(),
            120.0,
            5000.0,
            vec![Busses::AcBus1, Busses::AcBus2],
        )
        .build();
}
