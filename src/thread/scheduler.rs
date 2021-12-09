use super::Thread;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use spin::Mutex;
use lazy_static::*;
use super::Tid;
use alloc::vec::Vec;

pub trait Scheduler {
    // 如果 tid 不存在，表明将一个新线程加入线程调度
    // 否则表明一个已有的线程要继续运行
    fn push(&mut self, tid: Tid);
    
    // 从若干可运行线程中选择一个运行
    fn pop(&mut self) -> Option<Tid>;

    // 时钟中断中，提醒调度算法当前线程又运行了一个 tick
    // 返回的 bool 表示调度算法认为当前线程是否需要被切换出去
    fn tick(&mut self) -> bool;

    // 告诉调度算法一个线程已经结束
    fn exit(&mut self, tid: Tid);
}



#[derive(Clone)]
pub struct FifoInfo{
    valid: bool,
}

pub struct FifoScheduler {
    ready_queue: Vec<FifoInfo>,
    current: usize,
}

impl Scheduler for FifoScheduler {

    fn push(&mut self, tid: Tid) {

        let tid = tid + 1;

        if tid + 1 > self.ready_queue.len() {
            self.ready_queue.resize(tid + 1, FifoInfo{valid: false});
        }

    }

    fn pop(&mut self) -> Option<Tid> {
        let ret = self.current;
        if ret != 0 {
            Some(self.current)
        }else {
            None
        }
    }

    // 当前线程的可用时间片 -= 1
    fn tick(&mut self) -> bool{
        false
    }

    fn exit(&mut self, tid : Tid) {
        let tid = tid + 1;
        if self.current == tid {
            self.current = 0;
        }
    }

}



impl FifoScheduler {
    pub fn new() -> Self {

        let mut rr = FifoScheduler {
            ready_queue: Vec::default(),
            current: 0,
        };

        rr.ready_queue.push(
            FifoInfo {
                valid: false,
            }
        );
        rr
    }
}









