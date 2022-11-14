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

    pub fn read_char(&self, mut buf: Vec<u8>) -> Option<char> {
        if let Some(bytes) = syscall::read(0, &mut buf) {
            if bytes > 0 {
                buf.resize(bytes, 0);
                return Some(String::from_utf8_lossy(&buf).to_string().remove(0));
            }
        }

        None
    }

    pub fn read_line(&self, mut buf: Vec<u8>) -> String {
        if let Some(bytes) = syscall::read(0, &mut buf) {
            buf.resize(bytes, 0);
            String::from_utf8_lossy(&buf).to_string()
        } else {
            String::new()
        }
    }
}

pub struct Stdout;

impl Stdout {
    fn new() -> Self {
        Self {}
    }
}

pub struct Stderr;

impl Stderr {
    fn new() -> Self {
        Self {}
    }
}
