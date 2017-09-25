#![recursion_limit="128"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![feature(vec_remove_item)] 
#![allow(unused_variables)]
#![allow(unused_extern_crates)]
#![feature(i128_type)]
#![feature(i128)]
#![feature(raw)]

extern crate rustue;
extern crate docopt;

#[macro_use] 
extern crate error_chain;

//extern crate env_logger;
//use env_logger::{LogBuilder, Logger};
//#[macro_use] extern crate log;
//use log::{LogLevelFilter,LogRecord, LogLevel};
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
//use log::LogLevel::*;

#[cfg(target_os = "windows")]
extern crate winapi;
#[cfg(target_os = "windows")]
extern crate kernel32;
#[cfg(target_os = "windows")]
extern crate shell32;
#[cfg(target_os = "windows")]
extern crate gdi32;
#[cfg(target_os = "windows")]
extern crate user32;
#[cfg(target_os = "windows")]
extern crate dwmapi;

pub mod errors;
pub mod platform;
pub mod eventloop;
//pub mod scopeLock;
//#[macro_use] pub mod logger;
pub mod outputdevice;
mod logger;




use std::sync::Mutex;
pub use errors::*;
use std::env;
use docopt::Docopt;
use std::error::Error;
use std::io::Write;
use std::ffi::OsString;

use std::thread;
use std::time::Duration;

use platform::misc;

use eventloop::FEngineLoop;
use std::fmt;
use std::time::SystemTime;
use platform::outputDevices::FOutputDevices;
use std::ffi::CString;
use logger::Category::*;
use logger::ELogVerbosity::*;
use logger::UE_LOG;



pub fn IsDebugMode() -> bool{
    if cfg!(debug_assertions){
        true
    }else{
        false
    }
}



#[derive(Deserialize, Debug)]
pub struct Flags{
    flag_version: bool,
    //flag_crashreports: bool,
    flag_abslog : bool,
    flag_allusers: bool,
    flag_auto: bool,
    flag_autocheckoutpackages: bool,
    flag_blascompressionforsize: bool,
    flag_buildmachine: bool,
    flag_bulkimportingsounds: bool,
    flag_neverfirst: bool,
    flag_cmd: bool,
    flag_bnoconsole: bool,
}
const USAGE: &'static str = "
Rust Unreal Engine Editor 

Usage:
    ue_editor [options]

Options: 
    -h, --help          Display this message
    --version       Print version info and exit
    --abslog         like log but no file name check
    --allusers    when choose INSTALLGE, add game for all users
    --auto          
    --autocheckoutpackages   
    --automatedmapbuild 
    --blascompressionforsize 
    --buildmachine
    --bulkimportingsounds
    --check_native_class_sizes
    --codermode
    --compatscale
    --conformdir
    --cookfordemo
    --d3ddebug 
    --debug
    --devcon
    --dumpfileiostats
    --dumpudksurvey
    --fatascriptwarnings
    --final_release
    --fixedseed
    --fixuptangents
    --forcelogflush
    --forcepvrtv
    --forcesoundrecook
    --genericbrowser
    --includeutgamecontent
    --installed
    --installfw
    --uninstallfw
    --installge
    --cultureforcooking
    --lighmassdebug
    --lighmassstats
    --log
    --logtimes
    --noconform
    --nocontentbrowser
    --noinnerexception
    --noloadstartuppackages
    --nologtimes
    --nomodautoload
    --nopause
    --nopauseonsuccess
    --norc
    --noverifygc
    --nowrite
    --outputhreads
    --seekfreeloading
    --seekfreepackagemap
    --seekfreeloadingpcconsole
    --seekfreeloadingserver
    --setthreadnames
    --showmissingloc
    --silent
    --traceanimusage
    --treatloadwarningsaserrors
    --unattended
    --uninstallge
    --useunpublished
    --vadebug
    --verbose
    --verifygc
    --warningsaserrors
    --neverfirst 
    --cmd
    --bnoconsole
";

fn editorInit(engineLoop : FEngineLoop) -> RueResult{


    Ok(())
}

fn editorExit(){

}
struct LL(());

impl fmt::Debug for LL{
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "LL")
    }
}
impl LL{
    pub fn get(&self) -> i32{
        32
    }
    pub fn set(&self, t: i32){

    }
}


