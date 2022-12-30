#[cfg(target_arch = "x86_64")]
use crate::kernel::arch::interrupts::halt;

use crate::kernel::{fs::FileIO, io::kprint};
use alloc::string::{String, ToString};
use core::{
    fmt,
    sync::atomic::{AtomicBool, Ordering},
};
use spin::Mutex;
use x86_64::instructions::interrupts as x86_64cint; // x86_64 crate interrupts

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

    fn foreground(color: &str) -> Self {
        Self {
            fg: Self::color_to_fg(color),
            bg: None,
        }
    }

    fn with_foreground(self, color: &str) -> Self {
        Self {
            fg: Self::color_to_fg(color),
            bg: self.bg,
        }
    }

    fn background(color: &str) -> Self {
        Self {
            fg: None,
            bg: Self::color_to_bg(color),
        }
    }

    fn with_background(self, color: &str) -> Self {
        Self {
            fg: self.fg,
            bg: Self::color_to_bg(color),
        }
    }

    pub fn color(color: &str) -> Self {
        Self::foreground(color)
    }

    fn with_color(self, color: &str) -> Self {
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
                write!(f, "\x1b[{fg};{bg}m")
            } else {
                write!(f, "\x1b[{fg}m")
            }
        } else if let Some(bg) = self.bg {
            write!(f, "\x1b[{bg}m")
        } else {
            write!(f, "\x1b[0m")
        }
    }
}

static INPUT: Mutex<String> = Mutex::new(String::new());
static ECHO: AtomicBool = AtomicBool::new(true);
static RAW: AtomicBool = AtomicBool::new(false);

pub(super) fn switch_echo(cmd: &str) {
    match cmd {
        "enable" => ECHO.store(true, Ordering::SeqCst),
        "disable" => ECHO.store(false, Ordering::SeqCst),
        _ => (),
    }
}

fn switch_raw(cmd: &str) {
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

#[derive(Debug, Clone)]
pub(crate) struct Console;

impl Console {
    pub(crate) fn new() -> Self {
        Self {}
    }

    fn read_char() -> char {
        switch_echo("disable");
        switch_raw("enable");

        loop {
            halt();

            let res = x86_64cint::without_interrupts(|| {
                let mut inp = INPUT.lock();

                if !inp.is_empty() {
                    Some(inp.remove(0))
                } else {
                    None
                }
            });

            if let Some(c) = res {
                switch_echo("enable");
                switch_raw("disable");
                return c;
            }
        }
    }

    fn read_line() -> String {
        loop {
            halt();

            let res = x86_64cint::without_interrupts(|| {
                let mut inp = INPUT.lock();

                match inp.chars().next_back() {
                    Some('\n') => {
                        let line = inp.clone();
                        inp.clear();
                        Some(line)
                    }
                    _ => None,
                }
            });

            if let Some(line) = res {
                return line;
            }
        }
    }
}

impl FileIO for Console {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()> {
        let mut s = if buf.len() == 4 {
            Self::read_char().to_string()
        } else {
            Self::read_line()
        };

        s.truncate(buf.len());
        let n = s.len();
        buf[0..n].copy_from_slice(s.as_bytes());
        Ok(n)
    }

    fn write(&mut self, buf: &[u8]) -> Result<usize, ()> {
        let s = String::from_utf8_lossy(buf);
        let n = s.len();
        kprint!("{s}");
        Ok(n)
    }
}

const ETXT: char = '\x03';
const EOT: char = '\x04';
const BACKSPACE: char = '\x08';
const ESC: char = '\x1b';

pub(crate) fn handle_key_inp(key: char) {
    let mut inp = INPUT.lock();

    if key == BACKSPACE && !is_enabled("raw") {
        if let Some(c) = inp.pop() {
            if is_enabled("echo") {
                let n = match c {
                    ETXT | EOT | ESC => 2,
                    _ => {
                        if (c as u32) < 0xFF {
                            1
                        } else {
                            c.len_utf8()
                        }
                    }
                };

                console_print(format_args!("{}", BACKSPACE.to_string().repeat(n)));
            }
        }
    } else {
        let key = if (key as u32) < 0xFF {
            (key as u8) as char
        } else {
            key
        };

        inp.push(key);

        if is_enabled("echo") {
            match key {
                ETXT => console_print(format_args!("^C")),
                EOT => console_print(format_args!("^D")),
                ESC => console_print(format_args!("^[")),
                _ => console_print(format_args!("{key}")),
            }
        }
    }
}

#[doc(hidden)]
pub fn console_print(args: fmt::Arguments) {
    #[cfg(feature = "vga")]
    {
        super::vga::vga_print(args);
    }

    #[cfg(feature = "serial")]
    {
        super::serial::serial_print(args);
    }
}
