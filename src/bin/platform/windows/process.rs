

use platformAffinity::FPlatformAffinity;
use super::TLS::FPlatformTLS;
use kernel32;
pub struct FPlatformProcess{

}

impl FPlatformProcess{

    pub fn SetThreadAffinityMask( affinityMask : u64){
      //  if affinityMask != FPlatformAffinity::Mask.bits(){
          if affinityMask != FPlatformAffinity::Mask as u64 {
            unsafe{
                kernel32::SetThreadAffinityMask(FPlatformTLS::GetCurrentThread(), affinityMask);
            }
        }
    }

    pub fn SetupGameThread() {}

    // pub fn SupportsMultithreading() -> bool{
    //     true
    // }

    // pub fn GetSynchEventFromPool(bIsManualReset : bool) -> FEvent{
        
    // }
}