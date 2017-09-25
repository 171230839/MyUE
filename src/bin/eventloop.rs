use errors::{RueResult,RueError, RustUEError};
use Flags;

pub struct FEngineLoop{

}

impl FEngineLoop{
    pub fn new() -> Self{
        FEngineLoop{

        }
    }


    pub fn preinit(&self, flags: &Flags) -> RueResult {
        // if true{
        //     return Err(RueError::from(RustUEError::from("preinit error")));
        // }

        Ok(())
    }


    pub fn init(&self) -> RueResult{

        Ok(())
    }

    pub fn tick(&self){
       
    }

    pub fn exit(&self) {

    }
}