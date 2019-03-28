use crate::drivers::keyboard::keysymb::KeySymb;
use crate::drivers::keyboard::{CallbackKeyboard, KEYBOARD_DRIVER};
//use crate::monitor::{IoResult, SCREEN_MONAD};
use crate::monitor::SCREEN_MONAD;
use alloc::collections::vec_deque::VecDeque;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::Write;

#[derive(Clone)]
struct TerminalBuffer {
    buf: VecDeque<Vec<u8>>,
    draw_start_pos: usize,
    write_pos: usize,
    nb_lines: usize,
    nb_columns: usize,
}

impl TerminalBuffer {
    pub fn new(nb_lines: usize, nb_columns: usize, buf_max_capacity: usize) -> Self {
        Self { buf: VecDeque::with_capacity(buf_max_capacity), write_pos: 0, nb_lines, nb_columns, draw_start_pos: 0 }
    }

    pub fn make_place(&mut self) {
        if self.buf.len() < self.buf.capacity() {
            self.buf.push_back(vec![0; self.nb_lines * self.nb_columns]);
        } else {
            let mut first = self.buf.pop_front().unwrap();
            // fresh the vec for reuse as last elem
            for c in &mut first {
                *c = 0;
            }
            self.buf.push_back(first);
            self.draw_start_pos -= self.nb_lines * self.nb_columns;
            self.write_pos -= self.nb_lines * self.nb_columns;
        }
    }

    fn get_char(&self, i: usize) -> Option<u8> {
        self.buf.get(i / (self.nb_lines * self.nb_columns)).map(|screen| screen[i % (self.nb_lines * self.nb_columns)])
    }

    pub fn print_screen(&self, offset: isize) {
        //        SCREEN_MONAD.lock().clear_screen(Buffer::CHARACTERS_BUFFER);
        //        SCREEN_MONAD.lock().set_cursor_position(0, 0).unwrap();
        let mut _pos_last_char_writen = self.write_pos;
        let start_pos = if offset > 0 {
            self.draw_start_pos + offset as usize
        } else {
            self.draw_start_pos.checked_sub((-offset) as usize).unwrap_or(0)
        };
        for j in (start_pos..start_pos + self.nb_columns * self.nb_lines).step_by(self.nb_columns) {
            for i in j..j + self.nb_columns {
                if i >= start_pos + self.nb_columns * self.nb_lines {
                    break;
                }
                match self.get_char(i) {
                    None => {
                        break;
                    }
                    Some(n) if n == 0 => {
                        break;
                    }
                    Some(n) if n == '\n' as u8 => {
                        print_screen!("{}", "\n");
                        _pos_last_char_writen = i + (self.nb_columns - (i % self.nb_columns));
                        break;
                    }
                    Some(other) => {
                        print_screen!("{}", other as char);
                        _pos_last_char_writen = i + 1;
                    }
                }
            }
        }
        //        eprintln!("{}", (_pos_last_char_writen as isize));
        //        eprintln!("{}", (self.write_pos as isize));
        //        eprintln!("{}", (_pos_last_char_writen as isize as isize - self.write_pos as isize) as isize);
        // let res =
        //     SCREEN_MONAD.lock().move_graphical_cursor(CursorDirection::Left, _pos_last_char_writen - self.write_pos);
        // if offset == 0 {
        //     res.unwrap();
        // }
    }

