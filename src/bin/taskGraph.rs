use platform::FPlatformMisc;
use platform::FPlatformProcess;
use std::cmp;
use platform::FPlatformTLS;

use logger::{ue_log, ELogVerbosity};
//use std::rc::{Rc,Weak};
//use std::cell::RefCell;
use platformAffinity::{FPlatformAffinity, EThreadPriority};
use platform::FPlatformRunnableThread;
use std::ptr::{null, null_mut};
use threadManager::{FThreadManager, GFThreadManager};
use errors::{RustUEError, RustResult};
use winapi::c_void;
use FThreadSafeCounter;
use platform::{INFINITE};
use std::sync::{Mutex, Arc, RwLock, Weak};
use eventloop::{GTaskGraph};
use std::thread;
use FRandomStream;
use std::time::Duration;
use std::collections::LinkedList;

pub enum Thread{
    max_threads = 24,
    max_thread_priorites = 3,
}

#[derive(Debug, Clone)] 
pub enum ENamedThreads{
    UnusedAnchor = -1,
    RHIThread = 0,
    AudioThread,
    GameThread,
    ActualRenderingThread,
    AnyThread,
  
}

impl Default for ENamedThreads{
    fn default() -> ENamedThreads{
        ENamedThreads::UnusedAnchor
    }
}

impl From<i32> for ENamedThreads {
    fn from(i: i32) ->ENamedThreads{
       // println!("from --------- {}", i);
        match i{
            -1 => ENamedThreads::UnusedAnchor,
            0 => ENamedThreads::RHIThread,
            1 => ENamedThreads::AudioThread,
            2 => ENamedThreads::GameThread,
            3 => ENamedThreads::ActualRenderingThread,
            _ => ENamedThreads::AnyThread,
           // _ => ENamedThreads::None,
        }
    }
}


bitflags! {
    #[derive(Default)]
struct ENamedThreadsBit :u16{
    const MainQueue =			0x000;
	const LocalQueue =		0x100;

	const NumQueues =			2;
	const ThreadIndexMask =	0xff;
    const QueueIndexMask =	0x100;
	const	QueueIndexShift =	8;

		/** High bits are used for a queue index task priority and thread priority**/

	const	NormalTaskPriority =	0x000;
	const	HighTaskPriority =		0x200;

	const	NumTaskPriorities =		2;
	const	TaskPriorityMask =		0x200;
	const	TaskPriorityShift =		9;

	const	NormalThreadPriority = 0x000;
	const	HighThreadPriority = 0x400;
	const	BackgroundThreadPriority = 0x800;

	const	NumThreadPriorities = 3;
	const	ThreadPriorityMask = 0xC00;
	const	ThreadPriorityShift = 10;

     const   GameThread_Local = ENamedThreads::GameThread as u16 | ENamedThreadsBit::LocalQueue.bits;
	const	ActualRenderingThread_Local = ENamedThreads::ActualRenderingThread as u16| ENamedThreadsBit::LocalQueue.bits;

	const	AnyHiPriThreadNormalTask = ENamedThreads::AnyThread  as u16 | ENamedThreadsBit::HighThreadPriority.bits | ENamedThreadsBit::NormalTaskPriority.bits;
	const	AnyHiPriThreadHiPriTask = ENamedThreads::AnyThread as u16 | ENamedThreadsBit::HighThreadPriority.bits | ENamedThreadsBit::HighTaskPriority.bits;

	const	AnyNormalThreadNormalTask = ENamedThreads::AnyThread as u16 | ENamedThreadsBit::NormalThreadPriority.bits | ENamedThreadsBit::NormalTaskPriority.bits;
	const	AnyNormalThreadHiPriTask = ENamedThreads::AnyThread as u16 | ENamedThreadsBit::NormalThreadPriority.bits | ENamedThreadsBit::HighTaskPriority.bits;

	const	AnyBackgroundThreadNormalTask = ENamedThreads::AnyThread as u16 | ENamedThreadsBit::BackgroundThreadPriority.bits | ENamedThreadsBit::NormalTaskPriority.bits;
	const	AnyBackgroundHiPriTask = ENamedThreads::AnyThread as u16 | ENamedThreadsBit::BackgroundThreadPriority.bits | ENamedThreadsBit::HighTaskPriority.bits;
}
}

