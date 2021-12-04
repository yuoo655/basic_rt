use alloc::collections::VecDeque;
use alloc::sync::Arc;
use spin::Mutex;
use lazy_static::*;

pub trait Scheduler<T> {

    //向调度器添加任务
    fn push(&mut self, task: T);
    
    //从就绪队列取出任务
    fn pop(&mut self) -> Option<T>;

    //获取下一个任务,不弹出
    fn front(&mut self) -> Option<&T>;

    //获取下一个任务的可变引用,不弹出
    fn front_mut(&mut self) -> Option<&mut T>;

    // //任务退出
    // fn exit(&mut self, task: &T);
}



pub struct FifoScheduler<T> {
    ready_queue: VecDeque<T>,
}

impl<T> FifoScheduler<T>{
    pub fn new() -> Self {
        Self { ready_queue: VecDeque::new()}
    }
}


impl<T> Scheduler<T> for FifoScheduler<T>{

    fn push(&mut self, task: T){
        self.ready_queue.push_back(task);
    }

    fn pop(&mut self) -> Option<T>{
        self.ready_queue.pop_front()
    }

    fn front(&mut self) -> Option<&T>{
        self.ready_queue.front()
    }

    fn front_mut(&mut self) -> Option<&mut T>{
        self.ready_queue.front_mut()
    }

    // fn exit(&mut self, task: &T){
    //     self.ready_queue.remove(task)
    // }
}