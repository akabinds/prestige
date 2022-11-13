use core::fmt::{self, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;
use vte::Perform;

use super::PARSER;

lazy_static! {
    static ref SERIAL: Mutex<Serial> = {
        let mut serial = Serial::new(0x3F8);
        serial.init();
        Mutex::new(serial)
    };
}

pub struct Serial {
    port: SerialPort,
}

impl Serial {
    fn new(addr: u16) -> Self {
        Self {
            port: unsafe { SerialPort::new(addr) },
        }
    }

    fn init(&mut self) {
        self.port.init();
    }

    fn read_byte(&mut self) -> u8 {
        self.port.receive()
    }

    fn write_byte(&mut self, byte: u8) {
        self.port.send(byte);
    }
}

impl Perform for Serial {}

impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut parser = PARSER.lock();

        for byte in s.bytes() {
            parser.advance(self, byte);
            self.write_byte(byte);
        }

        Ok(())
    }
}

#[doc(hidden)]
pub fn _serial_print(args: fmt::Arguments) {
    use x86_64::instructions::interrupts::without_interrupts;

    without_interrupts(|| {
        SERIAL
            .lock()
            .write_fmt(args)
            .expect("Failed to write to serial port")
    });
}

pub macro serial_print($($arg:tt)*) {
    _serial_print(format_args!($($arg)*));
}

pub macro serial_println {
    () => (serial_print!("\n")),
    ($fmt:expr) => (serial_print!(concat!($fmt, "\n"))),
    ($fmt:expr, $($arg:tt)*) => (serial_print!(concat!($fmt, "\n"), $($arg)*))
}
