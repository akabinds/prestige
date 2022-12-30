use alloc::{boxed::Box, collections::BTreeMap, sync::Arc, task::Wake};
use core::{
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicU64, Ordering},
    task::{Context, Poll, Waker},
};
use crossbeam::queue::ArrayQueue;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SCHEDULER: Scheduler = Scheduler::new();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct TaskId(u64);

impl TaskId {
    fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        TaskId(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub(crate) enum TaskPriority {
    Low = 0,
    Mediocre = 1,

    #[default]
    Medium = 2,

    Boosted = 3,
    High = 4,
    Critical = 5,
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub(crate) enum TaskStatus {
    #[default]
    New = 0,

    Running = 1,
    Waiting = 2,
    Ready = 3,
    Terminated = 4,
}

pub struct Task {
    id: TaskId,
    priority: TaskPriority,
    status: TaskStatus,
    future: Option<Pin<Box<dyn Future<Output = ()>>>>,
}

impl Task {
    pub fn new() -> Self {
        Self {
            id: TaskId::new(),
            priority: TaskPriority::default(),
            status: TaskStatus::default(),
            future: None,
        }
    }

    pub fn new_async(future: impl Future<Output = ()> + 'static) -> Self {
        Self {
            id: TaskId::new(),
            priority: TaskPriority::default(),
            status: TaskStatus::default(),
            future: Some(Box::pin(future)),
        }
    }

    fn poll(&mut self, ctx: &mut Context) -> Option<Poll<()>> {
        if let Some(mut fut) = self.future.take() {
            return Some(fut.as_mut().poll(ctx));
        }

        None
    }
}

struct TaskWaker {
    task_id: TaskId,
    task_queue: Arc<ArrayQueue<TaskId>>,
}

impl TaskWaker {
    fn as_waker(task_id: TaskId, task_queue: Arc<ArrayQueue<TaskId>>) -> Waker {
        Waker::from(Arc::new(TaskWaker {
            task_id,
            task_queue,
        }))
    }

    fn wake_task(&self) {
        self.task_queue.push(self.task_id).expect("task_queue full");
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.wake_task();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_task();
    }
}

pub struct Scheduler {
    tasks: BTreeMap<TaskId, Task>,
    task_queue: Arc<ArrayQueue<TaskId>>,
    waker_cache: BTreeMap<TaskId, Waker>,
}

impl Scheduler {
    fn new() -> Self {
        Self {
            tasks: BTreeMap::new(),
            task_queue: Arc::new(ArrayQueue::new(100)),
            waker_cache: BTreeMap::new(),
        }
    }

    pub fn spawn(&mut self, task: Task) {
        let task_id = task.id;

        if self.tasks.insert(task.id, task).is_some() {
            panic!("task with same ID already in tasks");
        }

        self.task_queue.push(task_id).expect("queue full");
    }
}

unsafe impl Send for Scheduler {}
unsafe impl Sync for Scheduler {}
