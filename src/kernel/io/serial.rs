use super::{console, PARSER};
use core::fmt::{self, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;
use vte::{Params, Perform};

pub fn init() {
    SERIAL.lock().init();
}

lazy_static! {
    pub static ref SERIAL: Mutex<Serial> = Mutex::new(Serial::new(0x3F8));
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

    pub fn read_byte(&mut self) -> u8 {
        self.port.receive()
    }

    fn write_byte(&mut self, byte: u8) {
        self.port.send(byte);
    }
}

impl Perform for Serial {
    fn csi_dispatch(&mut self, params: &Params, _: &[u8], _: bool, c: char) {
        match c {
            'h' => {
                for param in params.iter() {
                    match param[0] {
                        12 => console::switch_echo("enable"),
                        _ => return,
                    }
                }
            }
            'l' => {
                for param in params.iter() {
                    match param[0] {
                        12 => console::switch_echo("disable"),
                        _ => return,
                    }
                }
            }
            _ => {}
        }
    }
}

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
pub fn serial_print(args: fmt::Arguments) {
    use x86_64::instructions::interrupts::without_interrupts;

    without_interrupts(|| {
        SERIAL
            .lock()
            .write_fmt(args)
            .expect("Failed to write to serial port")
    });
}
