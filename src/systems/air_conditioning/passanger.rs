// The average heat output of a human is 105watts or 356Btu's, we  will take a slight range to allow variation in our simulation
#[derive(Debug)]
pub struct Passenger {
    position: i32 // seat that the passenger occupies
}

impl Passenger {
    pub fn new(position:i32) -> Self {
        Self {
            position
        }
    }
}