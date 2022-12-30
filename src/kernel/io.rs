pub(crate) mod serial;
mod terminal;

use core::fmt;
use limine::LimineTerminalRequest;
use spin::Mutex;

static TERMINAL_REQUEST: LimineTerminalRequest = LimineTerminalRequest::new(0);

struct Writer {
    terminals: Option<&'static limine::LimineTerminalResponse>,
}

unsafe impl Send for Writer {}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // Get the Terminal response and cache it.
        let response = match self.terminals {
            None => {
                let response = TERMINAL_REQUEST.get_response().get().ok_or(fmt::Error)?;
                self.terminals = Some(response);
                response
            }
            Some(resp) => resp,
        };

        let write = response.write().ok_or(fmt::Error)?;

        // Output the string onto each terminal.
        for terminal in response.terminals() {
            write(terminal, s);
        }

        Ok(())
    }
}

static WRITER: Mutex<Writer> = Mutex::new(Writer { terminals: None });

pub fn _print(args: fmt::Arguments) {
    // NOTE: Locking needs to happen around `print_fmt`, not `print_str`, as the former
    // will call the latter potentially multiple times per invocation.
    let mut writer = WRITER.lock();
    fmt::Write::write_fmt(&mut *writer, args).ok();
}

pub macro print($($t:tt)*) {
    _print(format_args!($($t)*))
}

pub macro println {
    ()          => (print!("\n")),
    ($($t:tt)*) => (print!("{}\n", format_args!($($t)*)))
}

pub macro serial_print($($t:tt)*) {
    serial::serial_print(format_args!($($t)*))
}

pub macro serial_println {
    ()          => (serial_print!("\n")),
    ($($t:tt)*) => (serial_print!("{}\n", format_args!($($t)*)))
}
