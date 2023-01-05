pub mod asynchronous;
pub mod scheduler;

use alloc::task::Wake;
use core::{
    cell::UnsafeCell,
    sync::atomic::{AtomicU64, Ordering},
};
use crossbeam::atomic::AtomicCell;
use futures::task::{ArcWake, FutureObj, LocalFutureObj};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TaskId(u64);

impl TaskId {
    fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        TaskId(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

pub struct SyncTask {
    id: TaskId,
}

impl SyncTask {
    fn new() -> Self {
        Self { id: TaskId::new() }
    }
}

/// A non-thread-safe async task
pub struct LocalAsyncTask {
    id: TaskId,

    /// The `Option` stored  must be `Some(...)` as long as the future is not completed yet.
    future: UnsafeCell<Option<LocalFutureObj<'static, ()>>>,
}

impl LocalAsyncTask {
    fn new(future: LocalFutureObj<'static, ()>) -> Self {
        Self {
            id: TaskId::new(),
            future: UnsafeCell::new(Some(future)),
        }
    }
}

/// A thread-safe async task
pub struct AsyncTask {
    id: TaskId,

    /// The `Option` stored must be `Some(...)` as long as the future is not completed yet.
    future: AtomicCell<Option<FutureObj<'static, ()>>>,
}

impl AsyncTask {
    fn new(future: FutureObj<'static, ()>) -> Self {
        Self {
            id: TaskId::new(),
            future: AtomicCell::new(Some(future)),
        }
    }
}
