
use outputdevice::FOutputDevice;
use logger::{ELogVerbosity};
pub struct FOutputDeviceEventLog{
    name: String,
}

impl  FOutputDeviceEventLog{
    pub fn new() ->Self{
        FOutputDeviceEventLog{
            name: String::from("FOutputDeviceEventLog"),
        }
    }
}


impl FOutputDevice for FOutputDeviceEventLog{
    // fn Log(&self, target: &str, level: &LogLevel, arg: &str) {

   //  }
    fn CanBeUsedOnAnyThread(&self) -> bool{
        true
    }
     fn Serialize(&mut self, category: &String, level: &ELogVerbosity, data: &String, time: &u64){
        
    }
     fn getDeviceName(&self) -> String{
        self.name.clone()
    }
}