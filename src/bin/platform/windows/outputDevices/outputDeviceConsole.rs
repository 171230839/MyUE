use outputdevice::FOutputDevice;
use logger::{ ELogVerbosity, ELogTimes};
use winapi::winnt::HANDLE;
use std::ptr;
use winapi;
use kernel32;
use std::mem;
use winapi::minwindef::DWORD;
use winapi::wincon::COORD;
use outputDeviceHelper;
use std::os::raw::c_void;

bitflags! {
   struct Flags: u16{
  const COLOR_BLACK = 0b00000000;

  const COLOR_DARK_RED = 0b01000000;
  const COLOR_DARK_GREEN = 0b00100000;
  const COLOR_DARK_BLUE = 0b00010000;
  const COLOR_DARK_YELLOW = 0b01100000;
  const COLOR_DARK_CYAN = 0b00110000;
  const COLOR_DARK_PURPLE = 0b01010000;
  const COLOR_DARK_WHITE = 0b01110000;

  const COLOR_RED = 0b00001100;
  const COLOR_GREEN = 0b00001010;
  const COLOR_BLUE = 0b00001001;
  const COLOR_YELLOW = 0b00001110;
  const COLOR_CYAN = 0b00001011;
  const COLOR_PURPLE = 0b00001101;
  const COLOR_WHITE = 0b00001111;
  }
}


fn setColor(consoleHandle: HANDLE, color: Flags) {
    unsafe {
        kernel32::SetConsoleTextAttribute(consoleHandle, color.bits());
    }
}

fn to_wide_chars(s: &String) -> Vec<u16> {
  use std::ffi::OsStr;
  use std::os::windows::ffi::OsStrExt;
  OsStr::new(s)
    .encode_wide()
    .chain(Some(0).into_iter())
    .collect::<Vec<_>>()
}


fn writeConsoleData(consoleHandle: HANDLE, data: &String)
{
   let mut written: u32 = 0;
   let str = to_wide_chars(data);
   unsafe {
        kernel32::WriteConsoleW(
            consoleHandle,
            mem::transmute(str.as_ptr()),
            str.len() as u32,
            mem::transmute(&written),
            ptr::null_mut(),
        );
   }
}


pub struct FOutputDeviceConsoleWindows {
    name: String,
    //consoleHandle: HANDLE,
    consoleHandle: Box<i32>,
}


impl FOutputDeviceConsoleWindows {
    pub fn new() -> Self {
     
        unsafe {
            kernel32::AllocConsole();
            
            let console = kernel32::GetStdHandle(winapi::winbase::STD_OUTPUT_HANDLE);
          
            if console != winapi::shlobj::INVALID_HANDLE_VALUE {
                let size = COORD { X: 160, Y: 4000 };
                let consoleWidth = 160;
                kernel32::SetConsoleScreenBufferSize(console, size);
                let consoleHandle  = Box::from_raw(console as *mut i32);
                FOutputDeviceConsoleWindows {
                    name: String::from("FOutputDeviceConsolewindows"),
                    consoleHandle: consoleHandle,
                }
            }else{
                FOutputDeviceConsoleWindows {
                    name: String::from("FOutputDeviceConsolewindows"),
                    consoleHandle: Box::new(0),
                }
            }

        }
    }

    // fn initConsole(&mut self) {
    //     unsafe {
    //         kernel32::AllocConsole();
            
    //         let console = kernel32::GetStdHandle(winapi::winbase::STD_OUTPUT_HANDLE);
    //         self.consoleHandle  = Box::from_raw(console as *mut i32);
    //         if self.consoleHandle != winapi::shlobj::INVALID_HANDLE_VALUE {
    //             let size = COORD { X: 160, Y: 4000 };
    //             let consoleWidth = 160;
    //             kernel32::SetConsoleScreenBufferSize(self.consoleHandle, size);
    //         }
    //     }
    // }
}

impl FOutputDevice for FOutputDeviceConsoleWindows {
    //    fn Log(&self, target: &str, level: &LogLevel, arg: &str) {

    //   }
    fn CanBeUsedOnAnyThread(&self) -> bool {
        true
    }
    fn Serialize(&self, category: &String, level: &ELogVerbosity, data: &String, time: &u64) {
       // self.initConsole();
        let mut bNeedToResetColor = false;
        let console = Box::into_raw(self.consoleHandle.clone()) as *mut c_void ;
        if level == &ELogVerbosity::Error {
            setColor(console, Flags::COLOR_RED);
            bNeedToResetColor = true;
        } else if level == &ELogVerbosity::Warning {
            setColor(console, Flags::COLOR_YELLOW);
            bNeedToResetColor = true;
        }

        let result = outputDeviceHelper::FormatLogLine(category, level, data, time, ELogTimes::UTC);

        
       writeConsoleData(console, &result);

        if bNeedToResetColor {
            setColor(
                console,
                Flags::COLOR_WHITE,
            );
        }
    }
    fn getDeviceName(&self) -> String {
        self.name.clone()
    }
}
