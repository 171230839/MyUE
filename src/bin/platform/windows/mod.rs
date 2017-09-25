

use kernel32;
//use winapi::um::errhandlingapi;
use winapi::winerror;
use Flags;
use std::ffi::CString;
use winapi;
use std::ptr;


pub mod misc;
//pub mod criticalSection;
pub mod TLS;
pub mod outputDevices;
// mod outputDeviceEventLog;
// mod outputDeviceWindowsError;
// mod outputDeviceConsole;
// mod feedbackContextWindows;
// mod outputDeviceFile;
pub mod properties;


fn ReleaseNameMutex(mut nameMutex: winapi::HANDLE) {
    if nameMutex != ptr::null_mut() {
        unsafe {
       

            kernel32::ReleaseMutex(nameMutex);
        }
        nameMutex = ptr::null_mut();
    }
}


fn makeNameMutex(flags: &Flags) -> (bool, winapi::HANDLE) {
    let mut bIsFirstInstance: bool = false;
    let name = CString::new("RustUnrealEngine4").unwrap();
    let mut nameMutex: winapi::HANDLE = ptr::null_mut();
    unsafe {
        nameMutex = kernel32::CreateMutexA(ptr::null_mut(), 0, name.as_ptr());
       
        if (nameMutex != ptr::null_mut()) &&
            (kernel32::GetLastError() != winerror::ERROR_ALREADY_EXISTS) &&
            (!flags.flag_neverfirst)
        {
            bIsFirstInstance = true;
            println!("first instance");
        } else {
            ReleaseNameMutex(nameMutex);
            bIsFirstInstance = false;
        }
    }
    (bIsFirstInstance, nameMutex)
}

fn test() -> (i32, String) {
    (1, String::from("test"))
}

pub struct OSResultGuard {
    nameMutex :  winapi::HANDLE,
}

impl Drop for OSResultGuard {
    fn drop(&mut self){
        ReleaseNameMutex(self.nameMutex);
        println!("OSResultGuard Drop");
    }
}

pub fn osinit(flags: &Flags) ->OSResultGuard {
    let (GIsFirstInstance, nameMutex) = makeNameMutex(flags);


    misc::SetGracefulTerminationHandler();
    // let name = String::from("RustUnrealEngine4");
    // let mutex = createMutexForName(name);
    // match mutex.try_lock() {
    //     Ok(_) => {
    //         println!("First instance!");
    //         // Program code goes here. The sleep statement is just a placeholder.
    //         thread::sleep(Duration::from_secs(10));
    //     }
    //     Err(error) => {
    //         println!("An error occurred: {}", error);
    //     }
    // }

   OSResultGuard {
        nameMutex,
   }
}


