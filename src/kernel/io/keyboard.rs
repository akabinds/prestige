use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyEvent, Keyboard, ScancodeSet1};
use spin::Mutex;

pub static KEYBOARD: Mutex<Option<KeyboardLayout>> = Mutex::new(None);

pub(super) fn init() {
    // We can safely unwrap here because when building and running through the build script (`prestige.py`), the
    // KEYBOARD_LAYOUT environment variable will ALWAYS be set. This means that there will NEVER be a chance of a panic
    // due to unwrapping a None. The only reason `option_env!` is used instead of `env!` is to prevent the compiler
    // error that would be emitted because the environment variable wasn't present at the time of writing the code.
    let keyboard = KeyboardLayout::from(option_env!("KEYBOARD_LAYOUT").unwrap()).unwrap();
    *KEYBOARD.lock() = Some(keyboard);
}

pub enum KeyboardLayout {
    Azerty(Keyboard<layouts::Azerty, ScancodeSet1>),
    Dvorak(Keyboard<layouts::Dvorak104Key, ScancodeSet1>),
    Qwerty(Keyboard<layouts::Us104Key, ScancodeSet1>),
}

impl KeyboardLayout {
    pub fn add_byte(&mut self, scancode: u8) -> Result<Option<KeyEvent>, pc_keyboard::Error> {
        use KeyboardLayout::*;

        match self {
            Azerty(keyboard) => keyboard.add_byte(scancode),
            Dvorak(keyboard) => keyboard.add_byte(scancode),
            Qwerty(keyboard) => keyboard.add_byte(scancode),
        }
    }

    pub fn process_keyevent(&mut self, key_event: KeyEvent) -> Option<DecodedKey> {
        use KeyboardLayout::*;

        match self {
            Azerty(keyboard) => keyboard.process_keyevent(key_event),
            Dvorak(keyboard) => keyboard.process_keyevent(key_event),
            Qwerty(keyboard) => keyboard.process_keyevent(key_event),
        }
    }

    fn from(layout: &str) -> Option<Self> {
        use KeyboardLayout::*;

        match layout {
            "azerty" => Some(Azerty(Keyboard::new(HandleControl::MapLettersToUnicode))),
            "dvorak" => Some(Dvorak(Keyboard::new(HandleControl::MapLettersToUnicode))),
            "qwerty" => Some(Qwerty(Keyboard::new(HandleControl::MapLettersToUnicode))),
            _ => None,
        }
    }
}
