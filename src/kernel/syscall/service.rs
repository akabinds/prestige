use core::arch::asm;

use crate::kernel::{
    fs::FileIO,
    process::{self, ExitCode},
};

pub(super) fn read(handle: usize, buf: &mut [u8]) -> isize {
    let mut calling_proc = process::current_process();

    if let Some(mut res) = calling_proc.handle(handle) {
        if let Ok(bytes) = res.read(buf) {
            calling_proc.update_handle(handle, *res);
            return bytes as isize;
        }
    }

    -1
}

pub(super) fn write(handle: usize, buf: &[u8]) -> isize {
    let mut calling_proc = process::current_process();

    if let Some(mut res) = calling_proc.handle(handle) {
        if let Ok(bytes) = res.write(buf) {
            calling_proc.update_handle(handle, *res);
            return bytes as isize;
        }
    }

    0
}

pub(super) fn open(path: &str, flags: usize) -> isize {
    todo!();
}

pub(super) fn close(handle: usize) {
    let mut calling_proc = process::current_process();
    calling_proc.delete_handle(handle);
}

pub(super) fn dup(old_handle: usize, new_handle: usize) -> isize {
    let mut calling_proc = process::current_process();

    if let Some(handle) = calling_proc.handle(old_handle) {
        calling_proc.update_handle(new_handle, *handle);
        return new_handle as isize;
    }

    -1
}

pub(super) fn seek(handle: usize, offset: usize, flags: usize) -> isize {
    todo!();
}

pub(super) fn pspawn() {
    todo!();
}

pub(super) fn tspawn() {
    todo!();
}

pub(super) fn pfork() -> isize {
    let mut calling_proc = process::current_process();

    if let Ok(child) = calling_proc.fork() {
        return child.id() as isize;
    }

    -1
}

pub(super) fn tclone() -> isize {
    todo!();
}

pub(super) fn pkill(pid: usize, signal: usize) -> isize {
    todo!();
}

pub(super) fn tkill(tid: usize, signal: usize) -> isize {
    todo!();
}

pub(super) fn exit(code: ExitCode) -> ExitCode {
    let calling_proc = process::current_process();
    calling_proc.exit(code as u8)
}

pub(super) fn exit_group(code: ExitCode) {
    todo!();
}

pub(super) fn reboot() -> usize {
    unsafe {
        asm!("xor rax, rax", "mov cr3, rax");
    }

    0
}

pub(super) fn info(path: &str) -> isize {
    todo!();
}
