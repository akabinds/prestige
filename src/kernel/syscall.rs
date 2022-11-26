use super::{
    fs::{self, FileIO},
    process::{self, ExitCode, Process, Thread},
};
use core::arch::asm;

const READ: usize = 0x0;
const WRITE: usize = 0x1;
const OPEN: usize = 0x2;
const CLOSE: usize = 0x3;
const DUP: usize = 0x4;
const PROC_SPAWN: usize = 0x5;
const THREAD_SPAWN: usize = 0x6;
const PROC_FORK: usize = 0x7;
const THREAD_CLONE: usize = 0x8;
const EXIT: usize = 0x9;
const REBOOT: usize = 0xA;

pub(crate) fn dispatch(id: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize) -> usize {
    todo!();
}
