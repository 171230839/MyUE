
use taskGraph::FTaskThread;
use platformAffinity::{EThreadPriority, FPlatformAffinity};
//use std::rc::{Rc,Weak};
//use std::cell::RefCell;
use super::event::{GManualResetFEvent ,INFINITE};
use winapi::HANDLE;
use kernel32;
use std::ptr::{null, null_mut};

use super::FPlatformProcess;
use super::TLS::FPlatformTLS;
use winapi::c_void;
use logger::{ue_log, ELogVerbosity};
use std::mem::transmute;
use std::sync::{Arc, Mutex, Weak, RwLock};
use winapi;
use exit_with_error;
use errors::{RustUEError, RustResult};


#[derive(Debug)]
pub struct FPlatformRunnableThread{
    runnable: Weak<RwLock<FTaskThread>>,
    threadAffinityMask: u64,
   // threadInitSyncEvent: Arc<Mutex<FEvent>>,
    thread: HANDLE,
    threadId: u32,
    threadPriority: EThreadPriority,
    threadName: String,
}

unsafe impl Sync for FPlatformRunnableThread{}
unsafe impl Send for FPlatformRunnableThread{} 


static STACK_SIZE_PARAM_IS_A_RESERVATION  : u32 =  0x00010000;

lazy_static!{
    pub static ref RunnableTlsSlot: u32 = FPlatformRunnableThread::getTlsSlot();
}

unsafe extern "system"  fn threadProc( pSelf: *mut c_void) -> u32{
   // (*(pSelf as *mut FPlatformRunnableThread)).guardeRun()
   let t = transmute::<*mut c_void, *mut FPlatformRunnableThread>(pSelf);
   (*t).guardeRun()
}


impl FPlatformRunnableThread{
    pub fn new(task: Weak<RwLock<FTaskThread>>, name: &String, mask: u64) -> Self{

        FPlatformRunnableThread{
            runnable : task,
            threadAffinityMask: mask,
           // threadInitSyncEvent: GetEvent(true),
            thread: null_mut(),
            threadId:0,
            threadPriority: EThreadPriority::TPri_Normal,
            threadName: name.clone(),
        }
    }

    pub fn createInternel(&mut self, stackSize: u64, threadPri: &EThreadPriority ) ->bool{
        unsafe{
            self.thread = kernel32::CreateThread(null_mut(), stackSize, Some(threadProc), &*self as *const _ as *mut _ , STACK_SIZE_PARAM_IS_A_RESERVATION, transmute(&self.threadId) );
            if self.thread as u32 == 0{
           // UE_LOG!(Thread, Warning, "createThread failured");
                println!("createInternel failured");
                return false
            }else{
                //GFThreadManager.lock().unwrap().addThread(self.threadId, Rc::new(self));
                println!("threadInitSyncEvent wait||||");
               //GManualResetFEvent.lock().unwrap().Wait(INFINITE);
                GManualResetFEvent.Wait(INFINITE);
                println!("threadInitSyncEvent wait----");
                self.setThreadPriority(threadPri);
            }
            GManualResetFEvent.Reset();
            true
        }
       
    }

    pub fn guardeRun(&self) -> u32{
        println!("guardRun-----------");
        FPlatformProcess::SetThreadAffinityMask(self.threadAffinityMask);
        
        match self.run() {
         Err( e) => {
             exit_with_error(e);
                        return 1
                        },
         Ok(()) => {
             return 0
         },
     }
    }

    pub fn run(&self) -> RustResult{
        match self.runnable.upgrade() {
            Some(strong) => {
                println!("ssssssssssssss");
               // match strong.borrow_mut().init(){
               //     Ok(()) => {
                        strong.read().unwrap().init();
                        //println!("init()");
                        GManualResetFEvent.Trigger();
                        //GManualResetFEvent.lock().unwrap().Trigger();
                      //  println!("trigger");
                        self.setTls();
                        strong.read().unwrap().run()?;
                        strong.read().unwrap().exit();
                        self.freeTls();
             //       },
              //      Err(err) => {
              //          self.threadInitSyncEvent.Trigger();
              //          return Err(err);
              //      },
             //   }
            },
            None => {
               
                return Err(RustUEError::from("get runnable strong reference fail"));
            },
        }
        Ok(())
    }

    pub fn getTlsSlot() -> u32{
        FPlatformTLS::AllocTlsSlot()
    }

    pub fn setTls(&self){
    //   unsafe{
    //         FPlatformTLS::SetTlsValue(*RunnableTlsSlot,  transmute(self));
    //   }

    println!("setTls   {}", *RunnableTlsSlot);
     FPlatformTLS::SetTlsValue(*RunnableTlsSlot,&*self as *const _ as *mut _);
    }

    pub fn freeTls(&self){
        FPlatformTLS::SetTlsValue(*RunnableTlsSlot, null_mut() as *mut c_void);
    }


    pub fn getThreadName(&self) -> String{
        self.threadName.clone()
    }

    pub fn setThreadPriority(&mut self, newPriority: &EThreadPriority){
        if newPriority != &self.threadPriority{
            self.threadPriority = newPriority.clone();
            unsafe{
                kernel32::SetThreadPriority(self.thread, translateThreadPriority(&self.threadPriority));
       
            }
        }
    }

    pub fn getThreadId(&self) -> u32{
        self.threadId.clone()
    }
}

pub  fn translateThreadPriority(priority: &EThreadPriority) -> i32{
    match priority{
        &EThreadPriority::TPri_Normal => return winapi::winbase::THREAD_PRIORITY_NORMAL as i32,
        &EThreadPriority::TPri_AboveNormal => return winapi::winbase::THREAD_PRIORITY_ABOVE_NORMAL as i32,
        &EThreadPriority::TPri_BelowNormal => return winapi::winbase::THREAD_PRIORITY_BELOW_NORMAL as i32,
        &EThreadPriority::TPri_Highest => return winapi::winbase::THREAD_PRIORITY_HIGHEST as i32,
        &EThreadPriority::TPri_Lowest => return winapi::winbase::THREAD_PRIORITY_LOWEST as i32,
        &EThreadPriority::TPri_SlightlyBelowNormal => return winapi::winbase::THREAD_PRIORITY_NORMAL as i32 - 1,
    }
}

impl PartialEq for FPlatformRunnableThread{
    fn eq(&self, other: &FPlatformRunnableThread) ->bool{
        self.threadId == other.threadId
    }
}