
use winapi::minwinbase::CRITICAL_SECTION;
use std::ptr;
use std::mem;
use kernel32;

pub struct FCriticalSection {
    criticalSection: CRITICAL_SECTION,
    ptr: *mut CRITICAL_SECTION,
}

unsafe impl Sync for FCriticalSection{}
unsafe impl Send for FCriticalSection{}

impl FCriticalSection {
    pub fn new() -> Self {
        let mut cri = CRITICAL_SECTION {
            DebugInfo: ptr::null_mut(),
            LockCount: 0,
            RecursionCount: 0,
            OwningThread: ptr::null_mut(),
            LockSemaphore: ptr::null_mut(),
            SpinCount: 0,
        };

        unsafe {
            let p: *mut CRITICAL_SECTION = mem::transmute(&cri);
            kernel32::InitializeCriticalSection(p);
            kernel32::SetCriticalSectionSpinCount(p, 4000);
            FCriticalSection {
                criticalSection: cri,
                ptr: p,
            }
        }
    }

    // pub fn drop(&self) {
    //     unsafe {
    //       //  let ptr: *mut CRITICAL_SECTION = mem::transmute(&self.criticalSection);
    //         kernel32::DeleteCriticalSection(self.ptr);
    //     }
    // }


    pub fn Lock(&self) {
        unsafe {
            if kernel32::TryEnterCriticalSection(self.ptr) == 0 {
                kernel32::EnterCriticalSection(self.ptr);
            }
        }
    }

    pub fn TryLock(&self) -> bool {
        unsafe {
            if kernel32::TryEnterCriticalSection(self.ptr) != 0 {
                return true;
            }
            false
        }
    }


    pub fn Unlock(& self) {
        unsafe {
            kernel32::LeaveCriticalSection(self.ptr);
        }
    }
}



impl Drop for FCriticalSection {
    fn drop(&mut self) {
        unsafe {
            kernel32::DeleteCriticalSection(self.ptr);
        }
    }
}
