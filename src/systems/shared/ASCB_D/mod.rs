//  (Avionics Standard Communication Bus (version D)) data bus system supplies the AIOP interface data to the FCM (Flight ControlModule)s.
// our simulation will simply act as a getter and setter (send and recieve) of the proper list of mutex vars. when constructing, we will feed the mutex pointers (global communication buss).
// we will write to this struct and read from it, the mutex only serves to transfer data across threads. on every tick, this will be updated to match the mutex vars. think of this as merely an abstraction for calling the proper functions

use serde::de::value;

use crate::mutex::MutexVariables;

#[derive(Debug)]
pub enum AscbDChannels {
    Channel1,
    // not actually going to be named obscure names like this, rather we will implement a channel per variable we are trying to read and write to to avoid prolonged periods of locked mutex vars in this thread
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct AscbDData {
    blah: f64,
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct AscbD {
    mutex_vars: MutexVariables,
}

impl AscbD {
    pub fn new(mutex_ptr: MutexVariables) -> Self {
        //TODO: Implement this fn
        Self {
            mutex_vars: mutex_ptr,
        }
    }

    async fn sync_with_mutex(&self) {
        //TODO: IMPL this fn
        self.mutex_vars.write_ascb_vars();
    }

    pub fn write_to_bus(&mut self, channel: AscbDChannels, value: f64) {
        let mut mutex_bus_locked = self.mutex_vars.read_ascb_vars();
        match channel {
            AscbDChannels::Channel1 => {
                // mutex_bus_locked.channel1 = value
            }
        }
        drop(mutex_bus_locked);
    }
}
