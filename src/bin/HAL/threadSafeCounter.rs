
use std::sync::Arc;
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

#[derive(Debug, Clone)]
pub struct FThreadSafeCounter{
    counter: Arc<AtomicI32>,
}

impl FThreadSafeCounter{
    pub fn new() -> Self{
        FThreadSafeCounter{
            counter: Arc::new(AtomicI32::new(0)),
        }
    }

    pub fn get(&self) -> i32{
        self.counter.clone().load(Ordering::SeqCst)
    }

    pub fn new_with(value : i32) -> Self {
        FThreadSafeCounter{
            counter: Arc::new(AtomicI32::new(value )),
        }
    }


    pub fn add(&self, value : i32)  -> i32{
        self.counter.clone().fetch_add( value, Ordering::SeqCst)
    }

    pub fn subtract(&self, value: i32)  -> i32{
        self.counter.clone().fetch_sub(value, Ordering::SeqCst)
    }

    pub fn increment(&self) -> i32{
        self.counter.clone().fetch_add( 1, Ordering::SeqCst)
    }

    pub fn decrement(&self, value: i32) -> i32{
        self.counter.clone().fetch_sub(1, Ordering::SeqCst)
    }

    pub fn set(&self, value: i32){
        self.counter.clone().store(value, Ordering::SeqCst);
    }

    pub fn reset(&self) {
        self.counter.clone().store(0, Ordering::SeqCst);
    }
}