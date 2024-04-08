use core::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Busses {
    AcBus1,
    AcBus2,
    AcEssBus,
    AcGndSvcBus,
    AcStbyBus,
    DcGndSvcBus,
    DcBus1,
    DcBus2,
    DcEssBus1,
    DcEssBus2,
    DcEssBus3,
    HotBatBus1,
    HotBatBus2,
    ApuStartBus,
}

impl fmt::Display for Busses {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Busses::AcBus1 => write!(f, "AcBus1"),
            Busses::AcBus2 => write!(f, "AcBus2"),
            Busses::AcEssBus => write!(f, "AcEssBus"),
            Busses::AcGndSvcBus => write!(f, "AcGndSvcBus"),
            Busses::AcStbyBus => write!(f, "AcStbyBus"),
            Busses::DcGndSvcBus => write!(f, "DcGndSvcBus"),
            Busses::DcBus1 => write!(f, "DcBus1"),
            Busses::DcBus2 => write!(f, "DcBus2"),
            Busses::DcEssBus1 => write!(f, "DcEssBus1"),
            Busses::DcEssBus2 => write!(f, "DcEssBus2"),
            Busses::DcEssBus3 => write!(f, "DcEssBus3"),
            Busses::HotBatBus1 => write!(f, "HotBatBus1"),
            Busses::HotBatBus2 => write!(f, "HotBatBus2"),
            Busses::ApuStartBus => write!(f, "ApuStartBus"),
        }
    }
}