fn execute(flags: Flags)->RueResult {
    if flags.flag_version{
        let version = rustue::version();
        println!("{}", version);
        return Ok(())
    }

    let t = LL(());
    println!("t: {:?}", t);



   platform::osinit(&flags);

    let outputDevices = platform::outputDevices::FOutputDevices::new();
    outputDevices.SetupOutputDevices();
    UE_LOG(LogInit, Log, "LogInit log");

    let engineLoop = FEngineLoop::new();

 
    engineLoop.preinit(&flags)?;

    //let slowTask = FScopedSlowTask(100, "EngineInit", "EngineInit_loading", "Loading...");

    let gIsEditor = false;
    if cfg!(WITH_EDITOR){
        println!("with editor");
        editorInit(engineLoop)?;
    }else{
        engineLoop.init()?;
    }
 

    println!("loop init天下无敌");

    let isRequestingExit :bool = false;

    // while (!isRequestingExit){
    //     engineLoop.tick();
    // }

    // if cfg!(WITH_EDITOR){
    //     if (gIsEditor){
    //         editorExit();
    //     }
    // }

    
  //  FPlatformMisc::SetGracefulTerminationHandler();
    // let  mut GAlwaysReportCrash : bool = false;
    // if flags.flag_crashreports {
    //     GAlwaysReportCrash = true;
    // }



    




    // if cfg!(debug_assertions){
    //     println!("debug_assertions");
    // }


    Ok(())
}

pub fn exit_with_error(err: RueError)
{
 // error!("exit_with_error; err={:?}", err);
  let RueError{error, exit_code, unknown} = err;
     let fatal = exit_code != 0;
    if let Some(error) = error{
        if fatal{
                writeln!(&mut std::io::stderr(), "Error: {}", error);
            } else if !unknown{
                writeln!(&mut std::io::stderr(), "{}", error );
        }
    }
}

lazy_static!{
    pub static ref GStartTime :SystemTime = SystemTime::now();
}



fn main(){
    let getTime = &GStartTime;

//    if cfg!(debug_assertions){
//        println!("dddddebg");
//    }


//    // println!("sys time: {:?}", GStartTime);
//      let mut logBuilder = LogBuilder::new();
//     logBuilder.filter(Some("ttt"), LogLevelFilter::Info);
//     logBuilder.filter(None, LogLevelFilter::Trace);
//     logBuilder.format(|record: &LogRecord|{
//         let path = record.location().__module_path;
//         let line = record.location().__line;
//         if record.target() ==  path{
//             format!("{:?} in {}__{:?} : {}", record.level(),  path, line, record.args())
//         }else{
//             format!("{}| {:?} in {}__{:?} :{}", record.target(), record.level(), path, line, record.args())
//         }
//      });

//     logBuilder.init();

//     logger::UE_LOG("2222", &LogLevel::Error, "2222 Error");
    //UE_LOG!("22222", LogLevel::Error, "2222 error");

    // debug!(target: "ttt", "ttt debug");
    // info!(target: "ttt", "ttt info");
    //  warn!(target: "ttt", "ttt warn exeee");


    // trace!("trace!!!");
    // debug!("debug!");
    // info!("info!");
    // warn!("warn!");
    // error!("error!");

    // if log_enabled!(LogLevel::Info) {
    //     let x = 3 * 4; // expensive computation
    //     info!("the answer was: {}", x);
    // }

  
    

     let result = (|| {
        let args: Vec<String> = try!(env::args_os()
        .map(|s| {
                 match s.into_string(){
                     Ok(s) => {
                       //  println!("s: {}", s);
                         if s.starts_with("-") && (s.chars().nth(1).unwrap() != '-'){
                            if  s=="-h" {
                                 return Ok(s)
                             }
                             return Ok(s.replacen("-", "--", 1).to_lowercase())
                         }
                        Ok(s.to_lowercase())},
                     Err(e) => Err(RustUEError::from(format!("invalid unicode in argument: {:?}", e))),
                 }                
            })
            .collect());
        println!("args: {:?}", args);
        

        let docopt = Docopt::new(USAGE).unwrap().argv(&args).help(true);
        
        let flags = docopt.deserialize().map_err(|e| {
              let code = if e.fatal() {1} else {0};
        RueError::new(e.to_string().into(), code)
        })?;

        println!("flags :{:?}", flags);
     

        execute(flags)
     })();
     match result {
         Err( e) => exit_with_error(e),
         Ok(()) => {},
     }

    
    thread::sleep(Duration::new(2,0));
    println!("ue_editor exit");
}

