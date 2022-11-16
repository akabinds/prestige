use alloc::{boxed::Box, collections::BTreeMap, string::String};

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

#[derive(Debug, Clone)]
pub enum Resource {}

const MAX_RESOURCE_HANDLES: usize = 64;

#[derive(Debug, Clone)]
pub struct Thread {
    id: usize,
    // TODO
}

const MAX_THREADS: usize = 100;

#[derive(Debug, Clone)]
pub struct Process {
    id: usize,
    dir: String,
    env: BTreeMap<String, String>,
    threads: [Box<Thread>; MAX_THREADS],
    resource_handles: [Box<Resource>; MAX_RESOURCE_HANDLES],
}

impl Process {
    pub fn new() -> Self {
        todo!();
    }
}
