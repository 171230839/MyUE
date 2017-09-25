

use outputdevice::{FOutputDeviceRedirector, GLog};

#[derive(Clone, Debug)]
pub enum Category{
    LogInit,
}


#[derive(Clone, Debug)]
pub enum ELogVerbosity{
    NoLogging,
    Fatal,
    Error,
    Warning,
    Display,
    Log,
    Verbose,
    VeryVerbose,
}

// macro_rules! UE_LOG{
//     ($target:expr, $level:expr, $($arg:tt)*) =>{
//         GLog.Serialize($target, $level, $($arg)*);
//     }
// }

pub fn UE_LOG(category: Category, level: ELogVerbosity, data: &str){
    GLog.lock().unwrap().Serialize(&category, &level, data);
}



