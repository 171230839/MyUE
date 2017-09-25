
use outputdevice::FOutputDevice;
use logger::{Category, ELogVerbosity};
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
     fn Serialize(&self, category: &Category, level: &ELogVerbosity, data: &String, time: &u64){
        
    }
     fn getDeviceName(&self) -> String{
        self.name.clone()
    }
}