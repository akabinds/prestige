use super::STDIN;
use crate::kernel::fs::FileIO;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

pub static ECHO: AtomicBool = AtomicBool::new(true);
pub static RAW: AtomicBool = AtomicBool::new(false);

fn switch_echo(cmd: &str) {
    match cmd {
        "enable" => ECHO.store(true, Ordering::SeqCst),
        "disable" => ECHO.store(false, Ordering::SeqCst),
        _ => (),
    }
}

fn switch_raw(cmd: &str) {
    match cmd {
        "enable" => RAW.store(true, Ordering::SeqCst),
        "disable" => RAW.store(false, Ordering::SeqCst),
        _ => (),
    }
}

#[derive(Debug, Clone)]
pub struct Console;

impl Console {
    pub fn new() -> Self {
        Self {}
    }
}

impl FileIO for Console {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()> {
        todo!();
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize, ()> {
        todo!();
    }
}
