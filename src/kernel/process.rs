#[cfg(target_arch = "x86_64")]
use super::arch::mem::allocator;

use super::{
    io::{console::Console, recoverable},
    resource::{Device, Resource},
    scheduler::{TaskId, TaskPriority, TaskStatus},
};
use alloc::{
    boxed::Box,
    collections::{BTreeMap, BTreeSet},
    string::String,
};
use bitflags::bitflags;
use core::sync::atomic::{AtomicU64, Ordering};
use lazy_static::lazy_static;
use spin::RwLock;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum ExitCode {
    Success = 0,
    GeneralFailure = 1,
    UsageFault = 60,
    DataFault = 61,
    WriteFault = 125,
    ReadFault = 126,
    OpenFault = 127,
    ExecFault = 128,
    PageFault = 129,
    SegFault = 130,
    ShellExit = 225,
}

impl From<usize> for ExitCode {
    fn from(code: usize) -> Self {
        use ExitCode::*;

        match code {
            0 => Success,
            60 => UsageFault,
            61 => DataFault,
            125 => WriteFault,
            126 => ReadFault,
            127 => OpenFault,
            128 => ExecFault,
            129 => PageFault,
            130 => SegFault,
            225 => ShellExit,
            _ => GeneralFailure,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ThreadId(u64);

impl ThreadId {
    fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        ThreadId(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Thread {
    id: ThreadId,
    proc: ProcessId, // PID of the process containing this thread
}

impl Thread {
    pub(crate) fn new(proc: ProcessId) -> Self {
        Self {
            id: ThreadId::new(),
            proc,
        }
    }

    pub(crate) const fn id(&self) -> ThreadId {
        self.id
    }
}

pub(crate) fn current_thread() -> Thread {
    let calling_proc = current_process();

    todo!();
}

const MAX_RESOURCE_HANDLES: usize = 64;
const MAX_THREADS: usize = 100;
const MAX_PROCESSES: usize = 50;
const MAX_PROC_SIZE: usize = 4 << 40;

static CODE_ADDR: AtomicU64 = AtomicU64::new(0);

lazy_static! {
    pub(crate) static ref PROCESSES: RwLock<[Box<Process>; MAX_PROCESSES]> =
        RwLock::new([(); MAX_PROCESSES].map(|_| Box::new(Process::new("/"))));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ProcessId(u64);

impl ProcessId {
    fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        ProcessId(COUNTER.fetch_add(1, Ordering::Relaxed))
    }

    pub(crate) const fn inner(&self) -> u64 {
        self.0
    }
}

bitflags! {
    /// The first 15 bits represent the privileges for everybody outside the current process.
    /// The next 15 bits represent the privileges for everybody in the same group as the current process.
    /// The final 15 bits represent the privileges for the owner of the current process
    pub(crate) struct ProcessPrivileges: u64 {
        const EVERYONE_READFILE = 1;
        const EVERYONE_WRITEFILE = 1 << 1;
        const EVERYONE_EXECFILE = 1 << 2;
        const EVERYONE_READDIR = 1 << 3;
        const EVERYONE_WRITEDIR = 1 << 4; // grants permission to modify directory entries
        const EVERYONE_EXECDIR = 1 << 5; // grants permission to access directory entries
        const EVERYONE_READBLOCK = 1 << 6;
        const EVERYONE_WRITEBLOCK = 1 << 7;
        const EVERYONE_EXECBLOCK = 1 << 8;
        const EVERYONE_READLINK = 1 << 9;
        const EVERYONE_WRITELINK = 1 << 10;
        const EVERYONE_EXECLINK = 1 << 11;
        const EVERYONE_READSOCKET = 1 << 12;
        const EVERYONE_WRITESOCKET = 1 << 13;
        const EVERYONE_EXECSOCKET = 1 << 14;

        const GROUP_READFILE = 1 << 15;
        const GROUP_WRITEFILE = 1 << 16;
        const GROUP_EXECFILE = 1 << 17;
        const GROUP_READDIR = 1 << 18;
        const GROUP_WRITEDIR = 1 << 19; // grants permission to modify directory entries
        const GROUP_EXECDIR = 1 << 20; // grants permission to access directory entries
        const GROUP_READBLOCK = 1 << 21;
        const GROUP_WRITEBLOCK = 1 << 22;
        const GROUP_EXECBLOCK = 1 << 23;
        const GROUP_READLINK = 1 << 24;
        const GROUP_WRITELINK = 1 << 25;
        const GROUP_EXECLINK = 1 << 26;
        const GROUP_READSOCKET = 1 << 27;
        const GROUP_WRITESOCKET = 1 << 28;
        const GROUP_EXECSOCKET = 1 << 29;

        const OWNER_READFILE = 1 << 30;
        const OWNER_WRITEFILE = 1 << 31;
        const OWNER_EXECFILE = 1 << 32;
        const OWNER_READDIR = 1 << 33;
        const OWNER_WRITEDIR = 1 << 34; // grants permission to modify directory entries
        const OWNER_EXECDIR = 1 << 35; // grants permission to access directory entries
        const OWNER_READBLOCK = 1 << 36;
        const OWNER_WRITEBLOCK = 1 << 37;
        const OWNER_EXECBLOCK = 1 << 38;
        const OWNER_READLINK = 1 << 39;
        const OWNER_WRITELINK = 1 << 40;
        const OWNER_EXECLINK = 1 << 41;
        const OWNER_READSOCKET = 1 << 42;
        const OWNER_WRITESOCKET = 1 << 43;
        const OWNER_EXECSOCKET = 1 << 44;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ProcessUserId(u64);

impl ProcessUserId {
    fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        ProcessUserId(COUNTER.fetch_add(1, Ordering::Relaxed))
    }

    pub(crate) const fn inner(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct ProcessGroupId(u64);

impl ProcessGroupId {
    fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        ProcessGroupId(COUNTER.fetch_add(1, Ordering::Relaxed))
    }

    pub(crate) const fn inner(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ProcessSchedulingInfo {
    task_id: TaskId,
    task_priority: TaskPriority,
    task_status: TaskStatus,
}

#[derive(Debug, Clone)]
pub(crate) struct Process {
    id: ProcessId,
    parent: Option<ProcessId>,     // PID of the parent process
    children: BTreeSet<ProcessId>, // PID's of the child processes
    privileges: ProcessPrivileges,
    // sched: ProcessSchedulingInfo,
    dir: String,
    user: ProcessUserId,
    group: ProcessGroupId,
    env: BTreeMap<String, String>,
    threads: [Option<Box<Thread>>; MAX_THREADS],
    resource_handles: [Option<Box<Resource>>; MAX_RESOURCE_HANDLES],
    code_addr: u64,
    stack_addr: u64,
    entry_point_addr: u64,
}

impl Process {
    pub(crate) fn new(dir: &str) -> Self {
        let id = ProcessId::new();

        let mut threads = [(); MAX_THREADS].map(|_| None);
        threads[0] = Some(Box::new(Thread::new(id))); // main thread

        let mut resource_handles = [(); MAX_RESOURCE_HANDLES].map(|_| None);

        resource_handles[0] = Some(Box::new(Resource::Device(Device::Console(Console::new())))); // stdin
        resource_handles[1] = Some(Box::new(Resource::Device(Device::Console(Console::new())))); // stdout
        resource_handles[2] = Some(Box::new(Resource::Device(Device::Console(Console::new())))); // stderr
        resource_handles[3] = Some(Box::new(Resource::Device(Device::Null)));

        Self {
            id,
            parent: None,
            children: BTreeSet::new(),
            privileges: !ProcessPrivileges::from_bits(0b111111111111111111111111111111).unwrap(),
            dir: dir.into(),
            user: ProcessUserId::new(),
            group: ProcessGroupId::new(),
            env: BTreeMap::new(),
            threads,
            resource_handles,
            code_addr: 0,
            stack_addr: 0,
            entry_point_addr: 0,
        }
    }

    pub(crate) fn fork(&mut self) -> Self {
        let mut child = self.clone();
        child.set_parent(self.id());
        child.children.clear();
        self.children.insert(child.id);

        child
    }

    pub(crate) fn exit(self, code: u8) -> ExitCode {
        allocator::free(self.code_addr, MAX_PROC_SIZE);

        ExitCode::from(code as usize)
    }

    pub(crate) const fn id(&self) -> ProcessId {
        self.id
    }

    pub(crate) fn parent(&self) -> Option<Box<Self>> {
        self.parent
            .map(|pid| PROCESSES.read()[pid.0 as usize].clone())
    }

    pub(crate) fn set_parent(&mut self, pid: ProcessId) {
        self.parent = Some(pid);
    }

    pub(crate) fn handle(&self, handle: usize) -> Option<Box<Resource>> {
        self.resource_handles[handle].clone()
    }

    pub(crate) fn create_handle(&mut self, resource: Resource) -> Result<usize, ()> {
        let (min, max) = (4, MAX_RESOURCE_HANDLES);

        for handle in min..max {
            if self.handle(handle).is_none() {
                self.resource_handles[handle] = Some(Box::new(resource));
                return Ok(handle);
            }
        }

        recoverable!("Could not create file handle");
        Err(())
    }

    pub(crate) fn update_handle(&mut self, handle: usize, updated: Resource) {
        self.resource_handles[handle] = Some(Box::new(updated));
    }

    pub(crate) fn delete_handle(&mut self, handle: usize) {
        self.resource_handles[handle] = None;
    }

    pub(crate) fn get_env(&self, key: &str) -> String {
        self.env[key].clone()
    }

    pub(crate) fn set_env(&mut self, key: &str, value: &str) {
        if let Some(v) = self.env.get_mut(key) {
            *v = value.into();
        }
    }

    pub(crate) fn dir(&self) -> String {
        self.dir.clone()
    }

    pub(crate) fn set_dir(&mut self, dir: &str) {
        self.dir = dir.into();
    }

    pub(crate) const fn code_addr(&self) -> u64 {
        self.code_addr
    }

    pub(crate) fn set_code_addr(&mut self, addr: u64) {
        self.code_addr = addr;
    }

    pub(crate) fn ptr_from_addr(&self, addr: u64) -> *mut u8 {
        let code_addr = self.code_addr();

        if addr < code_addr {
            (code_addr + addr) as *mut u8
        } else {
            addr as *mut u8
        }
    }
}

pub(crate) fn current_process() -> Process {
    *PROCESSES.read()[0].clone()
}
