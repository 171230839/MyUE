



pub struct FPlatformProperties{

}

impl FPlatformProperties{

    pub fn SupportsWindowedMode() -> bool{
        true
    }

    pub fn IsProgram() -> bool{
        cfg!(IS_PROGRAM)
    }
}