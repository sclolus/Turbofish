//! See [PS/2 Keyboard](https://wiki.osdev.org/Keyboard)
use crate::io::{Io, Pio};
pub mod keysymb;
//use crate::keyboard::keysymb::KEYCODE_TO_KEYSYMB_AZERTY as KEYMAP;
use crate::keyboard::keysymb::KEYCODE_TO_KEYSYMB_QWERTY as KEYMAP;
use crate::keyboard::keysymb::{CapsLockSensitive, KeySymb};

#[allow(dead_code)]
struct Ps2Controler {
    data: Pio<u8>,
    /// command port unused for the moment
    _command: Pio<u8>,
    /// stock the current bytes of the scancode being read
    current_scancode: Option<u32>,
}

static mut PS2_CONTROLER: Ps2Controler = Ps2Controler::new();

impl Ps2Controler {
    pub const fn new() -> Self {
        Ps2Controler { data: Pio::new(0x60), _command: Pio::new(0x64), current_scancode: None }
    }
    // TODO or NOT TODO: it handle only escape sequence 0xE0, 0xE0 0xF0 for the moment
    /// read one byte on data port, return an entire scancode if any
    pub fn read_scancode(&mut self) -> Option<u32> {
        let key = self.data.read();
        match self.current_scancode {
            None => {
                // first escape code
                if key == 0xE0 {
                    self.current_scancode = Some(key as u32);
                    None
                } else {
                    Some(key as u32)
                }
            }
            Some(curr) => {
                // second escape code
                if key == 0xF0 {
                    self.current_scancode = Some((curr << 8) + key as u32);
                    None
                } else {
                    self.current_scancode = None;
                    Some((curr << 8) + key as u32)
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum KeyCode {
    Pressed(u8),
    Released(u8),
}

impl KeyCode {
    // TODO or NOT TODO: add more conversion
    /// generated with showkey and showkey -s
    #[cfg_attr(rustfmt, rustfmt_skip)]
    const ESCAPED_SCANCODE_TO_KEYCODE: [u8; 0x80] = [
        /*e0 00:*/    0,   0,   0,   0,   0,   0,   0,   0,
        /*e0 08:*/    0,   0,   0,   0,   0,   0,   0,   0,
        /*e0 10:*/    0,   0,   0,   0,   0,   0,   0,   0,
        /*e0 18:*/    0,   0,   0,   0,   0,   0,   0,   0,
        /*e0 20:*/    0,   0,   0,   0,   0,   0,   0,   0,
        /*e0 28:*/    0,   0,   0,   0,   0,   0,   0,   0,
        /*e0 30:*/    0,   0,   0,   0,   0,   0,   0,   0,
        /*e0 38:*/  100,   0,   0,   0,   0,   0,   0,   0,
        /*e0 40:*/    0,   0,   0,   0,   0,   0,   0,   0,
        /*e0 48:*/  103,   0,   0, 105,   0, 106,   0,   0,
        /*e0 50:*/  108,   0,   0,   0,   0,   0,   0,   0,
        /*e0 58:*/    0,   0,   0,   0,   0,   0,   0,   0,
        /*e0 60:*/    0,   0,   0,   0,   0,   0,   0,   0,
        /*e0 68:*/    0,   0,   0,   0,   0,   0,   0,   0,
        /*e0 70:*/    0,   0,   0,   0,   0,   0,   0,   0,
        /*e0 78:*/    0,   0,   0,   0,   0,   0,   0,   0,
    ];
    /// transform the multibyte scancode into One byte Keycode
    pub fn from_scancode(scancode: u32) -> Option<Self> {
        if scancode >= 0x1 && scancode <= 0x58 {
            return Some(KeyCode::Pressed(scancode as u8));
        }
        if scancode >= 0x81 && scancode <= 0xd8 {
            return Some(KeyCode::Released((scancode - 0x80) as u8));
        }
        if scancode >= 0xe010 && scancode <= 0xe06d {
            return Some(KeyCode::Pressed(Self::ESCAPED_SCANCODE_TO_KEYCODE[(scancode & 0xFF) as usize] as u8));
        }
        if scancode >= 0xe090 && scancode <= 0xe0ed {
            return Some(KeyCode::Released(
                (Self::ESCAPED_SCANCODE_TO_KEYCODE[((scancode & 0xFF) - 0x80) as usize]) as u8,
            ));
        }
        None
    }
}
#[allow(dead_code)]
enum EscapeKeyMask {
    Shift = 1,
    Altgr = 2,
    Control = 4,
    Alt = 8,
}

struct KeyboardDriver {
    escape_key_mask: u8,
    capslock: bool,
}

static mut KEYBOARD_DRIVER: KeyboardDriver = KeyboardDriver::new();

impl KeyboardDriver {
    pub const fn new() -> Self {
        Self { escape_key_mask: 0, capslock: false }
    }

    pub fn keycode_to_keymap(&mut self, keycode: KeyCode) -> Option<KeySymb> {
        match keycode {
            KeyCode::Pressed(k) => {
                let symb = &KEYMAP[k as usize][(self.escape_key_mask) as usize];
                match symb {
                    CapsLockSensitive::No(KeySymb::Control) => {
                        self.escape_key_mask |= EscapeKeyMask::Control as u8;
                        None
                    }
                    CapsLockSensitive::No(KeySymb::Shift) => {
                        self.escape_key_mask |= EscapeKeyMask::Shift as u8;
                        None
                    }
                    CapsLockSensitive::No(KeySymb::Alt) => {
                        self.escape_key_mask |= EscapeKeyMask::Alt as u8;
                        None
                    }
                    CapsLockSensitive::No(KeySymb::AltGr) => {
                        self.escape_key_mask |= EscapeKeyMask::Altgr as u8;
                        None
                    }
                    CapsLockSensitive::No(KeySymb::CtrlL_Lock) => {
                        self.capslock = !self.capslock;
                        None
                    }
                    CapsLockSensitive::No(other) => Some(*other),
                    CapsLockSensitive::Yes(_) => {
                        let symb = &KEYMAP[k as usize][(self.escape_key_mask ^ self.capslock as u8) as usize];
                        match symb {
                            CapsLockSensitive::No(other) | CapsLockSensitive::Yes(other) => Some(*other),
                        }
                    }
                }
            }
            KeyCode::Released(k) => {
                let symb = &KEYMAP[k as usize][(self.escape_key_mask ^ self.capslock as u8) as usize];
                match symb {
                    CapsLockSensitive::No(KeySymb::Control) => {
                        self.escape_key_mask &= !(EscapeKeyMask::Control as u8);
                    }
                    CapsLockSensitive::No(KeySymb::Shift) => {
                        self.escape_key_mask &= !(EscapeKeyMask::Shift as u8);
                    }
                    CapsLockSensitive::No(KeySymb::Alt) => {
                        self.escape_key_mask &= !(EscapeKeyMask::Alt as u8);
                    }
                    CapsLockSensitive::No(KeySymb::AltGr) => {
                        self.escape_key_mask &= !(EscapeKeyMask::Altgr as u8);
                    }
                    _ => {}
                }
                None
            }
        }
    }
}

#[no_mangle]
extern "C" fn keyboard_interrupt_handler(_interrupt_name: *const u8) {
    let scancode = unsafe { PS2_CONTROLER.read_scancode() };
    if let Some(scancode) = scancode {
        println!("key {:X?}", scancode);
        let keycode = KeyCode::from_scancode(scancode);
        if let Some(keycode) = keycode {
            let keysymb = unsafe { KEYBOARD_DRIVER.keycode_to_keymap(keycode) };
            println!("keycode {:X?}", keycode);
            if let Some(keysymb) = keysymb {
                println!("keysymb {:?}, = {:X?}", keysymb, keysymb as u32);
                let keysymb = keysymb as u32;
                if keysymb >= 0x20 && keysymb <= 0x7E {
                    println!("keysymb {:?}", keysymb as u8 as char);
                }
            }
        }
    }
    //println!("keyboard key code: {:X?}", scancode);
}
