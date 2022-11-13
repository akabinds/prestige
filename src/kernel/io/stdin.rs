use alloc::string::String;
use spin::Mutex;

pub static STDIN: Mutex<String> = Mutex::new(String::new());
