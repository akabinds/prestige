use alloc::{boxed::Box, collections::BTreeMap, string::String};

use super::{
    io::console::Console,
    resource::{Device, Resource},
};

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ExitCode {
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

const MAX_RESOURCE_HANDLES: usize = 64;

#[derive(Debug, Clone)]
pub struct Thread {
    id: usize,
    // TODO
}

impl Thread {
    pub fn id(&self) -> usize {
        self.id
    }
}

pub fn current_thread() -> Thread {
    todo!();
}

const MAX_THREADS: usize = 100;

#[derive(Debug, Clone)]
pub struct Process {
    id: usize,
    parent: Option<Box<Process>>,
    children: BTreeMap<usize, Process>,
    dir: String,
    user: Option<String>,
    env: BTreeMap<String, String>,
    threads: [Option<Box<Thread>>; MAX_THREADS],
    resource_handles: [Option<Box<Resource>>; MAX_RESOURCE_HANDLES],
}

impl Process {
    pub fn new(id: usize, dir: String, user: Option<String>) -> Self {
        let threads = [(); MAX_THREADS].map(|_| None);
        let mut resource_handles = [(); MAX_RESOURCE_HANDLES].map(|_| None);

        resource_handles[0] = Some(Box::new(Resource::Device(Device::Console(Console::new()))));
        resource_handles[1] = Some(Box::new(Resource::Device(Device::Console(Console::new()))));
        resource_handles[2] = Some(Box::new(Resource::Device(Device::Console(Console::new()))));
        resource_handles[3] = Some(Box::new(Resource::Device(Device::Null)));

        Self {
            id,
            parent: None,
            children: BTreeMap::new(),
            dir,
            user,
            env: BTreeMap::new(),
            threads,
            resource_handles,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn spawn() -> Result<(), ExitCode> {
        todo!();
    }

    pub fn fork(&self) -> Result<Self, ExitCode> {
        Ok(self.clone())
    }

    fn exec(&self) {
        todo!();
    }
}

pub fn current_process() -> Process {
    todo!();
}

pub fn create_resource_handle(resource: Resource) -> Result<usize, ()> {
    todo!();
}
