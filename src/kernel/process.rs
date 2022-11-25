use super::{
    io::console::Console,
    resource::{Device, Resource},
};
use alloc::{boxed::Box, collections::BTreeMap, string::String};
use lazy_static::lazy_static;
use spin::RwLock;
use x86_64::{structures::idt::InterruptStackFrameValue, VirtAddr};

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
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

#[repr(align(8), C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Registers {
    pub rax: usize,
    pub rbx: usize,
    pub rcx: usize,
    pub rdx: usize,
    pub rsi: usize,
    pub rdi: usize,
    pub rbp: usize,
    pub rsp: usize,
    pub r8: usize,
    pub r9: usize,
    pub r10: usize,
    pub r11: usize,
}

#[derive(Debug, Clone)]
pub struct Thread {
    id: usize,
    proc: Box<Process>,
    // WIP
}

impl Thread {
    pub fn new(id: usize, proc: Box<Process>) -> Self {
        Self { id, proc }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

pub fn current_thread() -> Thread {
    let calling_proc = current_process();

    todo!();
}

const MAX_RESOURCE_HANDLES: usize = 64;
const MAX_THREADS: usize = 100;
const MAX_PROCESSES: usize = 50;

lazy_static! {
    pub static ref PROCESSES: RwLock<[Box<Process>; MAX_PROCESSES]> =
        RwLock::new([(); MAX_PROCESSES].map(|_| Box::new(Process::new(0, "/", None))));
}

#[derive(Debug, Clone)]
pub struct Process {
    id: usize,
    code_addr: u64,
    stack_addr: u64,
    entry_point_addr: u64,
    stack_frame: InterruptStackFrameValue,
    registers: Registers,
    parent: Option<Box<Process>>,
    children: BTreeMap<usize, Process>,
    dir: String,
    user: Option<String>,
    env: BTreeMap<String, String>,
    threads: [Option<Box<Thread>>; MAX_THREADS],
    resource_handles: [Option<Box<Resource>>; MAX_RESOURCE_HANDLES],
}

impl Process {
    pub fn new(id: usize, dir: &str, user: Option<&str>) -> Self {
        let stack_frame = InterruptStackFrameValue {
            instruction_pointer: VirtAddr::new(0),
            code_segment: 0,
            cpu_flags: 0,
            stack_pointer: VirtAddr::new(0),
            stack_segment: 0,
        };

        let threads = [(); MAX_THREADS].map(|_| None);
        // threads[0] = Some(Box::new(Thread::new(0, Box::new(self))));

        let mut resource_handles = [(); MAX_RESOURCE_HANDLES].map(|_| None);

        resource_handles[0] = Some(Box::new(Resource::Device(Device::Console(Console::new())))); // stdin
        resource_handles[1] = Some(Box::new(Resource::Device(Device::Console(Console::new())))); // stdout
        resource_handles[2] = Some(Box::new(Resource::Device(Device::Console(Console::new())))); // stderr
        resource_handles[3] = Some(Box::new(Resource::Device(Device::Null)));

        Self {
            id,
            code_addr: 0,
            stack_addr: 0,
            entry_point_addr: 0,
            stack_frame,
            registers: Registers::default(),
            parent: None,
            children: BTreeMap::new(),
            dir: dir.into(),
            user: user.map(String::from),
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

    pub fn handle(&self, handle: usize) -> Option<Box<Resource>> {
        self.resource_handles[handle].clone()
    }

    pub fn create_handle(&mut self, resource: Resource) -> Result<usize, ()> {
        let (min, max) = (4, MAX_RESOURCE_HANDLES);

        for handle in min..max {
            if self.handle(handle).is_none() {
                self.resource_handles[handle] = Some(Box::new(resource));
                return Ok(handle);
            }
        }

        Err(())
    }

    pub fn update_handle(&mut self, handle: usize, updated: Resource) {
        self.resource_handles[handle] = Some(Box::new(updated));
    }

    pub fn delete_handle(&mut self, handle: usize) {
        self.resource_handles[handle] = None;
    }

    pub fn get_env(&self, key: &str) -> String {
        self.env[key].clone()
    }

    pub fn set_env(&mut self, key: &str, value: &str) {
        if let Some(v) = self.env.get_mut(key) {
            *v = value.into();
        }
    }

    pub fn dir(&self) -> String {
        self.dir.clone()
    }

    pub fn set_dir(&mut self, dir: &str) {
        self.dir = dir.into();
    }

    pub fn user(&self) -> Option<String> {
        self.user.clone()
    }

    pub fn set_user(&mut self, user: &str) {
        self.user = Some(user.into());
    }

    pub fn stack_frame(&self) -> InterruptStackFrameValue {
        self.stack_frame
    }

    pub fn set_stack_frame(&mut self, sf: InterruptStackFrameValue) {
        self.stack_frame = sf;
    }

    pub fn registers(&self) -> Registers {
        self.registers
    }

    pub fn set_registers(&mut self, registers: Registers) {
        self.registers = registers;
    }

    pub fn ptr_from_addr(&self, addr: u64) -> *mut u8 {
        let code_addr = self.code_addr;

        if addr < code_addr {
            (code_addr + addr) as *mut u8
        } else {
            addr as *mut u8
        }
    }

    fn exec(&self) {
        todo!();
    }

    pub fn exit(self, code: u8) -> ExitCode {
        todo!();
    }
}

pub fn current_process() -> Process {
    *PROCESSES.read()[0].clone()
}
