pub mod keyboard;
pub mod serial;
pub mod vga;

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

