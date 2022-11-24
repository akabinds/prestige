use super::{vga::print, Stdin, STDIN};
use crate::kernel::fs::FileIO;
use alloc::{
    string::{String, ToString},
    vec,
};
use core::sync::atomic::{AtomicBool, Ordering};

pub static ECHO: AtomicBool = AtomicBool::new(true);
pub static RAW: AtomicBool = AtomicBool::new(false);

pub fn switch_echo(cmd: &str) {
    match cmd {
        "enable" => ECHO.store(true, Ordering::SeqCst),
        "disable" => ECHO.store(false, Ordering::SeqCst),
        _ => (),
    }
}

pub fn switch_raw(cmd: &str) {
    match cmd {
        "enable" => RAW.store(true, Ordering::SeqCst),
        "disable" => RAW.store(false, Ordering::SeqCst),
        _ => (),
    }
}

fn is_enabled(mode: &str) -> bool {
    match mode {
        "echo" => ECHO.load(Ordering::SeqCst),
        "raw" => RAW.load(Ordering::SeqCst),
        _ => false,
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
        let stdin = STDIN.lock();

        let mut s = if buf.len() == 4 {
            stdin.read_char(&mut buf.to_vec()).to_string()
        } else {
            stdin.read_line(&mut buf.to_vec())
        };

        s.truncate(buf.len());
        let n = s.len();
        buf[0..n].copy_from_slice(s.as_bytes());
        Ok(n)
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize, ()> {
        let s = String::from_utf8_lossy(buf);
        let n = s.len();
        print!("{s}");
        Ok(n)
    }
}

const ETXT: char = '\x03';
const EOT: char = '\x04';
const BACKSPACE: char = '\x08';
const ESC: char = '\x1b';

pub fn handle_key_inp(key: char) {
    let mut stdin = STDIN.lock();

    if key == BACKSPACE && !is_enabled("raw") {
        todo!();
    } else {
        let key = if (key as u32) < 0xFF {
            (key as u8) as char
        } else {
            key
        };

        stdin.read_char(&mut vec![0; 4]);

        if is_enabled("echo") {
            match key {
                ETXT => print!("^C"),
                EOT => print!("^D"),
                ESC => print!("^["),
                _ => print!("{key}"),
            }
        }
    }
}
