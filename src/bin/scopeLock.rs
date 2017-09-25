
//use std::mem;
use platform::criticalSection::FCriticalSection;
//use std::sync::{Arc, Weak};
use std::rc::{Rc, Weak};
pub struct FScopeLock<'a> {
   // synchObject: *mut FCriticalSection,
  // synchObject: Weak<FCriticalSection>,
     synchObject: &'a FCriticalSection,
}


//impl FScopeLock {
impl <'a> FScopeLock<'a>{
    //pub fn new(inSyncObject: &FCriticalSection) -> FScopeLock{
    //pub fn new(inSyncObject: &Arc<FCriticalSection>) -> FScopeLock {
     //      pub fn new(inSyncObject: &Rc<FCriticalSection>) -> FScopeLock {
         pub fn new(inSyncObject: &'a FCriticalSection) -> Self{
        inSyncObject.Lock();
        // unsafe{
        //     let p: *mut FCriticalSection = mem::transmute(inSyncObject);
        //         FScopeLock{
        //     synchObject: p,
        // }
        FScopeLock {
            //synchObject: Arc::downgrade(inSyncObject),
          //  synchObject: Rc::downgrade(inSyncObject),
              synchObject: inSyncObject,
        }
    }
}

impl <'a> Drop for FScopeLock<'a> {
    fn drop(&mut self) {
        // unsafe{
        //     self.synchObject.as_ref().unwrap().Unlock();
        // }
       // self.synchObject.upgrade().unwrap().Unlock();
       self.synchObject.Unlock();
    }
}
