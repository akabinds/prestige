use core::{
    fmt::{self, Write},
    ops,
};
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        col_pos: 0,
        cc: ColorCode::new(Color::Yellow, Color::Black),
        buf: unsafe { &mut *(0xb8000 as *mut Buffer) },
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

pub struct Writer {
    col_pos: usize,
    cc: ColorCode,
    buf: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col_pos >= BUF_WIDTH {
                    self.new_line();
                }

                let row = BUF_HEIGHT - 1;
                let col = self.col_pos;

                let cc = self.cc;
                self.buf.chars[row][col].write(SChar { ascii: byte, cc });
                self.col_pos += 1;
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
        for row in 1..BUF_HEIGHT {
            for col in 0..BUF_WIDTH {
                let character = self.buf.chars[row][col].read();
                self.buf.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUF_HEIGHT - 1);
        self.col_pos = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = SChar {
            ascii: b' ',
            cc: self.cc,
        };
        for col in 0..BUF_WIDTH {
            self.buf.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn _std_out(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}

pub macro print($($arg:tt)*) {
    (_std_out(format_args!($($arg)*)))
}

pub macro println {
    () => (print!("\n")),
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)))
}
