pub mod context;
pub mod thread;
pub mod user_stack;

use thread::Thread;

pub mod fifo;

use alloc::boxed::Box;
use alloc::{sync::Arc};
use lazy_static::*;
use spin::Mutex;


use fifo::THREAD_MANAGER;
pub type Tid = usize;


pub mod scheduler;
pub mod thread_pool;
pub mod processor;

use processor::Processor;
use scheduler::FifoScheduler;
use thread_pool::ThreadPool;

pub static CPU : Processor = Processor::new();


use crate::task::thread_mian;

use crate::println;

pub fn init() {
    // 使用 Round Robin Scheduler
    let scheduler = FifoScheduler::new();
    // 新建线程池
    let thread_pool = ThreadPool::new(100, Box::new(scheduler));

    // 新建内核线程 idle ，其入口为 coroutine::thread_mian
    let idle = Thread::new_box_thread(Processor::idle_main as usize, &CPU as *const Processor as usize);


    // 我们需要传入 CPU 的地址作为参数
    // 初始化 CPU
    CPU.init(idle, Box::new(thread_pool));

    // 新建一个thread_main加入线程池
    
    CPU.add_thread({
        let thread = Thread::new_box_thread(thread_mian as usize, 1);
        thread
    });

}





pub fn add_to_thread_pool(addr: usize, space_id:usize) {
    THREAD_MANAGER.lock().add(
        {
            let thread = Thread::new_thread(addr, space_id);
            thread
        }
    );
}


