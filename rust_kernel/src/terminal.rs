use crate::drivers::keyboard::keysymb::KeySymb;
use crate::drivers::keyboard::{CallbackKeyboard, KEYBOARD_DRIVER};
use crate::monitor::{CursorDirection, IoResult, SCREEN_MONAD};
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
            self.draw_start_pos -= self.nb_lines * self.nb_colons;
            self.write_pos -= self.nb_lines * self.nb_colons;
        }
    }

    fn get_char(&self, i: usize) -> Option<u8> {
        self.buf.get(i / (self.nb_lines * self.nb_colons)).map(|screen| screen[i % (self.nb_lines * self.nb_colons)])
    }

    pub fn print_screen(&self, offset: isize) {
        unsafe {
            SCREEN_MONAD.clear_screen();
            SCREEN_MONAD.set_cursor_position(0, 0);
            let start_pos = if offset > 0 {
                self.draw_start_pos + offset as usize
            } else {
                self.draw_start_pos.checked_sub((-offset) as usize).unwrap_or(0)
            };
            for j in (start_pos..start_pos + self.nb_colons * self.nb_lines).step_by(self.nb_colons) {
                for i in j..j + self.nb_colons {
                    if i >= self.write_pos {
                        break;
                    }
                    match self.get_char(i) {
                        None => {
                            break;
                        }
                        Some(n) if n == '\n' as u8 => {
                            print_screen!("{}", "\n");
                            break;
                        }
                        Some(other) => {
                            print_screen!("{}", other as char);
                        }
                    }
                }
            }
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
                if self.write_pos > self.draw_start_pos + self.nb_colons * self.nb_lines {
                    self.draw_start_pos += self.nb_colons;
                }
                // TODO: write if actif
            }
            None => {
                self.make_place();
                self.write_char(c)
            }
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for c in s.chars() {
            self.write_char(c);
        }
    }
}

#[derive(Clone)]
pub struct Tty {
    echo: bool,
    buf: TerminalBuffer,
    scroll_offset: isize,
}

pub enum Scroll {
    Up,
    Down,
    HalfScreenDown,
    HalfScreenUp,
}

impl Tty {
    fn new(echo: bool, buf: TerminalBuffer) -> Self {
        Self { echo, buf, scroll_offset: 0 }
    }

    fn refresh(&mut self) {
        eprintln!("refresh");
        self.buf.print_screen(self.scroll_offset)
    }

    fn scroll(&mut self, scroll: Scroll) {
        use Scroll::*;
        let add_scroll = match scroll {
            Up => -(self.buf.nb_colons as isize),
            Down => self.buf.nb_colons as isize,
            HalfScreenUp => -(((self.buf.nb_lines * self.buf.nb_colons) / 2) as isize),
            HalfScreenDown => ((self.buf.nb_lines * self.buf.nb_colons) / 2) as isize,
        };
        self.scroll_offset = if (self.scroll_offset + add_scroll + self.buf.draw_start_pos as isize) < 0 {
            -(self.buf.draw_start_pos as isize)
        } else {
            self.scroll_offset + add_scroll
        };
        self.buf.print_screen(self.scroll_offset)
    }

    pub fn move_cursor(&mut self, direction: CursorDirection, q: usize) -> IoResult {
        match direction {
            CursorDirection::Right => self.buf.write_pos += q,
            CursorDirection::Left => self.buf.write_pos -= q,
        }
        unsafe { SCREEN_MONAD.move_graphical_cursor(direction, q) }
    }
}

impl core::fmt::Write for Tty {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if self.scroll_offset != 0 {
            self.scroll_offset = 0;
            self.refresh();
        }
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

    fn get_foreground_tty(&mut self) -> Option<&mut Tty> {
        self.ttys.iter_mut().find(|tty| tty.echo)
    }

    fn handle_macros(&mut self, keysymb: KeySymb) -> Option<KeySymb> {
        match keysymb {
            KeySymb::o => {
                self.switch_foreground_tty(0);
                None
            }
            KeySymb::p => {
                self.switch_foreground_tty(1);
                None
            }
            KeySymb::Control_p => {
                self.get_foreground_tty().unwrap().scroll(Scroll::Up);
                None
            }
            KeySymb::Control_n => {
                self.get_foreground_tty().unwrap().scroll(Scroll::Down);
                None
            }
            KeySymb::Control_b => {
                self.get_foreground_tty().unwrap().scroll(Scroll::HalfScreenUp);
                None
            }
            KeySymb::Control_d => {
                self.get_foreground_tty().unwrap().scroll(Scroll::HalfScreenDown);
                None
            }
            other => Some(other),
        }
    }

    fn stock_keysymb(&mut self, key_symb: KeySymb) {
        eprintln!("{:?}", key_symb);
        if let Some(key) = self.handle_macros(key_symb) {
            if self.curr_offset >= self.buf.len() {
                return;
            }
            self.buf[self.curr_offset] = key;
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

    pub fn move_cursor(&mut self, direction: CursorDirection, q: usize) -> IoResult {
        self.get_foreground_tty().unwrap().move_cursor(direction, q)
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
