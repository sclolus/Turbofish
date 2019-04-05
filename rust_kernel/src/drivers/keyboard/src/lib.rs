//! See [PS/2 Keyboard](https://wiki.osdev.org/Keyboard)
#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

use io::{Io, Pio};

/// this module contains all the keySymbols for multiple layouts
pub mod keysymb;
use keysymb::KEYCODE_TO_KEYSYMB_AZERTY as KEYMAP_AZERTY;
use keysymb::KEYCODE_TO_KEYSYMB_QWERTY as KEYMAP_QWERTY;
use keysymb::{CapsLockSensitive, KeyMapArray, KeySymb};

#[allow(dead_code)]
struct Ps2Controler {
    data: Pio<u8>,
    command: Pio<u8>,
    current_scancode: Option<u32>,
}

static mut PS2_CONTROLER: Ps2Controler = Ps2Controler::new();

impl Ps2Controler {
    pub const fn new() -> Self {
        Ps2Controler { data: Pio::new(0x60), command: Pio::new(0x64), current_scancode: None }
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

/// keycode has two variants 'pressed' or 'released'
#[derive(Copy, Clone, Debug)]
pub enum KeyCode {
    /// It is very simple to understand what this variant means
    Pressed(u8),
    /// It is very simple to understand what this variant means
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
/// enum C of escape key sequence
enum EscapeKeyMask {
    Shift = 1,
    Altgr = 2,
    Control = 4,
    Alt = 8,
}

type ScanCode = u32;

/// callback consumer may get three types of keyboard data depending of their implementations
#[derive(Copy, Clone, Debug)]
pub enum CallbackKeyboard {
    /// consumer want a simple scancode
    RequestScanCode(fn(ScanCode)),
    /// consumer want a more complex thing, a keycode
    RequestKeyCode(fn(KeyCode)),
    /// consumer want a very complex keysymbol, ideal for a shell
    RequestKeySymb(fn(KeySymb)),
}

/// exported enum for user, allow to select a specific layout
#[derive(Copy, Clone, Debug)]
pub enum KeyMap {
    /// variant QWERTY, english (EN/US)
    En,
    /// variant AZERTY with extended ascii, french (FR)
    Fr,
}

/// main structure of the driver
pub struct KeyboardDriver {
    /// mask of control keys like ctrl, alt etc...
    escape_key_mask: u8,
    /// is capslock active
    capslock: bool,
    /// consumer callback registered
    io_term: Option<CallbackKeyboard>,
    /// keymap selected by consumer
    pub keymap: KeyMap,
}

/// this globale can be used by consumer
pub static mut KEYBOARD_DRIVER: Option<KeyboardDriver> = None;

impl KeyboardDriver {
    /// default initialisation
    pub fn new(f: Option<CallbackKeyboard>) -> Self {
        Self { escape_key_mask: 0, capslock: false, io_term: f, keymap: KeyMap::En }
    }

    /// bind a new callback
    pub fn bind(&mut self, f: CallbackKeyboard) {
        self.io_term = Some(f);
    }

    /// return the current keymap
    fn get_keymap(&self) -> &KeyMapArray {
        match self.keymap {
            KeyMap::En => &KEYMAP_QWERTY,
            KeyMap::Fr => &KEYMAP_AZERTY,
        }
    }

    /// convert a heycode to a keysymb
    pub fn keycode_to_keysymb(&mut self, keycode: KeyCode) -> Option<KeySymb> {
        match keycode {
            KeyCode::Pressed(k) => {
                let symb = self.get_keymap()[k as usize][(self.escape_key_mask) as usize];
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
                    CapsLockSensitive::No(other) => Some(other),
                    CapsLockSensitive::Yes(_) => {
                        let symb = self.get_keymap()[k as usize][(self.escape_key_mask ^ self.capslock as u8) as usize];
                        match symb {
                            CapsLockSensitive::No(other) | CapsLockSensitive::Yes(other) => Some(other),
                        }
                    }
                }
            }
            KeyCode::Released(k) => {
                let symb = self.get_keymap()[k as usize][(self.escape_key_mask ^ self.capslock as u8) as usize];
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

    /// call the registered callback with choosen options
    pub fn interrupt_handler(&mut self, scancode: u32) {
        self.io_term.map(|arg| {
            use CallbackKeyboard::*;
            match arg {
                RequestScanCode(u) => u(scancode),
                RequestKeyCode(u) => {
                    KeyCode::from_scancode(scancode).map(|s| u(s));
                }
                RequestKeySymb(u) => {
                    KeyCode::from_scancode(scancode).map(|s| self.keycode_to_keysymb(s).map(|s| u(s)));
                }
            }
        });
    }
}

/// extern initialisation function of the keyboard driver with default parameters
pub fn init_keyboard_driver() {
    unsafe {
        KEYBOARD_DRIVER = Some(KeyboardDriver::new(None));
    }
}

#[no_mangle]
extern "C" fn keyboard_interrupt_handler(_interrupt_name: *const u8) {
    let scancode = unsafe { PS2_CONTROLER.read_scancode() };
    if let Some(scancode) = scancode {
        unsafe { KEYBOARD_DRIVER.as_mut().unwrap().interrupt_handler(scancode) }
    }
}
