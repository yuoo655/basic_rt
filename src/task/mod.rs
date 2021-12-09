use super::runtime::*;
use alloc::sync::Arc;
use alloc::boxed::Box;
use core::future::Future;
use core::pin::Pin;
use core::sync::atomic::{AtomicUsize, Ordering};
use alloc::collections::btree_map::BTreeMap;
use alloc::collections::VecDeque;

use core::task::{Context, Poll};
use core::mem;
use spin::Mutex;
use woke::waker_ref;
use lazy_static::*;



lazy_static! {
    pub static ref USER_TASK_QUEUE: Arc<Mutex<Box<UserTaskQueue>>> =
        Arc::new(
            Mutex::new(
                Box::new(
                    UserTaskQueue {
                        queue: VecDeque::new()
                    }
                )
            )
        );
}


pub fn thread_mian() {
    loop {
        let mut queue = USER_TASK_QUEUE.lock();
        let task = queue.peek_task();
        match task {
            // have any task
            Some(task) => {
                let mywaker = task.clone();
                let waker = waker_ref(&mywaker);
                let mut context = Context::from_waker(&*waker);

                let r = task.reactor.clone();
                let mut r = r.lock();

                if r.is_ready(task.id) {
                    let mut future = task.future.lock();
                    match future.as_mut().poll(&mut context) {
                        Poll::Ready(_) => {
                            // 任务完成
                            r.finish_task(task.id);
                        }
                        Poll::Pending => {
                            r.add_task(task.id);
                        }
                    }
                } else if r.contains_task(task.id) {
                    r.add_task(task.id);
                } else {
                    let mut future = task.future.lock();
                    match future.as_mut().poll(&mut context) {
                        Poll::Ready(_) => {
                            // // 任务完成
                            // println!("task completed");
                        }
                        Poll::Pending => {
                            r.register(task.id);
                        }
                    }
                }
            }
            None => return
        }
    }
}


//执行器
pub fn run() {
    loop {
        let mut queue = USER_TASK_QUEUE.lock();
        let task = queue.peek_task();
        match task {
            // have any task
            Some(task) => {
                let mywaker = task.clone();
                let waker = waker_ref(&mywaker);
                let mut context = Context::from_waker(&*waker);

                let r = task.reactor.clone();
                let mut r = r.lock();

                if r.is_ready(task.id) {
                    let mut future = task.future.lock();
                    match future.as_mut().poll(&mut context) {
                        Poll::Ready(_) => {
                            // 任务完成
                            r.finish_task(task.id);
                        }
                        Poll::Pending => {
                            r.add_task(task.id);
                        }
                    }
                } else if r.contains_task(task.id) {
                    r.add_task(task.id);
                } else {
                    let mut future = task.future.lock();
                    match future.as_mut().poll(&mut context) {
                        Poll::Ready(_) => {
                            // // 任务完成
                            // println!("task completed");
                        }
                        Poll::Pending => {
                            r.register(task.id);
                        }
                    }
                }
            }
            None => return
        }
    }
}