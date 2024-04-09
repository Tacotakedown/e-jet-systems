// if hot food is served we can construct this struct to calculate the heat produced from the food

#[derive(Debug)]
pub struct Food {
    heat_output:f64, // watts
    position: Vec<i32> // seat
}

impl Food {
    pub fn new(heat_output:f64) ->Self {
        Self{
            heat_output,
            position:Vec::new()
        }
    }
    // for simulation purpose, food cannot be removed, instead the potential energy will just be dissipated
    pub fn add_food(&mut self, positions:Vec<i32>) {
        self.position = positions;
    }
}