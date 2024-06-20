pub enum EjetCabinLayout {
    E170(E170CabinLayouts),
    E175(E175CabinLayouts),
    E190(E190CabinLayouts),
    E195(E195CabinLayouts),
}

pub enum E170CabinLayouts {
    HasFirstClass,
    NoFirstClass,
}

pub enum E175CabinLayouts {
    HasFirstClass,
    NoFirstClass,
}

pub enum E190CabinLayouts {
    HasFirstClass,
    NoFirstClass,
}

pub enum E195CabinLayouts {
    HasFirstClass,
    NoFirstClass,
}

#[derive(Debug, Clone)]
pub enum SeatStatus {
    Occupied,
    Empty,
    Null, // for when a seat does not exist (first class )
}
pub struct EjetCabin {
    seat_layout: Vec<Vec<SeatStatus>>,
}

impl EjetCabin {
    fn initialize_layout(
        rows: usize,
        cols: usize,
        first_class_rows: usize,
    ) -> Vec<Vec<SeatStatus>> {
        let mut layout = vec![vec![SeatStatus::Empty; cols]; rows];
        for i in 0..first_class_rows {
            layout[i][2] = SeatStatus::Null;
        }
        layout
    }

    pub fn new(cabin_layout: EjetCabinLayout) -> Self {
        let seat_layout = match cabin_layout {
            EjetCabinLayout::E170(E170CabinLayouts::HasFirstClass) => {
                EjetCabin::initialize_layout(18, 4, 2)
            }
            EjetCabinLayout::E170(E170CabinLayouts::NoFirstClass) => {
                EjetCabin::initialize_layout(18, 4, 0)
            }

            EjetCabinLayout::E175(E175CabinLayouts::HasFirstClass) => {
                EjetCabin::initialize_layout(20, 4, 3)
            }
            EjetCabinLayout::E175(E175CabinLayouts::NoFirstClass) => {
                EjetCabin::initialize_layout(20, 4, 0)
            }

            EjetCabinLayout::E190(E190CabinLayouts::HasFirstClass) => {
                EjetCabin::initialize_layout(24, 4, 4)
            }
            EjetCabinLayout::E190(E190CabinLayouts::NoFirstClass) => {
                EjetCabin::initialize_layout(24, 4, 0)
            } // TODO: figure out the correct dimensions for these

            EjetCabinLayout::E195(E195CabinLayouts::HasFirstClass) => {
                EjetCabin::initialize_layout(26, 4, 5)
            }
            EjetCabinLayout::E195(E195CabinLayouts::NoFirstClass) => {
                EjetCabin::initialize_layout(26, 4, 0)
            }
        };

        Self { seat_layout }
    }
}
