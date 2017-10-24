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
#![feature(type_ascription)]
//#![windows_subsystem = "windows"]
#![feature(integer_atomics)]

extern crate rustue;

#[macro_use] 
extern crate error_chain;

//extern crate env_logger;
//use env_logger::{LogBuilder, Logger};
//#[macro_use] extern crate log;
//use log::{LogLevelFilter,LogRecord, LogLevel};
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate bitflags;

//use log::LogLevel::*;

#[cfg(target_os = "windows")]
extern crate winapi;
#[cfg(target_os = "windows")]
extern crate kernel32;
// #[cfg(target_os = "windows")]
// extern crate shell32;
// #[cfg(target_os = "windows")]
// extern crate gdi32;
// #[cfg(target_os = "windows")]
// extern crate user32;
// #[cfg(target_os = "windows")]
// extern crate dwmapi;
extern crate chrono;
extern crate rand;
#[macro_use] extern crate rand_derive;

#[macro_use] mod logger;
use logger::{ue_log, ELogVerbosity};
pub mod errors;
pub mod platform;
pub mod eventloop;
//pub mod scopeLock;
//#[macro_use] pub mod logger;
pub mod outputdevice;


mod outputDeviceHelper;
mod app;
use app::FApp;
mod path;
mod projectManager;

mod projectDescriptor;
use projectDescriptor::FProjectDesriptor;

mod platformAffinity;

mod taskGraph;
//mod runnableThread;

mod threadManager;

pub mod HAL;
pub use self::HAL::*;

pub mod Math;
pub use self::Math::*;

pub mod UObject;
pub use self::UObject::*;

pub mod stat2;

use std::sync::Mutex;
pub use errors::*;
use std::env;

use std::error::Error;
use std::io::Write;
use std::ffi::OsString;

use std::thread;
use std::time::Duration;

use platform::misc;

use eventloop::FEngineLoop;
use std::fmt;
use std::time::SystemTime;
use platform::FOutputDevices;
use std::ffi::CString;

use std::path::Path;


pub fn IsDebugMode() -> bool{
    if cfg!(debug_assertions){
        true
    }else{
        false
    }
}


fn editorInit(engineLoop : FEngineLoop) -> RustResult{


    Ok(())
}

fn editorExit(){

}


fn execute(flags: Flags)->RustResult {

   platform::osinit(&flags);

   

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


    Ok(())
}

pub fn exit_with_error(err: RustUEError)
{
 // error!("exit_with_error; err={:?}", err);
//   let RueError{error, exit_code, unknown} = err;
//      let fatal = exit_code != 0;
//     if let Some(error) = error{
//         if fatal{
//                 writeln!(&mut std::io::stderr(), "Error: {}", error);
//             } else if !unknown{
//                 writeln!(&mut std::io::stderr(), "{}", error );
//         }
//     }
    writeln!(&mut std::io::stderr(), "Error: {}", err  );
}

lazy_static!{
    pub static ref GStartTime :SystemTime = SystemTime::now();
}

 lazy_static!{
     pub static ref GApp : Mutex<FApp> = Mutex::new(Default::default()); 
 }



fn main(){
   lazy_static::initialize(&GStartTime);

     let result = (|| {
        let mut args: Vec<String> = try!(env::args_os()
        .map(|s| {
                 match s.into_string(){
                     Ok(s) => {
                       //  println!("s: {}", s);
                        //  if s.starts_with("-") && (s.chars().nth(1).unwrap() != '-'){
                        //     if  s=="-h" {
                        //          return Ok(s)
                        //      }
                        //      return Ok(s.replacen("-", "--", 1).to_lowercase())
                        //  }
                        Ok(s.to_lowercase())},
                     Err(e) => Err(RustUEError::from(format!("invalid unicode in argument: {:?}", e))),
                 }                
            })
            .skip(1).collect());
               
        let outputDevices = platform::outputDevices::FOutputDevices::new();
        outputDevices.SetupOutputDevices();
       // UE_LOG!(Init, Error, "TTTTTTTTTTT");

        let flags: Flags = parseCmdArgs(&mut args);

        println!("flags : {:?}", flags);

   

        execute(flags)
     })();
     match result {
         Err( e) => exit_with_error(e),
         Ok(()) => {},
     }

    
    thread::sleep(Duration::new(10,0));
    println!("ue_editor exit");
}




