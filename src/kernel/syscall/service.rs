use core::arch::asm;

use crate::kernel::{
    fs::FileIO,
    process::{self, ExitCode},
};

pub(super) fn read(handle: usize, buf: &mut [u8]) -> isize {
    let mut calling_proc = process::current_process();

    let Some(mut res) = calling_proc.handle(handle) else {
        return -1;
    };

    let Ok(bytes) = res.read(buf) else {
        return -1;
    };

    calling_proc.update_handle(handle, *res);
    bytes as isize
}

pub(super) fn write(handle: usize, buf: &[u8]) -> isize {
    let mut calling_proc = process::current_process();

    let Some(mut res) = calling_proc.handle(handle) else {
        return -1;
    };

    let Ok(bytes) = res.write(buf) else {
        return -1;
    };

    calling_proc.update_handle(handle, *res);
    bytes as isize
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

    let Some(handle) = calling_proc.handle(old_handle) else {
        return -1;
    };

    calling_proc.update_handle(new_handle, *handle);
    new_handle as isize
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

    let Ok(child) = calling_proc.fork() else {
        return -1;
    };

    child.id() as isize
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
