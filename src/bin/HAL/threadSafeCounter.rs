
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

#[derive(Debug, Clone)]
pub struct FThreadSafeCounter{
    counter: Arc<AtomicUsize>,
}

impl FThreadSafeCounter{
    pub fn new() -> Self{
        FThreadSafeCounter{
            counter: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn get(&mut self) -> usize{
        self.counter.clone().load(Ordering::SeqCst)
    }

    pub fn new_with(value : usize) -> Self {
        FThreadSafeCounter{
            counter: Arc::new(AtomicUsize::new(value )),
        }
    }


    pub fn add(&mut self, value : usize) {
        self.counter.clone().fetch_add( value, Ordering::SeqCst);
    }

    pub fn subtract(&mut self, value: usize) {
        self.counter.clone().fetch_sub(value, Ordering::SeqCst);
    }

    pub fn increment(&mut self){
        self.counter.clone().fetch_add( 1, Ordering::SeqCst);
    }

    pub fn decrement(&mut self, value: usize) {
        self.counter.clone().fetch_sub(1, Ordering::SeqCst);
    }

    pub fn set(&mut self, value: usize){
        self.counter.clone().store(value, Ordering::SeqCst);
    }

    pub fn reset(&mut self) {
        self.counter.clone().store(0, Ordering::SeqCst);
    }
}