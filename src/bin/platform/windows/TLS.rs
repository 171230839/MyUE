use kernel32;
use std::os::raw::c_void;
use winapi::winnt::HANDLE;

pub struct FPlatformTLS{

}

impl FPlatformTLS{

    pub fn IsValidTlsSlot(slotIndex: u32) -> bool{
        return slotIndex != 0xFFFFFFFF
    }

    pub fn GetCurrentThreadId() ->u32 {
        unsafe{
        kernel32::GetCurrentThreadId()
        }
    }

    pub fn GetCurrentThread() -> HANDLE{
        unsafe{
            kernel32::GetCurrentThread()
        }
    }

    pub fn AllocTlsSlot() -> u32{
        unsafe{
        kernel32::TlsAlloc()
        }
    }

    pub fn SetTlsValue(slotIndex: u32, value: *mut c_void){
        unsafe{
            kernel32::TlsSetValue(slotIndex, value); 
        }

    }

    pub fn GetTlsValue(slotIndex: u32) -> *mut c_void{
        unsafe{
            kernel32::TlsGetValue(slotIndex)
        }
    }

    pub fn FreeTlsSlot(slotIndex: u32) {
        unsafe{
            kernel32::TlsFree(slotIndex);
        }
    }
}