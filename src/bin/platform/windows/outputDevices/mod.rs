

// use super::outputDeviceConsole::FOutputDeviceConsoleWindows;
// use super::outputDeviceEventLog::FOutputDeviceEventLog;
// use super::outputDeviceWindowsError::FOutputDeviceWindowsError;
// use super::feedbackContextWindows::FFeedbackContextWindows;
// use super::outputDeviceFile::FOutputDeviceFile;
// use super::properties::FPlatformProperties;
//mod outputDeviceEventLog;
//mod outputDeviceWindowsError;
mod outputDeviceConsole;
//mod feedbackContextWindows;
mod outputDeviceFile;
mod outputDeviceDebug;

use self::outputDeviceConsole::FOutputDeviceConsoleWindows;
//use self::outputDeviceEventLog::FOutputDeviceEventLog;
//use self::outputDeviceWindowsError::FOutputDeviceWindowsError;
//use self::feedbackContextWindows::FFeedbackContextWindows;
use self::outputDeviceFile::FOutputDeviceFile;
use self::outputDeviceDebug::FOutputDeviceDebug;
use super::properties::FPlatformProperties;
use super::misc;

//mod properties;


use outputdevice::GLog;
use outputdevice::FOutputDevice;
use Flags;
use IsDebugMode;
use std::mem;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex, Once,RwLock, ONCE_INIT};

pub struct FOutputDevices{
   pub consoleLog: Option<Rc<RefCell<FOutputDevice>>>,
   pub log: Option<Rc<RefCell<FOutputDevice>>>,
   pub eventLog: Option<Rc<RefCell<FOutputDevice>>>,
   pub error: Option<Rc<RefCell<FOutputDevice>>>,
   pub warn: Option<Rc<RefCell<FOutputDevice>>>,
}


impl FOutputDevices{
      pub fn new() ->Self{
        // let console = Rc::new(RefCell::new(FOutputDeviceConsoleWindows::new()));;
        // let l = Rc::new(RefCell::new(FOutputDeviceFile::new()));
        // let error = Rc::new(RefCell::new(FOutputDeviceWindowsError::new()));
        // let event = Rc::new(RefCell::new(FOutputDeviceEventLog::new()));
        // let warn = Rc::new(RefCell::new(FFeedbackContextWindows::new()));
        
      
        // FOutputDevices{
        //     consoleLog : console,
        //     log: l,
        //     eventLog: event,
        //     error: error,
        //     warn: warn,
        // }
        FOutputDevices{
            consoleLog: None,
            log: None,
            eventLog: None,
            error: None,
            warn: None,
        }
    }

    pub fn SetupOutputDevices(&self){
        // let console = Arc::new(FOutputDeviceConsoleWindows::new());
       //  GLog.lock().unwrap().AddOutputDevice(console.clone());

       // GLog.lock().unwrap().AddOutputDevice(self.consoleLog.clone());
       // GLog.lock().unwrap().AddOutputDevice(self.log.clone());
         //    if misc::IsDebuggerPresent(){
      //  if ::IsDebugMode(){
       let debug = Arc::new(FOutputDeviceDebug::new());
            GLog.lock().unwrap().AddOutputDevice(debug.clone());
       // }
    }
}