// use errors::{RueResult,RueError, RustUEError};
use errors::{RustResult, RustUEError};
use Flags;
use platform::FPlatformProcess;
use platformAffinity::FPlatformAffinity;
use platform::FPlatformTLS;
use GApp;
use projectManager::GIProjectManager;
use std::fs::File;
use platform::FPlatformMisc;
use taskGraph::FTaskGraph;
use std::sync::{Mutex, RwLock};

lazy_static!{
    pub static ref GTaskGraph : RwLock<FTaskGraph> = RwLock::new(Default::default());
}

pub struct FEngineLoop {}



impl FEngineLoop {
    pub fn new() -> Self {
        FEngineLoop {}
    }


    pub fn preinit(&self, flags: &Flags) -> RustResult {
        // if true{
        //     return Err(RueError::from(RustUEError::from("preinit error")));
        // }

        let GGameThreadId = FPlatformTLS::GetCurrentThreadId();
        let GIsGameThreadIdInitialized = true;
        // FPlatformProcess::SetThreadAffinityMask(FPlatformAffinity::Mask.bits());
        FPlatformProcess::SetThreadAffinityMask(FPlatformAffinity::Mask as u64);
        FPlatformProcess::SetupGameThread();
        if !GApp.lock().unwrap().HasGameName() {
            LaunchUpdateMostRecentProjectFile();
        }

        let mut taskGraph: FTaskGraph = Default::default();
        let core = FPlatformMisc::NumberOfCores();
        GTaskGraph.write().unwrap().Startup(core);
        //taskGraph.Startup(core);
        Ok(())
    }


    pub fn init(&self) -> RustResult {
        Ok(())
    }

    pub fn tick(&self) {}

    pub fn exit(&self) {}
}


fn LaunchUpdateMostRecentProjectFile() {
    let autoLoadProjectFileName = GIProjectManager
        .lock()
        .unwrap()
        .GetAutoLoadProjectFileName();
    println!("llll : {}", autoLoadProjectFileName);

    //  let mut file = File::create(autoLoadProjectFileName);
}