impl ENamedThreads{
    pub fn bHasBackgroundThreads() ->bool{
        cfg!(PLATFORM_XBOXONE) || cfg!(PLATFORM_PS4)
    }
    pub fn bHasHighPriorityThreads() -> bool{
            cfg!(PLATFORM_XBOXONE) || cfg!(PLATFORM_PS4)
    }
}

#[derive(Debug, Clone)]
pub struct FThreadTaskQueue{
    quitWhenIdle : FThreadSafeCounter,
    recursionGuard: u32,
    //stallRestartEvent: Arc<Mutex<FEvent>>,
}

impl FThreadTaskQueue{
    pub fn new() -> Self{
        FThreadTaskQueue{
            quitWhenIdle: FThreadSafeCounter::new(),
            recursionGuard: 0,
         //   stallRestartEvent: GetEvent(true),
        }
    }
}

#[derive(Debug, Clone)]
struct TaskData{
    priority: i32,
    queue : FThreadTaskQueue,
}

impl TaskData{
    pub fn new(inPrioryty: i32) ->Self{
        TaskData{
            priority: inPrioryty,
            queue: FThreadTaskQueue::new(),
        }
    }
}
#[derive(Debug, Clone)]
struct NamedTaskData{

}

impl NamedTaskData{
    pub fn new() ->Self{
        NamedTaskData{

        }
    }
}

#[derive(Debug, Clone)]
enum FTaskThreadType{
    FNamedTaskThread(NamedTaskData),
    FTaskThreadAnyThread(TaskData),
}

impl FTaskThreadType{
    pub fn run(&self){
        match self{
             &FTaskThreadType::FNamedTaskThread(ref  namedThread) =>{
             },
             &FTaskThreadType::FTaskThreadAnyThread(ref  task) =>{
                 task.queue.quitWhenIdle.reset();
                while task.queue.quitWhenIdle.get() == 0 {
                    self.processTasks();
                }
             },
        }
    }

    pub fn processTasks(&self){

    }
}

#[derive(Debug)]
pub struct FTaskThread{
    taskType : FTaskThreadType,
    threadId: ENamedThreads,
    perThreadIDTLSSlot: u32,
    pub ownerWorker : Option<Weak<RwLock<FWorkerThread>>>,

}

impl FTaskThread{

    fn new( inType: FTaskThreadType, FTaskThreadinThreadId : ENamedThreads, inPerThreadIDTLSSlot: u32) -> Self{
           FTaskThread {
            taskType: inType,
            threadId : FTaskThreadinThreadId ,
            perThreadIDTLSSlot: inPerThreadIDTLSSlot,
            ownerWorker: None,
        }
    }

    pub fn initializeForCurrentThread(&self) {
         match self.ownerWorker.clone(){
            Some(weak) => {
                match weak.upgrade() {
                    Some(strong) => {
                        unsafe{
                            println!("init");
                            unsafe{
                            let mut tempL : *mut RwLock<FWorkerThread> = Arc::into_raw(strong) as *mut RwLock<FWorkerThread>;
                            let mut temp : &mut FWorkerThread = RwLock::get_mut(&mut *tempL).unwrap();
                            FPlatformTLS::SetTlsValue(self.perThreadIDTLSSlot, temp as *mut FWorkerThread as *mut c_void);
                            }
                        }
                        
                    },
                  //  None => {  return Err(RustUEError::from("get ownerWorker strong reference fail"));},
                  None => { UE_LOG!(FTaskThread, Fatal, "get ownerWorker strong reference fail");}
                };
                
            },
          //  None => { return Err(RustUEError::from("unwrap ownerWorker get none"));},
          None => { UE_LOG!(FTaskThread, Fatal, "unwrap ownerWorker get none");}
         }
       
    }

    pub fn init(&self){
        self.initializeForCurrentThread()
    }

