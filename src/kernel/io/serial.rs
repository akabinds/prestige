use core::fmt::{self, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _serial_std_out(args: fmt::Arguments) {
    SERIAL1.lock().write_fmt(args).ok();
}

pub macro serial_print($($arg:tt)*) {
    _serial_std_out(format_args!($($arg)*));
}

pub macro serial_println {
    () => (serial_print!("\n")),
    ($fmt:expr) => (serial_print!(concat!($fmt, "\n"))),
    ($fmt:expr, $($arg:tt)*) => (serial_print!(concat!($fmt, "\n"), $($arg)*))
}
