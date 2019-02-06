use crate::io::{Io, Pio};
pub mod keysymb;
use crate::keyboard::keysymb::KeySymb;
use crate::keyboard::keysymb::KEYCODE_TO_KEYSYMB_QWERTY as KEYMAP;

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
    pub fn from_scancode(scancode: u32) -> Option<Self> {
        if scancode >= 0x1 && scancode <= 0x53 {
            return Some(KeyCode::Pressed(scancode as u8));
        }
        if scancode >= 0x81 && scancode <= 0xd3 {
            return Some(KeyCode::Released((scancode - 0x80) as u8));
        }
        None
        /*
         0x50:   80  81  82  83  99   0  86  87
         0x58:   88 117   0   0  95 183 184 185
         0x60:    0   0   0   0   0   0   0   0
         0x68:    0   0   0   0   0   0   0   0
         0x70:   93   0   0  89   0   0  85  91
         0x78:   90  92   0  94   0 124 121   0

        e0 00:    0   0   0   0   0   0   0   0
        e0 08:    0   0   0   0   0   0   0   0
        e0 10:  165   0   0   0   0   0   0   0
        e0 18:    0 163   0   0  96  97   0   0
        e0 20:  113 140 164   0 166   0   0   0
        e0 28:    0   0 255   0   0   0 114   0
        e0 30:  115   0 150   0   0  98 255  99
        e0 38:  100   0   0   0   0   0   0   0
        e0 40:    0   0   0   0   0 119 119 102
        e0 48:  103 104   0 105 112 106 118 107
        e0 50:  108 109 110 111   0   0   0   0
        e0 58:    0   0   0 125 126 127 116 142
        e0 60:    0   0   0 143   0 217 156 173
        e0 68:  128 159 158 157 155 226   0 112
        e0 70:    0   0   0   0   0   0   0   0
        e0 78:    0   0   0   0   0   0   0   0
        */
    }
}
#[allow(dead_code)]
enum EscapeKeyMask {
    Shift = 1,
    Altgr = 2,
    Control = 4,
    Alt = 8,
    Shiftl = 16,
    Shiftr = 32,
    Ctrll = 64,
    Ctrlr = 128,
}

struct KeyboardDriver {
    escape_keys_lock: u8,
    capslock: bool,
}

static mut KEYBOARD_DRIVER: KeyboardDriver = KeyboardDriver::new();

impl KeyboardDriver {
    pub const fn new() -> Self {
        Self { escape_keys_lock: 0, capslock: false }
    }

    pub fn keycode_to_keymap(&mut self, keycode: KeyCode) -> Option<KeySymb> {
        match keycode {
            KeyCode::Pressed(k) => {
                let symb = &KEYMAP[k as usize][(self.escape_keys_lock ^ self.capslock as u8) as usize];
                match symb {
                    KeySymb::Control => {
                        self.escape_keys_lock |= EscapeKeyMask::Control as u8;
                        None
                    }
                    KeySymb::Shift => {
                        self.escape_keys_lock |= EscapeKeyMask::Shift as u8;
                        None
                    }
                    KeySymb::Alt => {
                        self.escape_keys_lock |= EscapeKeyMask::Alt as u8;
                        None
                    }
                    KeySymb::CtrlL_Lock => {
                        self.capslock = !self.capslock;
                        None
                    }
                    other => Some(*other),
                }
            }
            KeyCode::Released(k) => {
                let symb = &KEYMAP[k as usize][(self.escape_keys_lock ^ self.capslock as u8) as usize];
                match symb {
                    KeySymb::Control => {
                        self.escape_keys_lock &= !(EscapeKeyMask::Control as u8);
                    }
                    KeySymb::Shift => {
                        self.escape_keys_lock &= !(EscapeKeyMask::Shift as u8);
                    }
                    KeySymb::Alt => {
                        self.escape_keys_lock &= !(EscapeKeyMask::Alt as u8);
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
                println!("keysymb {:?}", keysymb);
                let keysymb = keysymb as u32;
                if keysymb > 0x20 && keysymb < 0x7E {
                    println!("keysymb {:?}", keysymb as u8 as char);
                }
            }
        }
    }
    //println!("keyboard key code: {:X?}", scancode);
}
