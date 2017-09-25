

use outputdevice::FOutputDevice;
use logger::{Category, ELogVerbosity};
pub struct FOutputDeviceWindowsError{
    name: String,
}


impl FOutputDeviceWindowsError{
    pub fn new() -> Self{
        FOutputDeviceWindowsError{
            name: String::from("FOutputDeviceWindowsError"),
        }
    }
}


impl FOutputDevice for FOutputDeviceWindowsError{
  //   fn Log(&self, target: &str, level: &LogLevel, arg: &str) {

    // }
    fn CanBeUsedOnAnyThread(&self) -> bool{
        true
    }
  fn Serialize(&self, category: &Category, level: &ELogVerbosity, data: &String, time: &u64){
        
    }
     fn getDeviceName(&self) -> String{
        self.name.clone()
    }
}