#![recursion_limit="128"]

extern crate rustue;
extern crate docopt;

#[macro_use] 
extern crate error_chain;

extern crate env_logger;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

mod errors;
use errors::*;
use std::env;
use docopt::Docopt;
use std::error::Error;
use std::io::Write;
use log::LogLevel;
use std::ffi::OsString;









#[derive(Deserialize, Debug)]
pub struct Flags{
    flag_version: bool,
    flag_crashreports: bool,
}
const USAGE: &'static str = "
Rust Unreal Engine Editor Config

Config:
    ue_editor [options]

Options: 
    -h, --help          Display this message
    --crashreports      set up program always report crash

";


fn execute(flags: Flags)->RueResult {
    if flags.flag_version{
        let version = rustue::version();
        println!("{}", version);
    }



    Ok(())
}

pub fn exit_with_error(err: RueError)
{
  error!("exit_with_error; err={:?}", err);
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

fn main(){
     env_logger::init();


    trace!("trace!!!");
    debug!("this is a debug {}", "message");
    info!("info!");
    warn!("warn!");
    error!("this is printed by default");

    // if log_enabled!(LogLevel::Info) {
    //     let x = 3 * 4; // expensive computation
    //     info!("the answer was: {}", x);
    // }

  
    

     let result = (|| {
        let args: Vec<String> = try!(env::args_os()
        .map(|s| {
                 match s.into_string(){
                     Ok(s) => {
                         println!("s: {}", s);
                         if s.starts_with("-") && (s.chars().nth(1).unwrap() != '-'){
                            if  s=="-h" {
                                 return Ok(s)
                             }
                             return Ok(s.replacen("-", "--", 1))
                         }
                        Ok(s)},
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

    
 
}