    pub fn run(&self) ->RustResult{
        self.processTasksUntilQuit();
        //self.taskType.run();
        Ok(())
    }

    pub fn exit(&self) {

    }

    pub fn processTasksUntilQuit(&self){
         match self.taskType.clone(){
            FTaskThreadType::FNamedTaskThread(ref data) => {

            },
            FTaskThreadType::FTaskThreadAnyThread(task) => {
                // if priorityIndex != (ENamedThreadsBit::BackgroundThreadPriority >> ENamedThreads::ThreadPriorityShift){

                // } 
              
                task.queue.quitWhenIdle.reset();
                while task.queue.quitWhenIdle.get() == 0 {
                    self.processTasks(&task.queue);
                }
            },

        }
    }

    pub fn processTasks(&self, queue: &FThreadTaskQueue) {
       // let stallStatId = TStatId::new();
        let bCountAsStall: bool = true;
       // assert!(queue.recursionGuard == 1);
        while true{
            let task = GTaskGraph.write().unwrap().findWork(self.threadId.clone());
            if task.is_none(){
                //stall(statllStatId, bCountAsStall);
            }
        }
       // assert!(--queue.recursionGuard == 0);
    }
}
#[derive(Debug)]
pub struct FRunnableThread{
    runnableThread: Option<Arc<RwLock<FPlatformRunnableThread>>>,
}

impl FRunnableThread{
    pub fn new(task: Weak<RwLock<FTaskThread>>, name: &String, stackSize: u64, threadPri:&EThreadPriority, mask: u64) ->Self{
        let mut thread = Arc::new(RwLock::new(FPlatformRunnableThread::new(task, name, mask)));
        if thread.write().unwrap().createInternel(stackSize , threadPri ) == false{
            FRunnableThread{
                runnableThread :   None ,
            }
        }else {
            GFThreadManager.lock().unwrap().addThread(thread.read().unwrap().getThreadId(), thread.clone());
             FRunnableThread{
                runnableThread :   Some(thread) ,
            }
        }
       
    }
}

#[derive(Debug)]
pub struct FWorkerThread{
    pub taskGraphWorker : Arc<RwLock<FTaskThread>>,
    pub runnableThread: Option<FRunnableThread>,
    pub bAttached: bool,
}

impl FWorkerThread{
    pub fn new( task :  Arc<RwLock<FTaskThread>>) -> Self{
        FWorkerThread{
            taskGraphWorker: task,
            bAttached: false,
            runnableThread: None,
        }
    }
}

unsafe impl Sync for FWorkerThread{}
unsafe impl Send for FWorkerThread{}


#[derive(Default)]
pub struct FTaskGraph{
    
    workerThreads: Vec<Arc<RwLock<FWorkerThread>>>,
    numThreads: i32,
    numNamedThreads: i32,
    numTaskThreadSets : i32,
    numTaskThreadsPerSet: i32,
    
    lastExternalThread: ENamedThreads,

    IncomingAnyThreadTasks: [LinkedList<Box<FGraphTask>>; Thread::max_thread_priorites as usize],
    IncomingAnyThreadTasksHiPri: [LinkedList<Box<FGraphTask>>; Thread::max_thread_priorites as usize],
    SortedAnyThreadTasks: [LinkedList<Box<FGraphTask>>; Thread::max_thread_priorites as usize],
    SortedAnyThreadTasksHiPri:  [LinkedList<Box<FGraphTask>>; Thread::max_thread_priorites as usize],
}

impl FTaskGraph{


