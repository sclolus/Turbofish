use crate::drivers::keyboard::keysymb::KeySymb;
use crate::drivers::keyboard::{CallbackKeyboard, KEYBOARD_DRIVER};
use crate::monitor::SCREEN_MONAD;
use alloc::collections::vec_deque::VecDeque;
use alloc::prelude::*;
use alloc::vec;
use core::fmt::Write;

#[derive(Clone)]
struct TerminalBuffer {
    buf: VecDeque<Vec<u8>>,
    draw_start_pos: usize,
    write_pos: usize,
    nb_lines: usize,
    nb_colons: usize,
}

impl TerminalBuffer {
    pub fn new(nb_lines: usize, nb_colons: usize, buf_max_capacity: usize) -> Self {
        Self { buf: VecDeque::with_capacity(buf_max_capacity), write_pos: 0, nb_lines, nb_colons, draw_start_pos: 0 }
    }
    pub fn make_place(&mut self) {
        if self.buf.len() < self.buf.capacity() {
            self.buf.push_back(vec![0; self.nb_lines * self.nb_colons]);
        } else {
            let mut first = self.buf.pop_front().unwrap();
            // fresh the vec for reuse as last elem
            for c in &mut first {
                *c = 0;
            }
            self.buf.push_back(first);
        }
    }
    pub fn print_current_screen(&self) {
        unsafe {
            SCREEN_MONAD.set_cursor_position(0, 0);
        }
    }
    pub fn write_char(&mut self, c: char) {
        match self.buf.get_mut(self.write_pos / (self.nb_lines * self.nb_colons)) {
            Some(screen) => {
                let pos = self.write_pos % (self.nb_lines * self.nb_colons);
                screen[pos] = c as u8;
                self.write_pos += match c {
                    '\n' => self.nb_colons - pos % self.nb_colons,
                    _ => 1,
                };

                // TODO: write if actif
            }
            None => {
                self.make_place();
                self.write_char(c)
            }
        }
    }
    pub fn write_str(&mut self, s: &str) {
        let cur = self.write_pos;
        for c in s.chars() {
            self.write_char(c);
        }
    }
}

#[derive(Clone)]
pub struct Tty {
    echo: bool,
    buf: TerminalBuffer,
}

impl Tty {
    fn new(echo: bool, buf: TerminalBuffer) -> Self {
        Self { echo, buf }
    }
    fn refresh(&mut self) {
        eprintln!("refresh");
        unsafe {
            SCREEN_MONAD.clear_screen();
            SCREEN_MONAD.write_str("switch").unwrap();
        }
    }
}

impl core::fmt::Write for Tty {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.buf.write_str(s);
        if self.echo {
            unsafe { SCREEN_MONAD.write_str(s) }
        } else {
            Ok(())
        }
    }
}
pub struct Terminal {
    buf: [KeySymb; 10],
    curr_offset: usize,
    ttys: Vec<Tty>,
}

pub static mut TERMINAL: Option<Terminal> = None;

const MAX_SCREEN_BUFFER: usize = 10;

impl Terminal {
    pub fn new() -> Self {
        unsafe {
            Self {
                buf: [KeySymb::nul; 10],
                curr_offset: 0,
                ttys: vec![
                    Tty::new(
                        false,
                        TerminalBuffer::new(SCREEN_MONAD.cursor.lines, SCREEN_MONAD.cursor.columns, MAX_SCREEN_BUFFER)
                    );
                    2
                ],
            }
        }
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
    fn switch_foreground_tty(&mut self, new_foreground_tty: usize) {
        self.ttys.iter_mut().find(|tty| tty.echo).map(|t| t.echo = false);
        self.ttys[new_foreground_tty].echo = true;
        eprintln!("switch");
        self.ttys[new_foreground_tty].refresh();
    }

    fn handle_macros(&mut self, keysymb: KeySymb) -> Option<KeySymb> {
        if keysymb == KeySymb::o {
            self.switch_foreground_tty(0);
            return None;
        } else if keysymb == KeySymb::p {
            self.switch_foreground_tty(1);
            return None;
        }
        return Some(keysymb);
    }
    fn stock_keysymb(&mut self, key_symb: KeySymb) {
        eprintln!("{:?}", key_symb);
        if let Some(key) = self.handle_macros(key_symb) {
            if self.curr_offset >= self.buf.len() {
                return;
            }
            self.buf[self.curr_offset] = key_symb;
            self.curr_offset += 1;
        }
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
    pub fn write_str(&mut self, fd: usize, s: &str) {
        self.ttys[fd].write_str(s).unwrap();
    }
    pub fn set_foreground_fd(&mut self, fd: usize) {
        self.ttys[fd].echo = true;
    }
    pub fn get_tty(&mut self, fd: usize) -> &mut Tty {
        eprintln!("get_tty");
        &mut self.ttys[fd]
    }
}

pub fn stock_keysymb(keysymb: KeySymb) {
    unsafe {
        TERMINAL.as_mut().unwrap().stock_keysymb(keysymb);
    }
}

pub fn init_terminal() {
    unsafe {
        let mut term = Terminal::new();
        term.set_foreground_fd(1);
        TERMINAL = Some(term);
        KEYBOARD_DRIVER.as_mut().unwrap().bind(CallbackKeyboard::RequestKeySymb(stock_keysymb));
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn big_write() {
//         let mut tty = Tty::new(false, TerminalBuffer::new(100, 100, 3));
//         tty.write_str(&"lala  ".repeat(100));
//     }
// }
