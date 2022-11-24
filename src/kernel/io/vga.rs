use core::{
    fmt::{self, Write},
    ops,
};
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
use x86_64::instructions::{interrupts, port::Port};

use super::console;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        cursor: [0; 2],
        writer: [0; 2],
        cc: ColorCode::new(Color::Yellow, Color::Black),
        buf: unsafe { &mut *(0xB8000 as *mut Buffer) },
    });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(fg: Color, bg: Color) -> Self {
        Self((bg as u8) << 4 | (fg as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct SChar {
    ascii: u8,
    cc: ColorCode,
}

impl ops::Deref for SChar {
    type Target = SChar;

    fn deref(&self) -> &Self::Target {
        self
    }
}

impl ops::DerefMut for SChar {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}

const BUF_HEIGHT: usize = 25;
const BUF_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<SChar>; BUF_WIDTH]; BUF_HEIGHT],
}

const CRTC_ADDR_REG: u16 = 0x3D4;
const CRTC_DATA_REG: u16 = 0x3D5;

const UNPRINTABLE: u8 = 0x00;

pub struct Writer {
    cursor: [usize; 2],
    writer: [usize; 2],
    cc: ColorCode,
    buf: &'static mut Buffer,
}

impl Writer {
    fn writer_pos(&self) -> (usize, usize) {
        (self.writer[0], self.writer[1])
    }

    fn set_writer_pos(&mut self, x: usize, y: usize) {
        self.writer = [x, y];
    }

    fn cursor_pos(&self) -> (usize, usize) {
        (self.cursor[0], self.cursor[1])
    }

    fn set_cursor_pos(&mut self, x: usize, y: usize) {
        self.cursor = [x, y];
        self.write_cursor();
    }

    fn write_cursor(&mut self) {
        let pos = self.cursor[0] + self.cursor[1] * BUF_WIDTH;
        let mut addr = Port::new(CRTC_ADDR_REG);
        let mut data = Port::new(CRTC_DATA_REG);

        unsafe {
            addr.write(0x0F_u8);
            data.write((pos & 0xFF) as u8);
            addr.write(0x0E_u8);
            data.write(((pos >> 8) & 0xFF) as u8);
        }
    }

    fn enable_cursor(&self) {
        let mut addr: Port<u8> = Port::new(CRTC_ADDR_REG);
        let mut data: Port<u8> = Port::new(CRTC_DATA_REG);
        let cursor_start = 13;
        let cursor_end = 14;

        unsafe {
            addr.write(0x0A);
            let b = data.read();
            data.write((b & 0xC0) | cursor_start);

            addr.write(0x0B);
            let b = data.read();
            data.write((b & 0xE0) | cursor_end);
        }
    }

    fn disable_cursor(&self) {
        let mut addr = Port::new(CRTC_ADDR_REG);
        let mut data = Port::new(CRTC_DATA_REG);

        unsafe {
            addr.write(0x0A_u8);
            data.write(0x20_u8);
        }
    }

    fn switch_echo(&self, cmd: &str) {
        console::switch_echo(cmd);
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            b'\r' => {}
            0x08 => {
                if self.writer[0] > 0 {
                    self.writer[0] -= 1;
                    let c = SChar {
                        ascii: b' ',
                        cc: self.cc,
                    };
                    let (x, y) = self.writer_pos();
                    self.buf.chars[y][x].write(c);
                }
            }
            byte => {
                if self.writer[0] >= BUF_WIDTH {
                    self.new_line();
                }

                let (x, y) = self.writer_pos();

                let ascii = if is_printable(byte) {
                    byte
                } else {
                    UNPRINTABLE
                };

                let cc = self.cc;
                let c = SChar { ascii, cc };
                self.buf.chars[y][x].write(c);
                self.writer[0] += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        s.bytes().for_each(|b| match b {
            0x20..=0x7e | b'\n' => self.write_byte(b),
            _ => self.write_byte(0xfe),
        })
    }

    fn new_line(&mut self) {
        if self.writer[1] < BUF_HEIGHT - 1 {
            self.writer[1] += 1;
        } else {
            for row in 1..BUF_HEIGHT {
                for col in 0..BUF_WIDTH {
                    let c = self.buf.chars[row][col].read();
                    self.buf.chars[row - 1][col].write(c);
                }
            }
            self.clear_row(0, BUF_HEIGHT - 1);
        }

        self.writer[0] = 0;
    }

    fn clear_row(&mut self, x: usize, row: usize) {
        let c = SChar {
            ascii: b' ',
            cc: self.cc,
        };
        for col in x..BUF_WIDTH {
            self.buf.chars[row][col].write(c);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

fn is_printable(c: u8) -> bool {
    matches!(c, 0x20..=0x7E | 0x08 | 0x0A | 0x0D | 0x7F..=0xFF)
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use x86_64::instructions::interrupts::without_interrupts;

    without_interrupts(|| {
        WRITER
            .lock()
            .write_fmt(args)
            .expect("Failed to write to VGA");
    });
}

pub macro print($($arg:tt)*) {
    (_print(format_args!($($arg)*)))
}

pub macro println {
    () => (print!("\n")),
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)))
}