    pub fn Startup(&mut self, numThreads: i32){

        let mut bCreatedHiPriorityThreads = ENamedThreads::bHasHighPriorityThreads();
        let mut bCreatedBackgroundPriorityThreads = ENamedThreads::bHasBackgroundThreads();

        let mut maxTaskThreads :i32 = Thread::max_threads as i32;
        let mut numTaskThreads = FPlatformMisc::NumberOfWorkerThreadsToSpawn();
        // if !FPlatformProcess::SupportsMultithreading(){
        //     maxTaskThreads = 1;
        //     numTaskThreads = 1;
        //     self.lastExternalThread = ENamedThreads::GameThread;
        //     bCreatedBackgroundPriorityThreads = false;
        //     bCreatedHiPriorityThreads = false;

        // }else{
            self.lastExternalThread = ENamedThreads::ActualRenderingThread;
        // }


        self.numNamedThreads = self.lastExternalThread.clone()  as i32 + 1;
        self.numTaskThreadSets = 1 + bCreatedBackgroundPriorityThreads as i32 + bCreatedHiPriorityThreads as i32;

        assert!((self.numTaskThreadSets == 1) ||
         (cmp::min(self.numNamedThreads * self.numTaskThreadSets + self.numNamedThreads, maxTaskThreads ) == 
         (self.numThreads * self.numTaskThreadSets + self.numNamedThreads)));
    
        self.numThreads = cmp::max(cmp::min(numTaskThreads * self.numTaskThreadSets + self.numNamedThreads, maxTaskThreads), self.numNamedThreads + 1);
        println!("Numthreads: {} {} {}", self.numThreads, numTaskThreads, self.numTaskThreadSets);
        self.numTaskThreadsPerSet = (self.numThreads - self.numNamedThreads) / self.numTaskThreadSets;
        UE_LOG!(LogTaskGraph, Log, &format!("Started task graph with {} named threads and {} total threads with {} sets of task threads.", self.numNamedThreads, self.numThreads, self.numTaskThreadSets));
		
        let perThreadIDTLSSlot = FPlatformTLS::AllocTlsSlot();

    /*
        for threadIndex in 0..self.numThreads{
         //   println!("threadIndex: {}", threadIndex);
            let bAnyTaskThread = threadIndex >= self.numNamedThreads;
            if bAnyTaskThread{
                    let priority = self.ThreadIndexToPriorityIndex(threadIndex);
                  //  println!("priority: {}, threadIndex:{}", priority, threadIndex);
                   // let worker :FWorkerThread = FWorkerThread::new(FTaskThread::new(FTaskThreadType::FTaskThreadAnyThread(priority), ENamedThreads::from(threadIndex), perThreadIDTLSSlot));
                   let rcTask = Rc::new(RefCell::new(FTaskThread::new(FTaskThreadType::FTaskThreadAnyThread(priority), ENamedThreads::from(threadIndex), perThreadIDTLSSlot)));
                   let mut worker :FWorkerThread = FWorkerThread::new(rcTask.clone());
                  
                    let mut name = String::new();
                    let mut threadPri = EThreadPriority::TPri_Normal;
                    match priority{
                        1 => {
                            name = format!("TaskGraphThreadHP {}", threadIndex - self.numNamedThreads);
                            threadPri = EThreadPriority::TPri_SlightlyBelowNormal;
                        },
                        2 => {
                            name = format!("TaskGraphThreadBP {}", threadIndex - self.numNamedThreads);
                            threadPri = EThreadPriority::TPri_Lowest;
                        },
                        _ => {
                            name = format!("TashGraphThreadNp {}", threadIndex - self.numNamedThreads);
                            threadPri = EThreadPriority::TPri_BelowNormal;
                        },
                       
                    }
                  //  println!("priority: {} , name: {}", priority, name);
                    let stackSize:u64 = 384*1024;
                    worker.runnableThread = Some(FRunnableThread::new(Rc::downgrade(&rcTask), &name, stackSize, &threadPri, FPlatformAffinity::Mask as u64));
                    worker.bAttached = true;
                    let mut rc = Rc::new(RefCell::new(worker));
                  //let borrow = rc.borrow_mut().taskGraphWorker;
                   // borrow.borrow_mut().ownerWorker = Some(Rc::downgrade(&rc));
                    rcTask.borrow_mut().ownerWorker = Some(Rc::downgrade(&rc));
                 //   rc.borrow_mut().taskGraphWorker.borrow_mut().ownerWorker  = Some(Rc::downgrade(&rc));
                  // rc.borrow_mut().taskGraphWorker.ownerWorker  = Some(Rc::downgrade(&rc));
                  println!("rc: {:?} ", rc.clone());
                    self.workerThreads.push(rc);
                    
            }else{
                    let rcTask = Rc::new(RefCell::new(FTaskThread::new(FTaskThreadType::FNamedTaskThread, ENamedThreads::from(threadIndex), perThreadIDTLSSlot)));
                  //let worker :FWorkerThread = FWorkerThread::new(Rc::new(RefCell::new(FTaskThread::new(FTaskThreadType::FNamedTaskThread, ENamedThreads::from(threadIndex), perThreadIDTLSSlot))));
                   let worker :FWorkerThread = FWorkerThread::new(rcTask.clone());
                    let mut rc = Rc::new(RefCell::new(worker));
                    rcTask.borrow_mut().ownerWorker  = Some(Rc::downgrade(&rc));

                    self.workerThreads.push(rc);
            }
        
        }
    */
         for threadIndex in 0..self.numThreads{
            
            let bAnyTaskThread = threadIndex >= self.numNamedThreads;
            if bAnyTaskThread{
                let priority = self.ThreadIndexToPriorityIndex(threadIndex);
                let rcTask = Arc::new(RwLock::new(FTaskThread::new(FTaskThreadType::FTaskThreadAnyThread(TaskData::new(priority)), ENamedThreads::from(threadIndex), perThreadIDTLSSlot)));
                let mut worker :FWorkerThread = FWorkerThread::new(rcTask.clone());
                let mut rc = Arc::new(RwLock::new(worker));
                rcTask.write().unwrap().ownerWorker  = Some(Arc::downgrade(&rc));
                self.workerThreads.push(rc);
            }else{
               let rcTask = Arc::new(RwLock::new(FTaskThread::new(FTaskThreadType::FNamedTaskThread(NamedTaskData::new()), ENamedThreads::from(threadIndex), perThreadIDTLSSlot)));
                let mut worker :FWorkerThread = FWorkerThread::new(rcTask.clone());
                let mut rc = Arc::new(RwLock::new(worker));
                rcTask.write().unwrap().ownerWorker  = Some(Arc::downgrade(&rc));
                self.workerThreads.push(rc);
            }
         }

         for threadIndex in self.numNamedThreads..self.numThreads{
               let priority = self.ThreadIndexToPriorityIndex(threadIndex);
               let mut name = String::new();
               let mut threadPri = EThreadPriority::TPri_Normal;
                    match priority{
                        1 => {
                            name = format!("TaskGraphThreadHP {}", threadIndex - self.numNamedThreads);
                            threadPri = EThreadPriority::TPri_SlightlyBelowNormal;
                        },
                        2 => {
                            name = format!("TaskGraphThreadBP {}", threadIndex - self.numNamedThreads);
                            threadPri = EThreadPriority::TPri_Lowest;
                        },
                        _ => {
                            name = format!("TashGraphThreadNp {}", threadIndex - self.numNamedThreads);
                            threadPri = EThreadPriority::TPri_BelowNormal;
                        },
                       
                    }
                let stackSize:u64 = 384*1024;
                let workerRc = self.workerThreads[threadIndex as usize].clone();
           

                 // No1  style ---------
                    let worker : &mut FWorkerThread = &mut workerRc.write().unwrap();
                        let rcTask : Arc<RwLock<FTaskThread>> = worker.taskGraphWorker.clone();
                        worker.runnableThread = Some(FRunnableThread::new(Arc::downgrade(&rcTask), &name, stackSize, &threadPri, FPlatformAffinity::Mask as u64));
                        worker.bAttached = true; 
                //  No2 style -------------
                    // unsafe{
                    //      let mut workerPtr : *mut FWorkerThread =  (*Rc::into_raw(workerRc)).as_ptr();
                    //     let mut worker : &mut FWorkerThread = &mut *workerPtr;
                    //      let rcTask : Rc<RefCell<FTaskThread>> = worker.taskGraphWorker.clone();
                    //     worker.runnableThread = Some(FRunnableThread::new(Rc::downgrade(&rcTask), &name, stackSize, &threadPri, FPlatformAffinity::Mask as u64));
                    //     worker.bAttached = true;
                    // }
                // No3 style ------------
                    //   unsafe{
                    //      let mut worker : *mut FWorkerThread =  (*Rc::into_raw(workerRc)).as_ptr();
                    //      let rcTask : Rc<RefCell<FTaskThread>> = (*worker).taskGraphWorker.clone();
                    //      (*worker).runnableThread = Some(FRunnableThread::new(Rc::downgrade(&rcTask), &name, stackSize, &threadPri, FPlatformAffinity::Mask as u64));
                    //      (*worker).bAttached = true;
                    // }                   
                //  ---------------
                  
         }


        // for threadIndex in 0..self.numThreads{
        //      println!("WorkerRc: {} {:?}", threadIndex, self.workerThreads[threadIndex as usize].clone());  
        // }
    
    }

