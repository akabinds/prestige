mod service;

use super::{
    fs::{self, FileIO},
    io::kprint,
    process::{self, ExitCode, Process, Thread},
};
use core::arch::asm;

const READ: usize = 0x0;
const WRITE: usize = 0x1;
const OPEN: usize = 0x2;
const CLOSE: usize = 0x3;
const DUP: usize = 0x4;
const SEEK: usize = 0x5;
const PROC_SPAWN: usize = 0x6;
const THREAD_SPAWN: usize = 0x7;
const PROC_FORK: usize = 0x8;
const THREAD_CLONE: usize = 0x9;
const PROC_KILL: usize = 0xA;
const THREAD_KILL: usize = 0xB;
const EXIT: usize = 0xC;
const EXIT_GROUP: usize = 0xD;
const REBOOT: usize = 0xE;
const INFO: usize = 0xF;

pub(super) fn dispatch(id: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    let calling_proc = process::current_process();

    match id {
        READ => {
            let handle = arg1;
            let ptr = calling_proc.ptr_from_addr(arg2 as u64);
            let len = arg3;
            let buf = unsafe { core::slice::from_raw_parts_mut(ptr, len) };

            service::read(handle, buf) as usize
        }
        WRITE => {
            let handle = arg1;
            let ptr = calling_proc.ptr_from_addr(arg2 as u64);
            let len = arg3;
            let buf = unsafe { core::slice::from_raw_parts(ptr, len) };

            service::write(handle, buf) as usize
        }
        OPEN => {
            let ptr = calling_proc.ptr_from_addr(arg1 as u64);
            let len = arg2;
            let flags = arg3;
            let path =
                unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(ptr, len)) };

            service::open(path, flags) as usize
        }
        CLOSE => {
            let handle = arg1;
            service::close(handle);
            0
        }
        DUP => {
            let old_handle = arg1;
            let new_handle = arg2;

            service::dup(old_handle, new_handle) as usize
        }
        SEEK => todo!(),
        PROC_SPAWN => todo!(),
        THREAD_SPAWN => todo!(),
        PROC_FORK => service::pfork() as usize,
        THREAD_CLONE => todo!(),
        PROC_KILL => todo!(),
        THREAD_KILL => todo!(),
        EXIT => service::exit(ExitCode::from(arg1)) as usize,
        REBOOT => service::reboot(),
        INFO => todo!(),
        _ => unimplemented!("Invalid syscall ID"),
    }
}

pub(super) fn read(handle: usize, buf: &mut [u8]) -> Option<usize> {
    let res = unsafe { syscall!(READ, handle, buf.as_ptr(), buf.len()) } as isize;

    if res >= 0 {
        Some(res as usize)
    } else {
        None
    }
}

pub(super) fn write(handle: usize, buf: &[u8]) -> Option<usize> {
    let res = unsafe { syscall!(WRITE, handle, buf.as_ptr(), buf.len()) } as isize;

    if res >= 0 {
        Some(res as usize)
    } else {
        None
    }
}

pub(super) fn open(path: &str, flags: usize) -> Option<usize> {
    let res = unsafe { syscall!(OPEN, path.as_ptr(), path.len(), flags) } as isize;

    if res >= 0 {
        Some(res as usize)
    } else {
        None
    }
}

pub fn close(handle: usize) {
    unsafe { syscall!(CLOSE, handle) };
}

pub(super) fn dup(old_handle: usize, new_handle: usize) -> Option<usize> {
    let res = unsafe { syscall!(DUP, old_handle, new_handle) } as isize;

    if res >= 0 {
        Some(res as usize)
    } else {
        None
    }
}

pub(super) fn seek(handle: usize, offset: usize, flags: usize) -> Option<usize> {
    let res = unsafe { syscall!(SEEK, handle, offset, flags) } as isize;

    if res >= 0 {
        Some(res as usize)
    } else {
        None
    }
}

pub(super) fn pspawn() {
    todo!();
}

pub(super) fn tspawn() {
    todo!();
}

pub(super) fn pfork() {
    todo!();
}

pub(super) fn tclone() {
    todo!();
}

pub(super) fn pkill(pid: usize, signal: usize) -> Option<usize> {
    todo!();
}

pub(super) fn tkill(tid: usize, signal: usize) -> Option<usize> {
    todo!();
}

pub(super) fn exit(code: ExitCode) {
    unsafe { syscall!(EXIT, code as usize) };
}

pub(super) fn exit_group(code: ExitCode) {
    unsafe { syscall!(EXIT_GROUP, code as usize) };
}

pub(super) fn reboot() {
    unsafe { syscall!(REBOOT) };
}

pub(super) fn info(path: &str) {
    todo!();
}

#[doc(hidden)]
unsafe fn syscall0(n: usize) -> usize {
    let res: usize;
    asm!(
        "int 0x80",
        in("rax") n,
        lateout("rax") res
    );
    res
}

#[doc(hidden)]
unsafe fn syscall1(n: usize, arg1: usize) -> usize {
    let res: usize;
    asm!(
        "int 0x80",
        in("rax") n,
        in("rdi") arg1,
        lateout("rax") res
    );
    res
}

#[doc(hidden)]
unsafe fn syscall2(n: usize, arg1: usize, arg2: usize) -> usize {
    let res: usize;
    asm!(
        "int 0x80",
        in("rax") n,
        in("rdi") arg1,
        in("rsi") arg2,
        lateout("rax") res
    );
    res
}

#[doc(hidden)]
unsafe fn syscall3(n: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    let res: usize;
    asm!(
        "int 0x80",
        in("rax") n,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        lateout("rax") res
    );
    res
}

#[doc(hidden)]
unsafe fn syscall4(n: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    let res: usize;
    asm!(
        "int 0x80",
        in("rax") n,
        in("rdi") arg1,
        in("rsi") arg2,
        in("rdx") arg3,
        in("r8") arg4,
        lateout("rax") res
    );
    res
}

macro syscall {
    ($n:expr) => (syscall0($n)),
    ($n:expr, $a1:expr) => (syscall1($n, $a1 as usize)),
    ($n:expr, $a1:expr, $a2:expr) => (syscall2($n, $a1 as usize, $a2 as usize)),
    ($n:expr, $a1:expr, $a2:expr, $a3:expr) => (syscall3($n, $a1 as usize, $a2 as usize, $a3 as usize)),
    ($n:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => (syscall4($n, $a1 as usize, $a2 as usize, $a3 as usize, $a4 as usize))
}
