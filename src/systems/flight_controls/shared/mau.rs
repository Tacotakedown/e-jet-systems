use core::fmt;

#[derive(Debug)]
pub enum MAUs {
    MAU1,
    MAU2,
    MAU3,
}

impl fmt::Display for MAUs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MAUs::MAU1 => write!(f, "MAU1"),
            MAUs::MAU2 => write!(f, "MAU2"),
            MAUs::MAU3 => write!(f, "MAU3"),
        }
    }
}