    pub fn ThreadIndexToPriorityIndex(&self, threadIndex: i32) -> i32{
        assert!(threadIndex >= self.numNamedThreads);
        assert!(threadIndex < self.numThreads);
       // println!("ThreadIndexToPriorityIndex  -- threadIndex : {}, self.numNameThreads: {}, perset: {}", threadIndex, self.numNamedThreads, self.numTaskThreadsPerSet);
        let result = (threadIndex - self.numNamedThreads) / self.numTaskThreadsPerSet;
        assert!(result >= 0 && result < self.numTaskThreadSets);
        result
    }

    pub fn findWork(&mut self, threadId: ENamedThreads) -> Option<Box<FGraphTask>> {
        let localNumWorkingThread = self.getNumWorkerThreads();
        let threadIdInt = threadId as i32;
        let myIndex = (threadIdInt - self.numNamedThreads) % self.numTaskThreadsPerSet;
        let priority = (threadIdInt - self.numNamedThreads) / self.numTaskThreadsPerSet;
        GChaoMode.delay();
        let mut task = self.SortedAnyThreadTasksHiPri[priority as usize].pop_front();
        if task.is_some(){
            return task
        }
        if !self.IncomingAnyThreadTasksHiPri[priority as usize].is_empty(){

        }
        task = self.SortedAnyThreadTasks[priority as usize].pop_front();
        if task.is_some(){
            return task
        }
        
       
        None
    }

