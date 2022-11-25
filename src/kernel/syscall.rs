mod service;

use core::arch::asm;

use super::{
    fs::{self, FileIO},
    process::{self, ExitCode, Process, Thread},
};

const READ: usize = 0x0;
const WRITE: usize = 0x1;
const OPEN: usize = 0x2;
const CLOSE: usize = 0x3;
const PROC_SPAWN: usize = 0x4;
const THREAD_SPAWN: usize = 0x5;
const PROC_FORK: usize = 0x6;
const THREAD_CLONE: usize = 0x7;
const EXIT: usize = 0x8;

pub fn dispatch(id: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
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
        PROC_SPAWN => todo!(),
        THREAD_SPAWN => todo!(),
        PROC_FORK => service::pfork() as usize,
        THREAD_CLONE => todo!(),
        EXIT => service::exit(ExitCode::from(arg1)) as usize,
        _ => unimplemented!(),
    }
}

pub fn read(handle: usize, buf: &mut [u8]) -> Option<usize> {
    let ptr = buf.as_ptr() as usize;
    let len = buf.len();
    let res = unsafe { syscall!(READ, handle, ptr, len) } as isize;

    if res >= 0 {
        Some(res as usize)
    } else {
        None
    }
}

pub fn write(handle: usize, buf: &[u8]) -> Option<usize> {
    let ptr = buf.as_ptr() as usize;
    let len = buf.len();
    let res = unsafe { syscall!(WRITE, handle, ptr, len) } as isize;

    if res >= 0 {
        Some(res as usize)
    } else {
        None
    }
}

pub fn open(path: &str, flags: usize) -> Option<usize> {
    let ptr = path.as_ptr() as usize;
    let len = path.len();
    let res = unsafe { syscall!(OPEN, ptr, len, flags) } as isize;

    if res >= 0 {
        Some(res as usize)
    } else {
        None
    }
}

pub fn close(handle: usize) {
    unsafe { syscall!(CLOSE, handle) };
}

pub fn pspawn() {
    todo!();
}

pub fn tspawn() {
    todo!();
}

pub fn pfork() -> Option<usize> {
    let calling_proc = process::current_process();

    todo!();
}

pub fn tclone(thread: Thread) -> Option<usize> {
    todo!();
}

pub fn exit(code: ExitCode) {
    unsafe { syscall!(EXIT, code as usize) };
}

#[doc(hidden)]
unsafe fn syscall0(id: usize) -> usize {
    let res: usize;

    asm!(
        "int 0x80", in("rax") id,
        lateout("rax") res
    );

    res
}

#[doc(hidden)]
unsafe fn syscall1(id: usize, arg1: usize) -> usize {
    let res: usize;

    asm!(
        "int 0x80", in("rax") id,
        in("rdi") arg1,
        lateout("rax") res
    );

    res
}

#[doc(hidden)]
unsafe fn syscall2(id: usize, arg1: usize, arg2: usize) -> usize {
    let res: usize;

    asm!(
        "int 0x80", in("rax") id,
        in("rdi") arg1, in("rsi") arg2,
        lateout("rax") res
    );

    res
}

#[doc(hidden)]
unsafe fn syscall3(id: usize, arg1: usize, arg2: usize, arg3: usize) -> usize {
    let res: usize;

    asm!(
        "int 0x80", in("rax") id,
        in("rdi") arg1, in("rsi") arg2, in("rdx") arg3,
        lateout("rax") res
    );

    res
}

#[doc(hidden)]
unsafe fn syscall4(id: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    let res: usize;

    asm!(
        "int 0x80", in("rax") id,
        in("rdi") arg1, in("rsi") arg2, in("rdx") arg3, in("r8") arg4,
        lateout("rax") res
    );

    res
}

macro syscall {
    ($id:expr) => (
        syscall0(
            $id as usize)),
    ($id:expr, $a1:expr) => (
        syscall1(
            $id as usize, $a1 as usize)),
    ($id:expr, $a1:expr, $a2:expr) => (
        syscall2(
            $id as usize, $a1 as usize, $a2 as usize)),
    ($id:expr, $a1:expr, $a2:expr, $a3:expr) => (
        syscall3(
            $id as usize, $a1 as usize, $a2 as usize, $a3 as usize)),
    ($id:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => (
        syscall4(
            $id as usize, $a1 as usize, $a2 as usize, $a3 as usize, $a4 as usize)),
}
