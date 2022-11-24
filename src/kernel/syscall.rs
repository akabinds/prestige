use super::{
    fs::{self, FileIO},
    process::{self, ExitCode, Process, Thread},
};

pub fn read(handle: usize, buf: &mut [u8]) -> isize {
    let mut calling_proc = process::current_process();

    if let Some(mut res) = calling_proc.handle(handle) {
        if let Ok(bytes) = res.read(buf) {
            calling_proc.update_handle(handle, *res);
            return bytes as isize;
        }
    }

    -1
}

pub fn write(handle: usize, buf: &[u8]) -> isize {
    let mut calling_proc = process::current_process();

    if let Some(mut res) = calling_proc.handle(handle) {
        if let Ok(bytes) = res.write(buf) {
            calling_proc.update_handle(handle, *res);
            return bytes as isize;
        }
    }

    -1
}

pub fn open(path: &str, flags: usize) -> isize {
    let mut calling_proc = process::current_process();

    if let Some(resource) = fs::open(path, flags) {
        if let Ok(handle) = calling_proc.create_handle(resource) {
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

pub fn exit(code: u8) -> ExitCode {
    let calling_proc = process::current_process();
    calling_proc.exit(code)
}