    pub fn getNumWorkerThreads(&self) -> i32{
        (self.numThreads - self.numNamedThreads) / self.numTaskThreadSets
    }
}

lazy_static!{
    pub static ref GChaoMode: FChaosMode = FChaosMode::new();
}

const numSample : i32 = 45771;
pub struct FChaosMode{
    
    current: FThreadSafeCounter,
    delayTimes: Vec<f32>,
    enabled: bool,
}

impl FChaosMode{
    pub fn new() -> Self{
        let mut delayVec : Vec<f32> = Vec::new();
        let random = FRandomStream::new();
        for index in 0..numSample{
            delayVec.push(random.getFraction());
        }
        for cube in 0..2{
            for index in 0..numSample{
                delayVec[index as usize] *= random.getFraction();
            }
        }
        for index in 0..numSample{
            delayVec[index as usize] *= 0.00001;
        }

        for zeros in 0..numSample/20 {
            let index = random.randHelper(numSample);
            delayVec[index as usize] = 0.0;
        }

        for zeros in 0..numSample/100{
            let index = random.randHelper(numSample);
            delayVec[index as usize] = 0.00005;
        }

        FChaosMode{
            current: FThreadSafeCounter::new(),
            delayTimes:delayVec,
            enabled: false,
        }
    }

    pub fn delay(&self) {
        if self.enabled{
            let mut myIndex = self.current.increment();
            myIndex %= numSample;
            let delayS = self.delayTimes[myIndex as usize];
            if delayS > 0.0{
                    thread::sleep(Duration::new(delayS as u64, 0));
            }
        }
    }
}


pub struct FGraphTask{

}

impl FGraphTask{
    pub fn new()->Self{
        FGraphTask{
            
        }
    }
}