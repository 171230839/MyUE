
use outputdevice::FOutputDevice;
use logger::{ ELogVerbosity};


#[derive(Clone)]
pub struct FOutputDeviceFile {
    name: String,
}

impl FOutputDeviceFile {
    pub fn new() -> Self {
        println!("FOutputDeviceFile new");
        FOutputDeviceFile {
            name: String::from("FOutputDeviceFile"),
        }
    }


       pub fn print(&self) {
        println!("FOutputDeviceFile print");
    }
}

impl Drop for FOutputDeviceFile{
    fn drop(&mut self){
        println!("FOutputDeviceFile Drop");
    }
}


impl FOutputDevice for FOutputDeviceFile {
    //  fn Log(&self, target: &str, level: &LogLevel, arg: &str) {

    //   }
    fn CanBeUsedOnAnyThread(&self) -> bool {
        true
    }
    fn Serialize(&self, category: &String, level: &ELogVerbosity, data: &String, time: &u64) {}

    fn getDeviceName(&self) -> String{
        self.name.clone()
    }
}
