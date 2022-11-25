use super::{vga::print, Stdin, STDIN};
use crate::kernel::fs::FileIO;
use alloc::{
    string::{String, ToString},
    vec,
};
use core::{
    fmt,
    sync::atomic::{AtomicBool, Ordering},
};

pub static ECHO: AtomicBool = AtomicBool::new(true);
pub static RAW: AtomicBool = AtomicBool::new(false);

pub fn switch_echo(cmd: &str) {
    match cmd {
        "enable" => ECHO.store(true, Ordering::SeqCst),
        "disable" => ECHO.store(false, Ordering::SeqCst),
        _ => (),
    }
}

pub fn switch_raw(cmd: &str) {
    match cmd {
        "enable" => RAW.store(true, Ordering::SeqCst),
        "disable" => RAW.store(false, Ordering::SeqCst),
        _ => (),
    }
}

fn is_enabled(mode: &str) -> bool {
    match mode {
        "echo" => ECHO.load(Ordering::SeqCst),
        "raw" => RAW.load(Ordering::SeqCst),
        _ => false,
    }
}

#[derive(Clone, Copy)]
pub struct Style {
    fg: Option<usize>,
    bg: Option<usize>,
}

impl Style {
    fn color_to_fg(color: &str) -> Option<usize> {
        match color {
            "Black" => Some(30),
            "Red" => Some(31),
            "Green" => Some(32),
            "Brown" => Some(33),
            "Blue" => Some(34),
            "Magenta" => Some(35),
            "Cyan" => Some(36),
            "LightGray" => Some(37),
            "DarkGray" => Some(90),
            "LightRed" => Some(91),
            "LightGreen" => Some(92),
            "Yellow" => Some(93),
            "LightBlue" => Some(94),
            "Pink" => Some(95),
            "LightCyan" => Some(96),
            "White" => Some(97),
            _ => None,
        }
    }

    fn color_to_bg(color: &str) -> Option<usize> {
        Self::color_to_fg(color).map(|fg| fg + 10)
    }

    pub fn foreground(color: &str) -> Self {
        Self {
            fg: Self::color_to_fg(color),
            bg: None,
        }
    }

    pub fn with_foreground(self, color: &str) -> Self {
        Self {
            fg: Self::color_to_fg(color),
            bg: self.bg,
        }
    }

    pub fn background(color: &str) -> Self {
        Self {
            fg: None,
            bg: Self::color_to_bg(color),
        }
    }

    pub fn with_background(self, color: &str) -> Self {
        Self {
            fg: self.fg,
            bg: Self::color_to_bg(color),
        }
    }

    pub fn color(color: &str) -> Self {
        Self::foreground(color)
    }

    pub fn with_color(self, color: &str) -> Self {
        self.with_foreground(color)
    }

    pub fn reset() -> Self {
        Self { fg: None, bg: None }
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(fg) = self.fg {
            if let Some(bg) = self.bg {
                write!(f, "\x1b[{};{}m", fg, bg)
            } else {
                write!(f, "\x1b[{}m", fg)
            }
        } else if let Some(bg) = self.bg {
            write!(f, "\x1b[{}m", bg)
        } else {
            write!(f, "\x1b[0m")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Console;

impl Console {
    pub fn new() -> Self {
        Self {}
    }
}

impl FileIO for Console {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()> {
        let stdin = STDIN.lock();

        let mut s = if buf.len() == 4 {
            stdin.read_char(&mut buf.to_vec()).unwrap().to_string()
        } else {
            stdin.read_line(&mut buf.to_vec())
        };

        s.truncate(buf.len());
        let n = s.len();
        buf[0..n].copy_from_slice(s.as_bytes());
        Ok(n)
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize, ()> {
        let s = String::from_utf8_lossy(buf);
        let n = s.len();
        print!("{s}");
        Ok(n)
    }
}

const ETXT: char = '\x03';
const EOT: char = '\x04';
const BACKSPACE: char = '\x08';
const ESC: char = '\x1b';

pub fn handle_key_inp(key: char) {
    let mut stdin = STDIN.lock();

    if key == BACKSPACE && !is_enabled("raw") {
        todo!();
    } else {
        let key = if (key as u32) < 0xFF {
            (key as u8) as char
        } else {
            key
        };

        stdin.read_char(&mut vec![0; 4]);

        if is_enabled("echo") {
            match key {
                ETXT => print!("^C"),
                EOT => print!("^D"),
                ESC => print!("^["),
                _ => print!("{key}"),
            }
        }
    }
}
