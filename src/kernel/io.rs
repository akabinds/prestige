pub mod keyboard;
pub mod serial;
mod terminal;

pub fn init() {
    serial::init();
    keyboard::init();
}

pub macro print($($t:tt)*) {
    terminal::_print(format_args!($($t)*))
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
