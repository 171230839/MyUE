

use kernel32::*;
use std::env;
use std::ptr;
use winapi;
use winapi::winnt::PSYSTEM_LOGICAL_PROCESSOR_INFORMATION;
use winapi::winnt::SYSTEM_LOGICAL_PROCESSOR_INFORMATION;

use winapi::winnt::LOGICAL_PROCESSOR_RELATIONSHIP;
use winapi::winerror::ERROR_INSUFFICIENT_BUFFER;
use winapi::winnt::RelationProcessorCore;

use std::mem;
use std::cmp;

pub struct FPlatformMisc{

}

enum Thread{
    MaxGameThreads = 4,
    MaxThreads = 16,
}


impl FPlatformMisc{
    pub fn SetGracefulTerminationHandler() {
        println!("set");
    }

    pub fn IsDebuggerPresent() -> bool {
        unsafe {
        //kernel32::DebugBreak();
            IsDebuggerPresent() != 0
        }
    }

    pub fn EngineDir() -> String{
         let mut exePath = env::current_exe().unwrap();
         exePath.pop();
         exePath.pop();
         exePath.push("Engine");
         exePath.into_os_string().into_string().unwrap()

    }

    pub fn NumberOfCores() -> i32{
unsafe{
        
        let mut needed_size = 0;
        let result = GetLogicalProcessorInformation(ptr::null_mut(), &mut needed_size);
        assert!(result == 0);
        assert!(needed_size > 0);
        assert!(GetLastError() == ERROR_INSUFFICIENT_BUFFER);
        let struct_size = mem::size_of::<SYSTEM_LOGICAL_PROCESSOR_INFORMATION>() as u32;
        println!("struct_size : {}", struct_size);
        let count = needed_size /struct_size;
        let mut buf = Vec::with_capacity(count as usize);

        let result2 =  GetLogicalProcessorInformation(buf.as_mut_ptr(), &mut needed_size);
        println!("result2: {}", result2);
        buf.set_len(count as usize);
        let phys_proc_count = buf.iter()
        // Only interested in processor packages (physical processors.)
        .filter(|proc_info| proc_info.Relationship == RelationProcessorCore)
        .count();
        println!("count: {}", phys_proc_count);

        phys_proc_count as i32
            
        }
        
    }
    
    pub fn NumberOfWorkerThreadsToSpawn() -> i32{
        let cores = FPlatformMisc::NumberOfCores();
        let workerWanted = Thread::MaxThreads as i32;
        cmp::max(cmp::min(cores -1 , workerWanted), 1)
    }
}


