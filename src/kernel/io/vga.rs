use super::{console, PARSER};
use bit_field::BitField;
use core::{
    fmt::{self, Write},
    ops,
};
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
use vte::{Params, Perform};
use x86_64::instructions::{interrupts as x86_64cint, port::Port};

pub(crate) fn init() {
    set_attr_ctrl_reg(0x0, 0x00);
    set_attr_ctrl_reg(0x1, 0x01);
    set_attr_ctrl_reg(0x2, 0x02);
    set_attr_ctrl_reg(0x3, 0x03);
    set_attr_ctrl_reg(0x4, 0x04);
    set_attr_ctrl_reg(0x5, 0x05);
    set_attr_ctrl_reg(0x6, 0x14);
    set_attr_ctrl_reg(0x7, 0x07);
    set_attr_ctrl_reg(0x8, 0x38);
    set_attr_ctrl_reg(0x9, 0x39);
    set_attr_ctrl_reg(0xA, 0x3A);
    set_attr_ctrl_reg(0xB, 0x3B);
    set_attr_ctrl_reg(0xC, 0x3C);
    set_attr_ctrl_reg(0xD, 0x3D);
    set_attr_ctrl_reg(0xE, 0x3E);
    set_attr_ctrl_reg(0xF, 0x3F);

    x86_64cint::without_interrupts(|| {
        WRITER.lock().set_palette(Palette::default());
    });

    let reg = 0x10;
    let mut attr = get_attr_ctrl_reg(reg);
    attr.set_bit(3, false);
    set_attr_ctrl_reg(reg, attr);

    set_underline_location(0x1F);

    WRITER.lock().clear_screen();
}

lazy_static! {
    static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        cursor: [0; 2],
        writer: [0; 2],
        cc: ColorCode::new(FG, BG),
        buf: unsafe { &mut *(0xB8000 as *mut Buffer) },
    });
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
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

const FG: Color = Color::LightGray;
const BG: Color = Color::Black;

const COLORS: [Color; 16] = [
    Color::Black,
    Color::Blue,
    Color::Green,
    Color::Cyan,
    Color::Red,
    Color::Magenta,
    Color::Brown,
    Color::LightGray,
    Color::DarkGray,
    Color::LightBlue,
    Color::LightGreen,
    Color::LightCyan,
    Color::LightRed,
    Color::Pink,
    Color::Yellow,
    Color::White,
];

impl Color {
    fn from_idx(idx: usize) -> Self {
        COLORS[idx]
    }

    fn from_ansi(code: u8) -> Self {
        use Color::*;

        match code {
            30 => Black,
            31 => Red,
            32 => Green,
            33 => Brown,
            34 => Blue,
            35 => Magenta,
            36 => Cyan,
            37 => LightGray,
            90 => DarkGray,
            91 => LightRed,
            92 => LightGreen,
            93 => Yellow,
            94 => LightBlue,
            95 => Pink,
            96 => LightCyan,
            97 => White,
            _ => Black,
        }
    }

