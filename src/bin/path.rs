use platform::FPlatformMisc;

pub struct FPaths{

}

impl FPaths{

    pub fn GameAgnosticSavedDir() -> String{
        FPaths::EngineSavedDir()
    }

    pub fn EngineSavedDir() -> String{
        FPaths::EngineUserDir() + "/Saved/"
        
    }

    pub fn EngineUserDir() -> String{
        FPaths::EngineDir()
    }

    pub fn EngineDir() -> String{
        FPlatformMisc::EngineDir()
    }
}