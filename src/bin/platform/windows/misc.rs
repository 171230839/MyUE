

use kernel32;


pub fn SetGracefulTerminationHandler() {
    println!("set");
}

pub fn IsDebuggerPresent() -> bool {
    unsafe {
        //kernel32::DebugBreak();
        kernel32::IsDebuggerPresent() != 0
    }
}
