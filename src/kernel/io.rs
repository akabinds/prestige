pub mod keyboard;
pub mod serial;
pub mod stdin;
pub mod vga;

use lazy_static::lazy_static;
use spin::Mutex;
use vte::Parser;

lazy_static! {
    pub static ref PARSER: Mutex<Parser> = Mutex::new(Parser::new());
}
