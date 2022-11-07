use core::fmt::{self, Write};
use limine::LimineTerminalRequest;
use spin::Mutex;

static TERM_REQ: LimineTerminalRequest = LimineTerminalRequest::new(0);

struct Writer {
    terminals: Option<&'static limine::LimineTerminalResponse>,
}

unsafe impl Send for Writer {}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let response = match self.terminals {
            None => {
                let response = TERM_REQ.get_response().get().ok_or(fmt::Error)?;
                self.terminals = Some(response);
                response
            }
            Some(resp) => resp,
        };

        let write = response.write().ok_or(fmt::Error)?;

        for terminal in response.terminals() {
            write(terminal, s);
        }

        Ok(())
    }
}

static WRITER: Mutex<Writer> = Mutex::new(Writer { terminals: None });

#[doc(hidden)]
pub fn _std_out(args: fmt::Arguments) {
    use x86_64::instructions::interrupts::without_interrupts;

    without_interrupts(|| WRITER.lock().write_fmt(args).ok());
}

pub macro print($($arg:tt)*) {
    _std_out(format_args!($($arg)*));
}

pub macro println {
    () => (print!("\n")),
    ($($arg:tt)*) => (print!("{}", format_args_nl!($($arg)*)))
}