    fn to_vga_reg(self) -> u8 {
        use Color::*;

        match self {
            Black => 0x00,
            Blue => 0x01,
            Green => 0x02,
            Cyan => 0x03,
            Red => 0x04,
            Magenta => 0x05,
            Brown => 0x14,
            LightGray => 0x07,
            DarkGray => 0x38,
            LightBlue => 0x39,
            LightGreen => 0x3A,
            LightCyan => 0x3B,
            LightRed => 0x3C,
            Pink => 0x3D,
            Yellow => 0x3E,
            White => 0x3F,
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ColorCode(u8);

impl ColorCode {
    fn new(fg: Color, bg: Color) -> Self {
        Self((bg as u8) << 4 | (fg as u8))
    }
}

struct Palette {
    colors: [(u8, u8, u8); 16],
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            colors: [
                (0x00, 0x00, 0x00),
                (0x00, 0x00, 0x80),
                (0x00, 0x80, 0x00),
                (0x00, 0x80, 0x80),
                (0x80, 0x00, 0x00),
                (0x80, 0x00, 0x80),
                (0x80, 0x80, 0x00),
                (0xC0, 0xC0, 0xC0),
                (0x80, 0x80, 0x80),
                (0x00, 0x00, 0xFF),
                (0x00, 0xFF, 0x00),
                (0x00, 0xFF, 0xFF),
                (0xFF, 0x00, 0x00),
                (0xFF, 0x00, 0xFF),
                (0xFF, 0xFF, 0x00),
                (0xFF, 0xFF, 0xFF),
            ],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

const ATTR_ADDR_DATA_REG: u16 = 0x3C0;
const ATTR_DATA_READ_REG: u16 = 0x3C1;
const DAC_ADDR_WRITE_MODE_REG: u16 = 0x3C8;
const DAC_DATA_REG: u16 = 0x3C9;
const CRTC_ADDR_REG: u16 = 0x3D4;
const CRTC_DATA_REG: u16 = 0x3D5;
const INPUT_STATUS_REG: u16 = 0x3DA;

const UNPRINTABLE: u8 = 0x00;

struct Writer {
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

    fn write_byte(&mut self, byte: u8) {
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

            self.clear_row_after(0, BUF_HEIGHT - 1);
        }

        self.writer[0] = 0;
    }

    fn clear_row_before(&mut self, x: usize, row: usize) {
        todo!();
    }

    fn clear_row_after(&mut self, x: usize, row: usize) {
        let c = SChar {
            ascii: b' ',
            cc: self.cc,
        };

        for col in x..BUF_WIDTH {
            self.buf.chars[row][col].write(c);
        }
    }

    fn clear_screen(&mut self) {
        for y in 0..BUF_HEIGHT {
            self.clear_row_after(0, y);
        }
    }

    fn set_color(&mut self, fg: Color, bg: Color) {
        self.cc = ColorCode::new(fg, bg);
    }

    fn set_palette(&mut self, palette: Palette) {
        let mut addr = Port::new(DAC_ADDR_WRITE_MODE_REG);
        let mut data = Port::new(DAC_DATA_REG);

        for (i, (r, g, b)) in palette.colors.iter().enumerate() {
            if i < 16 {
                let reg = Color::from_idx(i).to_vga_reg();

                unsafe {
                    addr.write(reg);
                    data.write(*r >> 2);
                    data.write(*g >> 2);
                    data.write(*b >> 2);
                }
            }
        }
    }
}

impl Perform for Writer {
    fn print(&mut self, c: char) {
        self.write_byte(c as u8);
    }

    fn execute(&mut self, byte: u8) {
        self.write_byte(byte);
    }

    fn csi_dispatch(&mut self, params: &Params, _: &[u8], _: bool, c: char) {
        match c {
            'm' => {
                let (mut fg, mut bg) = (FG, BG);

                for param in params.iter() {
                    match param[0] {
                        0 => {
                            fg = FG;
                            bg = BG;
                        }
                        30..=37 | 90..=97 => fg = Color::from_ansi(param[0] as u8),
                        40..=47 | 100..=107 => {
                            bg = Color::from_ansi((param[0] as u8) - 10);
                        }
                        _ => {}
                    }
                }

                self.set_color(fg, bg);
            }
            'A' => {
                let mut n = 1;

                for param in params.iter() {
                    n = param[0] as usize;
                }

                self.writer[1] -= n;
                self.cursor[1] -= n;
            }
            'B' => {
                let mut n = 1;

                for param in params.iter() {
                    n = param[0] as usize;
                }

                self.writer[1] += n;
                self.cursor[1] += n;
            }
            'C' => {
                let mut n = 1;

                for param in params.iter() {
                    n = param[0] as usize;
                }

                self.writer[0] += n;
                self.cursor[0] += n;
            }
            'D' => {
                let mut n = 1;

                for param in params.iter() {
                    n = param[0] as usize;
                }

                self.writer[0] -= n;
                self.cursor[0] -= n;
            }
            'G' => {
                let (_, y) = self.cursor_pos();
                let mut x = 1;

                for param in params.iter() {
                    x = param[0] as usize;
                }

                if x > BUF_WIDTH {
                    return;
                }

                self.set_writer_pos(x - 1, y);
                self.set_cursor_pos(x - 1, y);
            }
            'H' => {
                let mut x = 1;
                let mut y = 1;

                for (i, param) in params.iter().enumerate() {
                    match i {
                        0 => y = param[0] as usize,
                        1 => x = param[0] as usize,
                        _ => break,
                    };
                }

                if x > BUF_WIDTH || y > BUF_HEIGHT {
                    return;
                }

                self.set_writer_pos(x - 1, y - 1);
                self.set_cursor_pos(x - 1, y - 1);
            }
            'J' => {
                let mut n = 0;

                for param in params.iter() {
                    n = param[0] as usize;
                }

                match n {
                    // TODO: 0 and 1, from cursor to begining or to end of screen
                    2 => self.clear_screen(),
                    _ => return,
                }

                self.set_writer_pos(0, 0);
                self.set_cursor_pos(0, 0);
            }
            'K' => {
                let (x, y) = self.cursor_pos();
                let mut n = 0;

                for param in params.iter() {
                    n = param[0] as usize;
                }

                match n {
                    0 => self.clear_row_after(x, y),
                    // 1 => self.clear_row_before(x, y),
                    2 => self.clear_row_after(0, y),
                    _ => return,
                }

                self.set_writer_pos(x, y);
                self.set_cursor_pos(x, y);
            }
            'h' => {
                for param in params.iter() {
                    match param[0] {
                        12 => self.switch_echo("enable"),
                        25 => self.enable_cursor(),
                        _ => return,
                    }
                }
            }
            'l' => {
                for param in params.iter() {
                    match param[0] {
                        12 => self.switch_echo("disable"),
                        25 => self.disable_cursor(),
                        _ => return,
                    }
                }
            }
            _ => {}
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut parser = PARSER.lock();

        for byte in s.bytes() {
            parser.advance(self, byte);
        }

        let (x, y) = self.writer_pos();
        self.set_cursor_pos(x, y);

        Ok(())
    }
}

fn is_printable(c: u8) -> bool {
    matches!(c, 0x20..=0x7E | 0x08 | 0x0A | 0x0D | 0x7F..=0xFF)
}

fn get_attr_ctrl_reg(idx: u8) -> u8 {
    x86_64cint::without_interrupts(|| {
        let mut isr: Port<u8> = Port::new(INPUT_STATUS_REG);
        let mut addr: Port<u8> = Port::new(ATTR_ADDR_DATA_REG);
        let mut data: Port<u8> = Port::new(ATTR_DATA_READ_REG);

        unsafe {
            isr.read();
            let tmp = addr.read();
            addr.write(idx | 0x20);
            let res = data.read();
            addr.write(tmp);
            res
        }
    })
}

fn set_attr_ctrl_reg(idx: u8, value: u8) {
    x86_64cint::without_interrupts(|| {
        let mut isr: Port<u8> = Port::new(INPUT_STATUS_REG);
        let mut addr: Port<u8> = Port::new(ATTR_ADDR_DATA_REG);

        unsafe {
            isr.read();
            let tmp = addr.read();
            addr.write(idx);
            addr.write(value);
            addr.write(tmp);
        }
    });
}

fn set_underline_location(location: u8) {
    x86_64cint::without_interrupts(|| {
        let mut addr: Port<u8> = Port::new(CRTC_ADDR_REG);
        let mut data: Port<u8> = Port::new(CRTC_DATA_REG);

        unsafe {
            addr.write(0x14);
            data.write(location);
        }
    });
}

#[doc(hidden)]
pub(super) fn vga_print(args: fmt::Arguments) {
    x86_64cint::without_interrupts(|| {
        WRITER
            .lock()
            .write_fmt(args)
            .expect("Failed to write to VGA");
    });
}
