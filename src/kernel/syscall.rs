use super::{
    fs,
    process::{self, ExitCode, Process, Thread},
};

pub fn read(handle: usize, buf: &mut [u8]) -> Option<usize> {
    todo!();
}

pub fn write(handle: usize, buf: &[u8]) -> Option<usize> {
    todo!();
}

pub fn open(path: &str, flags: usize) -> isize {
    if let Some(resource) = fs::open(path, flags) {
        if let Ok(handle) = process::create_resource_handle(resource) {
            return handle as isize;
        }
    }

    -1
}

pub fn pspawn() {
    todo!();
}

pub fn tspawn() {
    todo!();
}

pub fn pfork() -> isize {
    let calling_proc = process::current_process();

    if let Ok(child) = calling_proc.fork() {
        return child.id() as isize;
    }

    -1
}

pub fn tclone(thread: Thread) -> isize {
    todo!();
}

pub fn exit(code: usize) {
    todo!();
}
