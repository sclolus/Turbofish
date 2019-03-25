use crate::drivers::keyboard::keysymb::KeySymb;
use crate::drivers::keyboard::{CallbackKeyboard, KEYBOARD_DRIVER};

pub struct Terminal {
    buf: [KeySymb; 10],
    curr_offset: usize,
}

pub static mut TERMINAL: Option<Terminal> = None;

impl Terminal {
    pub fn new() -> Self {
        Self { buf: [KeySymb::nul; 10], curr_offset: 0 }
    }
    fn _display_char(key_symb: KeySymb) {
        match key_symb {
            KeySymb::Return => println!(""),
            _ => {
                if (key_symb >= KeySymb::space) && (key_symb <= KeySymb::asciitilde) {
                    print!("{}", key_symb as u32 as u8 as char);
                }
            }
        }
    }
    fn stock_keysymb(&mut self, key_symb: KeySymb) {
        if self.curr_offset >= self.buf.len() {
            return;
        }
        self.buf[self.curr_offset] = key_symb;
        self.curr_offset += 1;
    }
    pub fn read(&mut self, buf: &mut [KeySymb]) -> usize {
        // println!("read");
        let amt = core::cmp::min(buf.len(), self.curr_offset);
        let (a, _b) = self.buf.split_at(amt);

        if amt == 0 {
            return 0;
        }
        // First check if the amount of bytes we want to read is small:
        // `copy_from_slice` will generally expand to a call to `memcpy`, and
        // for a single byte the overhead is significant.
        if amt == 1 {
            buf[0] = a[0];
        } else {
            buf[..amt].copy_from_slice(a);
        }

        self.buf.copy_within(amt..self.curr_offset, 0);
        self.curr_offset = 0;
        amt
    }
}

pub fn stock_keysymb(keysymb: KeySymb) {
    unsafe {
        TERMINAL.as_mut().unwrap().stock_keysymb(keysymb);
    }
}

pub fn init_terminal() {
    unsafe {
        TERMINAL = Some(Terminal::new());
        KEYBOARD_DRIVER.as_mut().unwrap().bind(CallbackKeyboard::RequestKeySymb(stock_keysymb));
    }
}
