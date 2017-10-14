
use super::logger::{ELogVerbosity};
//use platform::criticalSection::FCriticalSection;
use platform::FPlatformTLS;
//use scopeLock::FScopeLock;
//use std::collections::HashSet;
//use std::hash::{Hash, Hasher};
use std::ptr;
use std::mem;
//use std::raw::TraitObject;
use std::rc::Rc;
use std::sync::{Arc,RwLock, Mutex};
use std::cell::RefCell;
use GStartTime;
use std::cmp::Ordering;
use std::thread;
use std::time::Duration;


pub trait FOutputDevice : Sync + Send {
   // fn Log(&self, target: &str, level: &LogLevel, arg: &str);
    fn CanBeUsedOnAnyThread(&self) -> bool;
    fn Serialize(&self, category: &String, level: &ELogVerbosity, data: &String, time: &u64);
    fn getDeviceName(&self) -> String;
}

impl Ord for FOutputDevice{
    fn cmp(&self, other: &FOutputDevice) -> Ordering{
        self.getDeviceName().cmp(&other.getDeviceName())
    }
}

impl Eq for FOutputDevice{
}

impl PartialEq for FOutputDevice{
    fn eq(&self, other: &FOutputDevice) -> bool{
        self.getDeviceName() == other.getDeviceName()
    }
}

impl PartialOrd for FOutputDevice{
    fn partial_cmp(&self, other: &FOutputDevice) ->Option<Ordering>{
        Some(self.getDeviceName().cmp(&other.getDeviceName()))
    }
}



struct FBufferedLine {
     category: String,
     level:  ELogVerbosity,
    data:  String ,
     time:  u64,
}

impl FBufferedLine {
    pub fn new(inCategory: &str, inLevel: & ELogVerbosity, inData: & str, inTime: & u64) -> Self {
        FBufferedLine {
          
            category: String::from(inCategory),
             level: inLevel.clone(),
              data: String::from(inData),
            time: inTime.clone(),
           
        }
    }
}

lazy_static!{
   // pub static  ref GLog : Arc<Mutex<FOutputDeviceRedirector>> = Arc::new(Mutex::new(FOutputDeviceRedirector::new()));
    pub static ref GLog: Mutex<FOutputDeviceRedirector> = Mutex::new(FOutputDeviceRedirector::new());
}


pub struct FOutputDeviceRedirector {
    bufferedLines: Vec<FBufferedLine>,
     backlogLines:  Vec<FBufferedLine>,
     outputDevices: Vec<Arc<FOutputDevice>>,
     masterThreadID:  u32,
     bEnableBacklog:  bool,

}
//unsafe impl Sync for FOutputDeviceRedirector{}
//unsafe impl Send for FOutputDeviceRedirector{}


impl FOutputDeviceRedirector {
    pub fn new() -> Self {
        FOutputDeviceRedirector {
            masterThreadID: FPlatformTLS::GetCurrentThreadId(),
            bEnableBacklog: false,
            bufferedLines: Vec::new(),
            backlogLines: Vec::new(),
            outputDevices: Vec::new(),
      }
    }


    pub fn AddOutputDevice(&mut self, outputDevice: Arc<FOutputDevice>) {
        self.outputDevices.push(outputDevice);
       
    }


    pub fn RemoveOutputDevice(&mut self, outputDevice: &Arc<FOutputDevice>) {
     

        let index = self.outputDevices.binary_search(&outputDevice).unwrap();
        self.outputDevices.remove(index);
    }

    pub fn IsRedirectingTo(&mut self, outputDevice: &Arc<FOutputDevice>) -> bool {
     
        match self.outputDevices.binary_search(outputDevice){
            Ok(_) => true,
            Err(_) => false,
        }
        
    }

    pub fn Serialize(&mut self, category: &str, level: &ELogVerbosity, data: &str) {
       // thread::sleep(Duration::new(2,0));
        
     //let realTime = GStartTime.elapsed().unwrap().as_secs();
  
     let time =  GStartTime.elapsed().unwrap();
     let srealTime = format!("{0}", time.as_secs()*1000 + time.subsec_nanos() as u64/1000000 );
     // println!("realTime: {}", srealTime);
    let realTime :u64= srealTime.parse::<u64>().unwrap();
        if self.bEnableBacklog {
            self.backlogLines.push(FBufferedLine::new(category, level, data, &realTime));
        }

        if (FPlatformTLS::GetCurrentThreadId() != self.masterThreadID) ||
            (self.outputDevices.len() == 0)
        {
            self.bufferedLines.push(FBufferedLine::new( category, level, data, &realTime));
        } else {
            self.UnsynchronizedFlushThreadedLogs(true);
          //  println!("count::: {}", self.outputDevices.len());
            for outputDevice in self.outputDevices.iter(){
                outputDevice.Serialize(&String::from(category), level, &String::from(data), &realTime);
            }
        }
    }

    pub fn UnsynchronizedFlushThreadedLogs(&mut self, bUseAllDevices: bool) {
        for bufferedLine in self.bufferedLines.iter() {
            for outputDevice in self.outputDevices.iter() {
               
                if outputDevice.CanBeUsedOnAnyThread() || bUseAllDevices {
                    outputDevice.Serialize(
                        &bufferedLine.category,
                        &bufferedLine.level,
                        &bufferedLine.data,
                        &bufferedLine.time,
                    );
                }
            }
        }
        self.bufferedLines.clear();
    }
}
