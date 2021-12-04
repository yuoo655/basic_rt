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




pub fn add_to_thread_pool(addr: usize, space_id:usize) {
    THREAD_MANAGER.lock().add(
        {
            let thread = Thread::new_thread(addr, space_id);
            thread
        }
    );
}


// #[no_mangle]
// pub extern "C" fn hello_thread(arg: usize){
//     println!("begin of thread {}", arg);
//     for i in 0..10 {
//         print!("{}", arg);
//     }
//     println!("\nend  of thread {}", arg);
// }

