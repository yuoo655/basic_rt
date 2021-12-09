#![no_std]

use alloc::boxed::Box;
use alloc::sync::Arc;
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

#[no_mangle]
lazy_static! {
    pub static ref REACTOR: Arc<Mutex<Box<Reactor>>> = Reactor::new();
}



#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash, Ord, PartialOrd)]
pub struct TaskId(usize);

impl TaskId {
    pub(crate) fn generate() -> TaskId {
        // 任务编号计数器，任务编号自增
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        if id > usize::MAX / 2 {
            // TODO: 不让系统 Panic
            panic!("too many tasks!")
        }
        TaskId(id)
    }
}



//Task包装协程
pub struct UserTask{
    // 任务编号
    pub id: TaskId,
    // future
    pub future: Mutex<Pin<Box<dyn Future<Output=()> + 'static + Send + Sync>>>, 
    // reactor
    pub reactor: Arc<Mutex<Box<Reactor>>>,
}

impl UserTask{
    //创建一个协程
    pub fn spawn(future: Mutex<Pin<Box<dyn Future<Output=()> + 'static + Send + Sync>>>) -> Self{
        UserTask{
            id: TaskId::generate(),
            future: future,
            reactor: REACTOR.clone(),
        }
    }

    pub fn do_wake(self: &Arc<Self>) {
        // todo!()
    }
}


impl Future for UserTask {
    type Output = usize;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut r = self.reactor.lock();
        if r.is_ready(self.id) {
            Poll::Ready(self.id.0)
        } else if r.contains_task(self.id) {
            r.add_task(self.id);
            Poll::Pending
        } else {
        let mut f = self.future.lock();
        match f.as_mut().poll(cx) {
            Poll::Ready(_) => {
                Poll::Ready(0)
            },
            Poll::Pending => {
                r.register(self.id); // fixme
                Poll::Pending
            }
        }

        }
    }
}



//用户协程队列
pub struct UserTaskQueue {
    pub queue: VecDeque<Arc<UserTask>>,
}

impl UserTaskQueue {
    pub fn add_task(&mut self, task: UserTask) {
        self.queue.push_front(Arc::new(task));
        // println!("queue len:{:?}", self.queue.len());
    }

    pub fn add_arc_task(&mut self, task: Arc<UserTask>) {
        self.queue.push_back(task);
    }

    pub fn peek_task(&mut self) -> Option<Arc<UserTask>> {
        self.queue.pop_front()
    }

    pub fn delete_task(&mut self, id: TaskId) {
        let index = self.queue.iter().position(|task| task.id == id).unwrap();
        self.queue.remove(index);
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}


pub enum TaskState {
    Ready,
    NotReady,
    Finish,
}

pub struct Reactor {
    tasks: BTreeMap<TaskId, TaskState>,
}

impl Reactor {
    pub(crate) fn new() -> Arc<Mutex<Box<Self>>> {
        let reactor = Arc::new(Mutex::new(Box::new(Reactor {
            tasks: BTreeMap::new(),
        })));
        reactor
    }

    pub(crate) fn wake(&mut self, id: TaskId) {
        let state = self.tasks.get_mut(&id).unwrap();
        match mem::replace(state, TaskState::Ready) {
            TaskState::NotReady => (),
            TaskState::Finish => panic!("Called 'wake' twice on task: {:?}", id),
            _ => unreachable!()
        }
    }

    pub(crate) fn register(&mut self, id: TaskId) {
        if self.tasks.insert(id, TaskState::NotReady).is_some() {
            panic!("Tried to insert a task with id: '{:?}', twice!", id);
        }
    }

    pub(crate) fn is_ready(&self, id: TaskId) -> bool {
        self.tasks.get(&id).map(|state| match state {
            TaskState::Ready => true,
            _ => false,
        }).unwrap_or(false)
    }

    pub(crate) fn get_task(&self, task_id: TaskId) -> Option<&TaskState> {
        self.tasks.get(&task_id)
    }

    pub(crate) fn get_task_mut(&mut self, task_id: TaskId) -> Option<&mut TaskState> {
        self.tasks.get_mut(&task_id)
    }

    pub(crate) fn add_task(&mut self, task_id: TaskId) -> Option<TaskState> {
        self.tasks.insert(task_id, TaskState::NotReady)
    }

    pub(crate) fn contains_task(&self, task_id: TaskId) -> bool {
        self.tasks.contains_key(&task_id)
    }

    pub(crate) fn is_finish(&self, task_id: TaskId) -> bool {
        self.tasks.get(&task_id).map(|state| match state {
            TaskState::Finish => true,
            _ => false,
        }).unwrap_or(false)
    }

    pub(crate) fn finish_task(&mut self, task_id: TaskId) {
        self.tasks.insert(task_id, TaskState::Finish);
    }

    pub(crate) fn remove_task(&mut self, task_id: TaskId) -> Option<TaskState>{
        self.tasks.remove(&task_id)
    }
}




impl woke::Woke for UserTask {
    fn wake_by_ref(task: &Arc<Self>) {
        task.do_wake()
    }
}

impl Drop for UserTask {
    fn drop(&mut self) {
        let r = self.reactor.clone();
        let mut r = r.lock();
        r.remove_task(self.id);
    }
}




//传递用户协程队列
pub fn diliver_to_kernel(){
    //to do
}


//检查kernel提供给用户的调度信息
pub fn check_kernel_clue(){
    //to do
    // println!("checking clue.");
}