#[derive(Default, Debug)]
pub struct Flags{
    projectFilePath: String,
    gamename : String,

    pub bFlags: BFlags,
    pub sFlags: MapFlags,
}

macro_rules! setBool {
    ($this: ident, $flagName: ident, $($name: ident), + ) => {
   $(
            if $flagName == stringify!($name) {
                     $this.$name = true;
            }
        )+
    };
}

macro_rules! my_macro_for_bflags {
     (struct $name:ident {
        $($field_name:ident: $field_type:ty,)*
    }) => {
   #[derive(Default, Debug)]
        pub struct $name {
            $($field_name: $field_type,)*
        }

        impl $name {
     
            fn setValue(&mut self, str: &str) {
                setBool!(self, str, $($field_name), *);
          }

        }
    }
}



my_macro_for_bflags!{
    struct BFlags{
        
        neverfirst: bool,
    }
}

macro_rules! setString {
    ($this: ident, $key: ident, $value: ident, $($name: ident), + ) => {
    $(
            if $key == stringify!($name) {
                     $this.$name = String::from($value);
            }
        )+
    };
}

macro_rules! my_macro_for_mapflags {
     (struct $name:ident {
        $($field_name:ident: $field_type:ty,)*
    }) => {
   #[derive(Default, Debug)]
        pub struct $name {
            $($field_name: $field_type,)*
        }
        impl $name {
            fn setValue(&mut self, key: &str, value: &str) {
                setString!(self, key, value, $($field_name), *);
          }

        }
    }
}

my_macro_for_mapflags!{
    struct MapFlags{
        log : String,
    }
}



fn parseCmdArgs( args: &mut Vec<String>) -> Flags{

    
   let mut flags:Flags = Default::default();
    if args.len() > 0{
         let token = args[0].clone();
        if !token.starts_with("-") && !token.contains("="){
           // let str = args[0].clone();
              //    UE_LOG(LogInit, Error, &format!("parseCMDarg  str: {}", str));
           
            let path = Path::new(&token);
            if path.exists(){
                if path.is_absolute(){
                    match path.extension() {
                        Some(ext) => { 
                            if ext.to_os_string().into_string().unwrap() == FProjectDesriptor::GetExtension(){
                                flags.projectFilePath = token.clone();
                                flags.gamename = path.file_stem().unwrap().to_os_string().into_string().unwrap();
                                args.remove(0);
                                
                            GApp.lock().unwrap().SetGameName(&token);
                              UE_LOG!(LogInit, Display, &format!("Running engine for game: {}", token));    
                        
                            }
                        },
                        None =>{},
                    }
                
                } else if path.is_relative() {
                    let gamename = path.file_stem().unwrap().to_os_string().into_string().unwrap();
                    flags.gamename = gamename;
                    let mut curdir = std::env::current_dir().unwrap().into_os_string().into_string().unwrap();
                    curdir.push_str(&token);
                    curdir.push_str(".");
                    curdir.push_str(&FProjectDesriptor::GetExtension());
                    flags.projectFilePath = curdir;
                    args.remove(0);
                     GApp.lock().unwrap().SetGameName(&token);
                       
                }
            }else{
                UE_LOG!(LogInit, Error, &format!("file path doesn't exists :{}", token));
             //  Err(RustUEError::from(format!("first argument as path is not exist :{}", str)));
            }
         
        }
    }


    if GApp.lock().unwrap().HasGameName(){
        UE_LOG!(LogInit, Display, &format!("Running engine for game: {}", GApp.lock().unwrap().GetGameName()));
    }else{
         UE_LOG!(LogInit, Display, "Running engine without a game");
    }

    for arg in args.iter(){
        println!("arg : {}", arg);
        if arg.starts_with("-") && !arg.contains("="){
            let mut s :String = arg.clone();
            //let mut s = String::from(temp);
            s.remove(0);
            flags.bFlags.setValue(&s);

        } else if arg.contains("="){
            let v : Vec<&str> = arg.split("=").collect();
            if v.len() == 2{
                flags.sFlags.setValue(v[0], v[1]);
            }else{
                UE_LOG!(LogInit, Warning, "cmd arg format error " );
            }
       
        }
    }

    flags
}


