

use platform::{FEvent, GAutoResetFEvent, GManualResetFEvent};

pub struct FSafeEvent{
    pub innerEvent: FEvent,
}
impl Drop for FSafeEvent{
    fn drop(&mut self) {
        self.innerEvent.Reset();
    }
}

struct FEventPool
{
}
impl FEventPool{
    pub fn GetEventFromPool(bIsManualReset : bool) -> FSafeEvent{
        match bIsManualReset{
            true => FSafeEvent{ innerEvent: GManualResetFEvent},
            false => FSafeEvent{ innerEvent: GAutoResetFEvent},
        }
    }

}

