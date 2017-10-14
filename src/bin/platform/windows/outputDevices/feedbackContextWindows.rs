
use outputdevice::FOutputDevice;
use logger::{ ELogVerbosity};
pub struct FFeedbackContextWindows{
    name: String,
}



impl FFeedbackContextWindows{
    pub fn new() -> Self{
        FFeedbackContextWindows{
            name: String::from("FFeedbackContextWindows"),
        }
    }
}


impl FOutputDevice for FFeedbackContextWindows{
    // fn Log(&self, target: &str, level: &LogLevel, arg: &str) {

    // }
    fn CanBeUsedOnAnyThread(&self) -> bool{
        true
    }
      fn Serialize(&mut self, category: &String, level: &ELogVerbosity, data: &String, time: &u64){
        
    }
      fn getDeviceName(&self) -> String{
        self.name.clone()
    }
}