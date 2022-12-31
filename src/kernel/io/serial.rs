use core::fmt::{self, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

#[cfg(target_arch = "x86_64")]
use x86_64::instructions::interrupts as x86_64cint; // x86_64 crate interrupts

pub fn init() {
    SERIAL.lock().init();
}

lazy_static! {
    pub static ref SERIAL: Mutex<Serial> = Mutex::new(Serial::new(0x3F8));
}

/// Wrapper around a serial port
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

    pub fn read_byte(&mut self) -> u8 {
        self.port.receive()
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.port.send(byte);
    }
}

impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }

        Ok(())
    }
}

#[doc(hidden)]
pub fn serial_print(args: fmt::Arguments) {
    SERIAL
        .lock()
        .write_fmt(args)
        .expect("Failed to write to serial port")
}
