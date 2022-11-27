use super::{
    gdt::GDT,
    io::{console::Console, recoverable},
    mem::allocator,
    resource::{Device, Resource},
};
use alloc::{
    boxed::Box,
    collections::{BTreeMap, BTreeSet},
    string::String,
};
use core::{
    arch::asm,
    sync::atomic::{AtomicU64, Ordering},
};
use lazy_static::lazy_static;
use spin::RwLock;
use x86_64::{structures::idt::InterruptStackFrameValue, VirtAddr};

pub(crate) fn init(addr: u64) {
    CODE_ADDR.store(addr, Ordering::SeqCst);
}

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

#[repr(align(8), C)]
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Registers {
    pub(crate) r11: usize,
    pub(crate) r10: usize,
    pub(crate) r9: usize,
    pub(crate) r8: usize,
    pub(crate) rdi: usize,
    pub(crate) rsi: usize,
    pub(crate) rdx: usize,
    pub(crate) rcx: usize,
    pub(crate) rax: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct Thread {
    id: usize,
    proc: usize, // PID of the process containing this thread
                 // WIP
}

impl Thread {
    pub(crate) fn new(id: usize, proc: usize) -> Self {
        Self { id, proc }
    }

    pub(crate) fn id(&self) -> usize {
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
        RwLock::new([(); MAX_PROCESSES].map(|_| Box::new(Process::new(0, "/", None))));
}

#[derive(Debug, Clone)]
pub(crate) struct Process {
    id: usize,
    code_addr: u64,
    stack_addr: u64,
    entry_point_addr: u64,
    stack_frame: InterruptStackFrameValue,
    registers: Registers,
    parent: Option<usize>,     // PID of the parent process
    children: BTreeSet<usize>, // PID's of the child processes
    dir: String,
    user: Option<String>,
    env: BTreeMap<String, String>,
    threads: [Option<Box<Thread>>; MAX_THREADS],
    resource_handles: [Option<Box<Resource>>; MAX_RESOURCE_HANDLES],
}

impl Process {
    pub(crate) fn new(id: usize, dir: &str, user: Option<&str>) -> Self {
        let stack_frame = InterruptStackFrameValue {
            instruction_pointer: VirtAddr::new(0),
            code_segment: 0,
            cpu_flags: 0,
            stack_pointer: VirtAddr::new(0),
            stack_segment: 0,
        };

        let mut threads = [(); MAX_THREADS].map(|_| None);
        threads[0] = Some(Box::new(Thread::new(0, id)));

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
            children: BTreeSet::new(),
            dir: dir.into(),
            user: user.map(String::from),
            env: BTreeMap::new(),
            threads,
            resource_handles,
        }
    }

    pub(crate) fn spawn(bin: &[u8], args_ptr: usize, args_len: usize) -> Result<(), ExitCode> {
        if let Ok(id) = Self::init(bin) {
            let proc = PROCESSES.read()[id].clone();
            proc.exec(args_ptr, args_len);
            Ok(())
        } else {
            Err(ExitCode::ExecFault)
        }
    }

    fn init(bin: &[u8]) -> Result<usize, ()> {
        let proc_size = MAX_PROC_SIZE as u64;
        let code_addr = CODE_ADDR.fetch_add(proc_size, Ordering::SeqCst);
        let stack_addr = code_addr + proc_size;

        todo!();
    }

    pub(crate) fn fork(&mut self) -> Result<Self, ExitCode> {
        let mut child = self.clone();
        child.set_id(self.id() + 1);
        child.set_parent(self.id());
        self.children.insert(child.id);

        Ok(child)
    }

    fn exec(&self, args_ptr: usize, args_len: usize) {
        let heap_addr = self.code_addr + (self.stack_addr - self.code_addr) / 2;
        allocator::alloc(heap_addr, 1).expect("Unable to allocate");

        unsafe {
            asm!(
                "cli",
                "push {:r}",
                "push {:r}",
                "push 0x200",
                "push {:r}",
                "push {:r}",
                "iretq",
                in(reg) GDT.1.user_data.0,
                in(reg) self.stack_addr,
                in(reg) GDT.1.user_code.0,
                in(reg) self.code_addr + self.entry_point_addr,
                in("rdi") args_ptr,
                in("rsi") args_len,
            )
        }
    }

    pub(crate) fn exit(self, code: u8) -> ExitCode {
        allocator::free(self.code_addr, MAX_PROC_SIZE);

        ExitCode::from(code as usize)
    }

    pub(crate) fn id(&self) -> usize {
        self.id
    }

    pub(crate) fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    pub(crate) fn parent(&self) -> Option<Box<Self>> {
        self.parent.map(|pid| PROCESSES.read()[pid].clone())
    }

    pub(crate) fn set_parent(&mut self, pid: usize) {
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

    pub(crate) fn user(&self) -> Option<String> {
        self.user.clone()
    }

    pub(crate) fn set_user(&mut self, user: &str) {
        self.user = Some(user.into());
    }

    pub(crate) fn stack_frame(&self) -> InterruptStackFrameValue {
        self.stack_frame
    }

    pub(crate) fn set_stack_frame(&mut self, sf: InterruptStackFrameValue) {
        self.stack_frame = sf;
    }

    pub(crate) fn registers(&self) -> Registers {
        self.registers
    }

    pub(crate) fn set_registers(&mut self, registers: Registers) {
        self.registers = registers;
    }

    pub(crate) fn code_addr(&self) -> u64 {
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
