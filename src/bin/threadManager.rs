
use std::collections::HashMap;
use platform::FPlatformRunnableThread;
use std::sync::{Mutex};
use std::rc::{Rc, Weak};
use std::cell::RefCell;



lazy_static!{
    pub static ref GFThreadManager: Mutex<FThreadManager> = Mutex::new(FThreadManager::new());
}

pub struct FThreadManager{
    threadsMap : HashMap<u32, Rc<RefCell<FPlatformRunnableThread>>>,
}

unsafe impl Sync for FThreadManager{}
unsafe impl Send for FThreadManager{}

impl FThreadManager{

    pub fn new() -> Self{
        FThreadManager{
            threadsMap: HashMap::new(),
        }
    }

    pub fn addThread(&mut self, threadId: u32, thread: Rc<RefCell<FPlatformRunnableThread>>){
            self.threadsMap.insert(threadId, thread);
    }

    pub fn removeThread(&mut self, thread: Rc<RefCell<FPlatformRunnableThread>>){
       // self.threadsMap.remove(thread);
        let key: u32 = findKey(&self.threadsMap, thread);
        if key != 0{
            self.threadsMap.remove(&key);
        }
    }
    

    // pub fn tick(&mut self){
    //     for (key, val) in self.threadsMap.iter_mut(){
    //         val.tick();
    //     }
    // }

    pub fn getThreadName(&mut self, threadId: &u32) -> Option<String>{
        match self.threadsMap.get(threadId){
            Some(thread) => return Some(thread.borrow().getThreadName()),
            None => None
        }
    }
}


pub fn findKey(map: &HashMap<u32, Rc<RefCell<FPlatformRunnableThread>>>, value: Rc<RefCell<FPlatformRunnableThread>>) -> u32 {
    for (key, val) in map{
        if val == &value {
            return key.clone()
        }
    }
    0
}