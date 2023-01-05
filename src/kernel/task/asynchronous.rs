use super::{AsyncTask, LocalAsyncTask, TaskId};
use alloc::{rc::Rc, sync::Arc};
use core::task::{Context, Waker};
use crossbeam::queue::ArrayQueue;
use futures::{stream::FuturesUnordered, task::ArcWake};

#[derive(Clone)]
struct LocalSpawner {
    task_queue: Rc<ArrayQueue<TaskId>>,
}

pub struct LocalExecutor {}
