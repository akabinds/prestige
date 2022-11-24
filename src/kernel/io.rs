pub mod console;
pub mod keyboard;
pub mod serial;
pub mod vga;

use super::syscall;
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use lazy_static::lazy_static;
use spin::Mutex;
use vte::Parser;

lazy_static! {
    pub static ref PARSER: Mutex<Parser> = Mutex::new(Parser::new());
    pub static ref STDIN: Mutex<Stdin> = Mutex::new(Stdin::new());
    pub static ref STDOUT: Mutex<Stdout> = Mutex::new(Stdout::new());
    pub static ref STDERR: Mutex<Stderr> = Mutex::new(Stderr::new());
}

pub struct Stdin;

impl Stdin {
    fn new() -> Self {
        Self {}
    }

    pub fn read_char(&self, buf: &mut Vec<u8>) -> char {
        let bytes = syscall::read(0, &mut *buf);

        if bytes > 0 {
            buf.resize(bytes as usize, 0);
            return String::from_utf8_lossy(buf).to_string().remove(0);
        }

        char::default()
    }

    pub fn read_line(&self, buf: &mut Vec<u8>) -> String {
        let bytes = syscall::read(0, &mut *buf);

        if bytes > 0 {
            buf.resize(bytes as usize, 0);
            return String::from_utf8_lossy(buf).to_string();
        }

        String::new()
    }
}

pub struct Stdout;

impl Stdout {
    fn new() -> Self {
        Self {}
    }

    pub fn write(&self, s: &str) {
        syscall::write(1, s.as_bytes());
    }
}

pub struct Stderr;

impl Stderr {
    fn new() -> Self {
        Self {}
    }

    pub fn write(&self, s: &str) {
        syscall::write(2, s.as_bytes());
    }
}