    pub fn write_char(&mut self, c: char) {
        match self.buf.get_mut(self.write_pos / (self.nb_lines * self.nb_columns)) {
            Some(screen) => {
                let pos = self.write_pos % (self.nb_lines * self.nb_columns);
                screen[pos] = c as u8;
                self.write_pos += match c {
                    '\n' => self.nb_columns - pos % self.nb_columns,
                    _ => 1,
                };
                if self.write_pos > self.draw_start_pos + self.nb_columns * self.nb_lines {
                    self.draw_start_pos += self.nb_columns;
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
        self.buf.print_screen(self.scroll_offset);
        // use crate::monitor::Drawer;
        // SCREEN_MONAD.lock().draw_cursor();
    }

    fn scroll(&mut self, scroll: Scroll) {
        use Scroll::*;
        let add_scroll = match scroll {
            Up => -(self.buf.nb_columns as isize),
            Down => self.buf.nb_columns as isize,
            HalfScreenUp => -(((self.buf.nb_lines * self.buf.nb_columns) / 2) as isize),
            HalfScreenDown => ((self.buf.nb_lines * self.buf.nb_columns) / 2) as isize,
        };
        self.scroll_offset = if (self.scroll_offset + add_scroll + self.buf.draw_start_pos as isize) < 0 {
            -(self.buf.draw_start_pos as isize)
        } else {
            self.scroll_offset + add_scroll
        };
        self.buf.print_screen(self.scroll_offset)
    }

    /*
    pub fn move_cursor(&mut self, direction: CursorDirection, q: usize) -> IoResult {
        match direction {
            CursorDirection::Right => self.buf.write_pos += q,
            CursorDirection::Left => self.buf.write_pos -= q,
        }
        if self.echo {
            SCREEN_MONAD.lock().move_graphical_cursor(direction, q)
        } else {
            Ok(())
        }
    }
    */
}

impl core::fmt::Write for Tty {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if self.scroll_offset != 0 {
            self.scroll_offset = 0;
            self.refresh();
        }
        self.buf.write_str(s);
        if self.echo {
            Ok(())
        // SCREEN_MONAD.lock().write_str(s)
        } else {
            Ok(())
        }
    }
}

pub struct Terminal {
    buf: Option<KeySymb>,
    ttys: Vec<Tty>,
}

pub static mut TERMINAL: Option<Terminal> = None;

const MAX_SCREEN_BUFFER: usize = 10;

impl Terminal {
    pub fn new() -> Self {
        let _screen_monad = SCREEN_MONAD.lock();
        Self {
            buf: None,
            ttys: vec![
                Tty::new(
                    false,
                    TerminalBuffer::new(
                        0,
                        0, /*screen_monad.cursor.lines, screen_monad.cursor.columns*/
                        MAX_SCREEN_BUFFER
                    )
                );
                2
            ],
        }
    }

    fn switch_foreground_tty(&mut self, new_foreground_tty: usize) {
        self.ttys.iter_mut().find(|tty| tty.echo).map(|t| t.echo = false);
        self.ttys[new_foreground_tty].echo = true;
        self.ttys[new_foreground_tty].refresh();
    }

    fn get_foreground_tty(&mut self) -> Option<&mut Tty> {
        self.ttys.iter_mut().find(|tty| tty.echo)
    }

    fn handle_macros(&mut self) {
        match self.buf {
            Some(KeySymb::F1) => self.switch_foreground_tty(0),
            Some(KeySymb::F2) => self.switch_foreground_tty(1),
            Some(KeySymb::Control_p) => self.get_foreground_tty().unwrap().scroll(Scroll::Up),
            Some(KeySymb::Control_n) => self.get_foreground_tty().unwrap().scroll(Scroll::Down),
            Some(KeySymb::Control_b) => self.get_foreground_tty().unwrap().scroll(Scroll::HalfScreenUp),
            Some(KeySymb::Control_d) => self.get_foreground_tty().unwrap().scroll(Scroll::HalfScreenDown),
            _ => {
                return;
            }
        };
        self.buf = None;
    }

    fn stock_keysymb(&mut self, keysymb: KeySymb) {
        if self.buf.is_none() {
            self.buf = Some(keysymb);
        }
    }

    pub fn read(&mut self, buf: &mut [KeySymb]) -> usize {
        self.handle_macros();
        if let Some(key) = self.buf {
            buf[0] = key;
            self.buf = None;
            return 1;
        }
        return 0;
    }

    pub fn write_str(&mut self, fd: usize, s: &str) {
        self.ttys[fd].write_str(s).unwrap();
    }

    // pub fn move_cursor(&mut self, direction: CursorDirection, q: usize) -> IoResult {
    //     self.get_foreground_tty().unwrap().move_cursor(direction, q)
    // }

    pub fn set_foreground_fd(&mut self, fd: usize) {
        self.ttys[fd].echo = true;
    }

    pub fn get_tty(&mut self, fd: usize) -> &mut Tty {
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
