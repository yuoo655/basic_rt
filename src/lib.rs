#![no_std]
#![feature(llvm_asm)]
#![feature(asm)]
#![feature(naked_functions)]
#![feature(linkage)]
#![feature(alloc_error_handler)]
#![allow(unused)]
pub mod thread;
pub mod console;
pub mod lang_items;
pub mod runtime;
pub mod scheduler;
pub mod syscall;
pub mod init;

pub use thread::*;
pub use runtime::*;
pub use scheduler::*;

extern crate alloc;


#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {

    let mut space_id :usize;
    // unsafe {
    //     HEAP.lock()
    //         .init(HEAP_SPACE.as_ptr() as usize, USER_HEAP_SIZE);
    // }
    // unsafe{asm!("mv {}, tp", out(reg) space_id, options(nomem, nostack));}

    // println!(" space_id : {:#x}", space_id);
    exit( main());
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}




use syscall::*;
pub fn read(fd: usize, buf: &mut [u8]) -> isize { sys_read(fd, buf) }
pub fn write(fd: usize, buf: &[u8]) -> isize { sys_write(fd, buf) }
pub fn exit(exit_code: i32) -> ! { sys_exit(exit_code); }





use buddy_system_allocator::LockedHeap;
use alloc::vec::Vec;
const USER_HEAP_SIZE: usize = 32768;

static mut HEAP_SPACE: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

// #[global_allocator]
// static HEAP: LockedHeap = LockedHeap::empty();

// #[alloc_error_handler]
// pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
//     panic!("Heap allocation error, layout = {:?}", layout);
// }





use spin::Mutex;
use alloc::boxed::Box;

async fn foo(x:usize){
    println!("{:?}", x);
}

pub fn coroutine(){
    let mut queue = USER_TASK_QUEUE.lock();
    for i in 0..100_000_000 {
        queue.add_task(UserTask::spawn(Mutex::new(Box::pin(foo(i)))));
        if i % 10_000_000 == 0 {
            println!("count {:?}", i);
        }
    }
    drop(queue);
    // runtime::run();
}


pub fn fooo(){
    println!("---");
}

pub fn thread(){
    for i in 0..1000_000{
        add_to_thread_pool(fooo as usize,0);
        if i % 100_000 == 0 { println!("count {:?}", i); }
    }
}




