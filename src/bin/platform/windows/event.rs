
use kernel32;
use winapi::HANDLE;
use std::ptr::{null, null_mut};
use winapi::winbase::WAIT_OBJECT_0;
use std::sync::{Arc, Mutex};
pub const INFINITE: u32 = 0xFFFFFFFF;

lazy_static!{
    pub static ref GManualResetFEvent: FEvent = FEvent::new(true);
}

lazy_static!{
    pub static ref GAutoResetFEvent : Mutex<FEvent> = Mutex::new(FEvent::new(false));
}

#[derive(Debug, Clone)]
pub struct FEvent {
    event: HANDLE,
    manualReset: bool,

}

impl FEvent {
    pub fn new(bIsManualReset: bool) -> Self {
        unsafe {
            let event: HANDLE =
                kernel32::CreateEventA(null_mut(), bIsManualReset as i32, 0, null_mut());
            FEvent {
                event: event,
                manualReset: bIsManualReset,
            }
        }
    }

    pub fn IsManualReset(&self) -> bool {
        self.manualReset
    }

    pub fn Trigger(&self) {
        println!("trigger ");
        unsafe {
            kernel32::SetEvent(self.event);
        }
    }

    pub fn Reset(&self) {
        unsafe {
            kernel32::ResetEvent(self.event);
        }
    }

    pub fn Wait(&self, waitTime: u32) -> bool {
        println!("wait");
        unsafe { kernel32::WaitForSingleObject(self.event, waitTime) == WAIT_OBJECT_0 }
    }
}


impl Drop for FEvent {
    fn drop(&mut self) {
        unsafe {
            kernel32::CloseHandle(self.event);
        }
    }
}

unsafe impl Sync for FEvent{}
unsafe impl Send for FEvent{}

#[derive(Debug)]
pub struct FSafeEvent {
    pub innerEvent: FEvent,
}
impl Drop for FSafeEvent {
    fn drop(&mut self) {
        self.innerEvent.Reset();
    }
}

// pub fn GetEvent(bIsManualReset: bool) -> FSafeEvent {
//     match bIsManualReset {
//         true => FSafeEvent {
//             innerEvent: GManualResetFEvent,
//         },
//         false => FSafeEvent {
//             innerEvent: GAutoResetFEvent,
//         },
//     }
// }

// pub fn GetEvent(bIsManualReset: bool) -> Arc<Mutex<FEvent>> {
//     match bIsManualReset {
//         true =>  (*GManualResetFEvent).clone(),
//         false =>  (*GAutoResetFEvent).clone(),
//     }
// }