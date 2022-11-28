pub(crate) mod console;
pub(crate) mod keyboard;
pub(crate) mod serial;
pub(crate) mod vga;

use super::syscall;
use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use console::Style;
use lazy_static::lazy_static;
use spin::{Mutex, MutexGuard};
use vte::Parser;

lazy_static! {
    pub(crate) static ref PARSER: Mutex<Parser> = Mutex::new(Parser::new());
    pub static ref STDIN: Mutex<Stdin> = Mutex::new(Stdin::new());
    pub static ref STDOUT: Mutex<Stdout> = Mutex::new(Stdout::new());
    pub static ref STDERR: Mutex<Stderr> = Mutex::new(Stderr::new());
}

pub struct Stdin;

impl Stdin {
    fn new() -> Self {
        Self {}
    }

    pub fn read_char(&self, buf: &mut [u8]) -> Option<char> {
        let Some(bytes) = syscall::read(0, buf) else {
            return None;
        };

        (bytes > 0).then(|| {
            buf.to_vec().resize(bytes, 0);
            String::from_utf8_lossy(buf).to_string().remove(0)
        })
    }

    pub fn read_line(&self, buf: &mut [u8]) -> String {
        let Some(bytes) = syscall::read(0, buf) else {
            return String::new();
        };

        buf.to_vec().resize(bytes, 0);
        String::from_utf8_lossy(buf).to_string()
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

pub fn stdin() -> MutexGuard<'static, Stdin> {
    STDIN.lock()
}

pub fn stdout() -> MutexGuard<'static, Stdout> {
    STDOUT.lock()
}

pub fn stderr() -> MutexGuard<'static, Stderr> {
    STDERR.lock()
}

pub(super) macro kprint($($arg:tt)*) {
    console::console_print(format_args!($($arg)*))
}

pub macro print($($arg:tt)*) {
    let s = format!("{}", format_args!($($arg)*));
    stdout().write(&s);
}

pub macro println {
    () => (print!("\n")),
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)))
}

macro eprint($($arg:tt)*) {
    let s = format!("{}", format_args!($($arg)*));
    stderr().write(&s);
}

macro eprintln {
    () => (eprint!("\n")),
    ($($arg:tt)*) => (eprint!("{}\n", format_args!($($arg)*)))
}

pub macro dbg($($arg:tt)*) {
    let color = Style::color("Cyan");
    let reset = Style::reset();
    kprint!("{}DEBUG:{} {}\n", color, reset, format_args!($($arg)*))
}

pub macro exception($($arg:tt)*) {
    let color = Style::color("LightRed");
    let reset = Style::reset();
    eprintln!("{}EXCEPTION:{} {}", color, reset, format_args!($($arg)*))
}

pub macro recoverable($($arg:tt)*) {
    let color = Style::color("LightRed");
    let reset = Style::reset();
    eprintln!("{}ERROR:{} {}", color, reset, format_args!($($arg)*))
}

pub macro fatal($($arg:tt)*) {
    let color = Style::color("Red");
    let reset = Style::reset();
    eprintln!("{}FATAL ERROR:{} {}", color, reset, format_args!($($arg)*))
}
