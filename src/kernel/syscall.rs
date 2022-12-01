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
    let res = syscall3(READ, handle, buf.as_ptr() as usize, buf.len()) as isize;

    if res >= 0 {
        Some(res as usize)
    } else {
        None
    }
}

pub(super) fn write(handle: usize, buf: &[u8]) -> Option<usize> {
    let res = syscall3(WRITE, handle, buf.as_ptr() as usize, buf.len()) as isize;

    if res >= 0 {
        Some(res as usize)
    } else {
        None
    }
}

pub(super) fn open(path: &str, flags: usize) -> Option<usize> {
    let res = syscall3(OPEN, path.as_ptr() as usize, path.len(), flags) as isize;

    if res >= 0 {
        Some(res as usize)
    } else {
        None
    }
}

pub fn close(handle: usize) {
    syscall1(CLOSE, handle);
}

pub(super) fn dup(old_handle: usize, new_handle: usize) -> Option<usize> {
    let res = syscall2(DUP, old_handle, new_handle) as isize;

    if res >= 0 {
        Some(res as usize)
    } else {
        None
    }
}

pub(super) fn seek(handle: usize, offset: usize, flags: usize) -> Option<usize> {
    let res = syscall3(SEEK, handle, offset, flags) as isize;

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
    syscall1(EXIT, code as usize);
}

pub(super) fn exit_group(code: ExitCode) {
    syscall1(EXIT_GROUP, code as usize);
}

pub(super) fn reboot() {
    syscall0(REBOOT);
}

pub(super) fn info(path: &str) {
    todo!();
}

macro syscall_fns($(fn $name:ident($id:ident $(,$arg1:ident $(,$arg2:ident $(,$arg3:ident $(,$arg4:ident)?)?)?)?) -> usize;)*) {
    $(
        fn $name(mut $id: usize, $($arg1: usize, $($arg2: usize, $($arg3: usize, $($arg4: usize)?)?)?)?) -> usize {
            unsafe {
                asm!(
                    "int 0x80",
                    inout("rax") $id,
                    $(in("rdi") $arg1, $(in("rsi") $arg2, $(in("rdx") $arg3, $(in("r8") $arg4,)?)?)?)?
                    options(nostack),
                );

                $id
            }
        }
    )+
}

syscall_fns!(
    fn syscall0(id) -> usize;
    fn syscall1(id, arg1) -> usize;
    fn syscall2(id, arg1, arg2) -> usize;
    fn syscall3(id, arg1, arg2, arg3) -> usize;
    fn syscall4(id, arg1, arg2, arg3, arg4) -> usize;
);
