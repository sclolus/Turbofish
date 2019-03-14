use crate::drivers::keyboard::keysymb::KeySymb;
use crate::drivers::keyboard::{CharType::RequestKeySymb, KEYBOARD_DRIVER};

#[derive(Debug)]
pub struct Terminal {}

pub static mut TERMINAL: Option<Terminal> = None;

impl Terminal {
    pub fn new() -> Self {
        unsafe {
            KEYBOARD_DRIVER.as_mut().unwrap().bind(RequestKeySymb(<Self>::display_char));
        }
        Self {}
    }
    fn display_char(key_symb: KeySymb) {
        match key_symb {
            KeySymb::Return => println!(""),
            _ => {
                if (key_symb >= KeySymb::space) && (key_symb <= KeySymb::asciitilde) {
                    print!("{}", key_symb as u32 as u8 as char);
                }
            }
        }
    }
}

pub fn init_terminal() {
    unsafe {
        TERMINAL = Some(Terminal::new());
    }
}
