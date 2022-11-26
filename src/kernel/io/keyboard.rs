use core::sync::atomic::AtomicBool;

use super::console;
use pc_keyboard::{layouts, DecodedKey, Error, HandleControl, KeyEvent, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::instructions::port::Port;

pub(crate) static KEYBOARD: Mutex<Option<KeyboardLayout>> = Mutex::new(None);
pub(crate) static ALT: AtomicBool = AtomicBool::new(false);
pub(crate) static CTRL: AtomicBool = AtomicBool::new(false);
pub(crate) static SHIFT: AtomicBool = AtomicBool::new(false);

pub(crate) fn init() {
    set_kbd_layout(option_env!("KBD_LAYOUT").unwrap_or("qwerty"));
}

pub(crate) enum KeyboardLayout {
    Azerty(Keyboard<layouts::Azerty, ScancodeSet1>),
    Dvorak(Keyboard<layouts::Dvorak104Key, ScancodeSet1>),
    Qwerty(Keyboard<layouts::Us104Key, ScancodeSet1>),
}

impl KeyboardLayout {
    pub(crate) fn add_byte(&mut self, scancode: u8) -> Result<Option<KeyEvent>, Error> {
        match self {
            KeyboardLayout::Azerty(keyboard) => keyboard.add_byte(scancode),
            KeyboardLayout::Dvorak(keyboard) => keyboard.add_byte(scancode),
            KeyboardLayout::Qwerty(keyboard) => keyboard.add_byte(scancode),
        }
    }

    pub(crate) fn process_keyevent(&mut self, key_event: KeyEvent) -> Option<DecodedKey> {
        match self {
            KeyboardLayout::Azerty(keyboard) => keyboard.process_keyevent(key_event),
            KeyboardLayout::Dvorak(keyboard) => keyboard.process_keyevent(key_event),
            KeyboardLayout::Qwerty(keyboard) => keyboard.process_keyevent(key_event),
        }
    }

    fn from(n: &str) -> Option<Self> {
        use KeyboardLayout::*;

        match n {
            "azerty" => Some(Azerty(Keyboard::new(HandleControl::MapLettersToUnicode))),
            "dvorak" => Some(Dvorak(Keyboard::new(HandleControl::MapLettersToUnicode))),
            "qwerty" => Some(Qwerty(Keyboard::new(HandleControl::MapLettersToUnicode))),
            _ => None,
        }
    }
}

pub(crate) fn set_kbd_layout(layout: &str) {
    if let Some(kbd) = KeyboardLayout::from(layout) {
        *KEYBOARD.lock() = Some(kbd);
    }
}

pub(crate) fn read_scancode() -> u8 {
    let mut port = Port::new(0x60);
    unsafe { port.read() }
}

pub(crate) fn send_key(k: char) {
    console::handle_key_inp(k);
}

pub(crate) fn send_csi(c: char) {
    send_key('\x1B');
    send_key('[');
    send_key(c);
}
