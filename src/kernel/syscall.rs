use super::process::{Process, Thread};

pub fn read(handle: usize, buf: &mut [u8]) -> Option<usize> {
    todo!();
}

pub fn write(handle: usize, buf: &[u8]) -> Option<usize> {
    todo!();
}

pub fn fork(process: Process) {
    todo!();
}

pub fn tclone(thread: Thread) {
    todo!();
}
