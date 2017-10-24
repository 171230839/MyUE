

use outputdevice::{FOutputDeviceRedirector, GLog};
use std::fmt;

// #[derive(Clone, Debug)]
// pub enum Category{
//     LogInit,
//     LogTaskGraph,
// }
// impl fmt::Display for Category {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", self)
//     }
// }

#[derive(Clone, Debug, Eq, PartialEq)]
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
impl fmt::Display for  ELogVerbosity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
pub enum ELogTimes{
    None,
    UTC,
    SinceGStartTime,
    Local,
}




pub fn ue_log(category: &str, level: ELogVerbosity, data: &str){
    GLog.lock().unwrap().Serialize(&category, &level, data);
}


macro_rules! UE_LOG{
    ($category: ident, $level:ident, $data :expr, $($arg:tt)+) =>{
        ue_log(stringify!($category), ELogVerbosity::$level, &format!($data, $($arg)+));
    };
    ($category: ident, $level:ident, $data :expr) =>{
        ue_log(stringify!($category), ELogVerbosity::$level, $data);
    };
}



