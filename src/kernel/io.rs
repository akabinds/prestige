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
use spin::Mutex;
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
        todo!();
    }

    pub fn read_line(&self, buf: &mut [u8]) -> String {
        todo!();
    }
}

pub struct Stdout;

impl Stdout {
    fn new() -> Self {
        Self {}
    }

    pub fn write(&self, s: &str) {
        todo!();
    }
}

pub struct Stderr;

impl Stderr {
    fn new() -> Self {
        Self {}
    }

    pub fn write(&self, s: &str) {
        todo!();
    }
}

pub(super) macro kprint($($arg:tt)*) {
    console::console_print(format_args!($($arg)*))
}

pub macro print($($arg:tt)*) {
    let s = format!("{}", format_args!($($arg)*));
    STDOUT.lock().write(&s);
}

pub macro println {
    () => (print!("\n")),
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)))
}

macro eprint($($arg:tt)*) {
    let s = format!("{}", format_args!($($arg)*));
    STDERR.lock().write(&s);
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
