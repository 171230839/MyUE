use outputdevice::FOutputDevice;
use logger::{Category, ELogVerbosity};
pub struct FOutputDeviceDebug{
    name: String,
 
}

impl FOutputDeviceDebug{
       pub fn new() ->Self{
        FOutputDeviceDebug{
            name: String::from("FOutputDeviceDebug"),
        }
    }
}

impl FOutputDevice for FOutputDeviceDebug{
  //   fn Log(&self, target: &str, level: &LogLevel, arg: &str) {

   //  }
    fn CanBeUsedOnAnyThread(&self) -> bool{
        true
    }
     fn Serialize(&self, category: &Category, level: &ELogVerbosity, data: &String, time: &u64){
        println!("1111{:?}: {}", category, data);
    }
     fn getDeviceName(&self) -> String{
        self.name.clone()
    }
}