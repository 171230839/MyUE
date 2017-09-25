use outputdevice::FOutputDevice;
use logger::{Category, ELogVerbosity};
pub struct FOutputDeviceConsoleWindows{
    name: String,
}


impl FOutputDeviceConsoleWindows{
    pub fn new() ->Self{
        FOutputDeviceConsoleWindows{
            name: String::from("FOutputDeviceConsolewindows"),
        }
    }
}

impl FOutputDevice for FOutputDeviceConsoleWindows{
 //    fn Log(&self, target: &str, level: &LogLevel, arg: &str) {

  //   }
    fn CanBeUsedOnAnyThread(&self) -> bool{
        true
    }
    fn Serialize(&self, category: &Category, level: &ELogVerbosity, data: &String, time: &u64){
        
    }
     fn getDeviceName(&self) -> String{
        self.name.clone()
    }
}