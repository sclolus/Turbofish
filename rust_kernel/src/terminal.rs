#[macro_use]
pub mod macros;

use crate::drivers::keyboard::keysymb::KeySymb;
use crate::drivers::keyboard::{CallbackKeyboard, KEYBOARD_DRIVER};
use crate::monitor::{AdvancedGraphic, Color, Drawer, IoResult, Pos, SCREEN_MONAD};
use alloc::collections::vec_deque::VecDeque;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::Write;

/// Description of a TTY buffer
#[derive(Debug, Clone)]
struct TerminalBuffer {
    buf: VecDeque<Vec<(u8, Color)>>,
    fixed_buf: Vec<Option<(u8, Color)>>,
    draw_start_pos: usize,
    write_pos: usize,
    nb_lines: usize,
    nb_columns: usize,
}

/// Here a TTY buffer
impl TerminalBuffer {
    pub fn new(nb_lines: usize, nb_columns: usize, buf_max_capacity: usize) -> Self {
        Self {
            buf: VecDeque::with_capacity(buf_max_capacity),
            fixed_buf: vec![None; nb_lines * nb_columns],
            write_pos: 0,
            nb_lines,
            nb_columns,
            draw_start_pos: 0,
        }
    }

    /// Make some place for a new screen
    pub fn make_place(&mut self) {
        if self.buf.len() < self.buf.capacity() {
            self.buf.push_back(vec![(0, Color::Black); self.nb_lines * self.nb_columns]);
        } else {
            let mut first = self.buf.pop_front().unwrap();
            // fresh the vec for reuse as last elem
            for c in &mut first {
                *c = (0, Color::Black);
            }
            self.buf.push_back(first);
            self.draw_start_pos -= self.nb_lines * self.nb_columns;
            self.write_pos -= self.nb_lines * self.nb_columns;
        }
    }

    /// Get a specified character from the buffer
    fn get_char(&self, i: usize) -> Option<(u8, Color)> {
        self.buf.get(i / (self.nb_lines * self.nb_columns)).map(|screen| screen[i % (self.nb_lines * self.nb_columns)])
    }

    /// Write a char into the buffer
    #[inline(always)]
    pub fn write_char(&mut self, c: char, color: Color) {
        match self.buf.get_mut(self.write_pos / (self.nb_lines * self.nb_columns)) {
            Some(screen) => {
                let pos = self.write_pos % (self.nb_lines * self.nb_columns);
                screen[pos] = (c as u8, color);
                self.write_pos += match c {
                    '\n' => self.nb_columns - pos % self.nb_columns,
                    _ => 1,
                };
                if self.write_pos > self.draw_start_pos + self.nb_columns * self.nb_lines {
                    self.draw_start_pos += self.nb_columns;
                }
            }
            None => {
                self.make_place();
                self.write_char(c, color)
            }
        }
    }
}

/// for the moment, we just handle left and right keys
#[derive(Debug, Copy, Clone)]
pub enum CursorDirection {
    Left,
    Right,
}

/// Simple and Basic implementation of cursor
#[derive(Debug, Copy, Clone, Default)]
pub struct Cursor {
    pos: Pos,
    nb_lines: usize,
    nb_columns: usize,
    visible: bool,
}

impl Cursor {
    /// Increment the cursor by one, return Option of line must be refreshed
    fn forward(&mut self) -> Option<usize> {
        self.pos.column += 1;
        if self.pos.column == self.nb_columns {
            self.cariage_return()
        } else {
            None
        }
    }
    /// Do a cariage_return, return Option of line must be refreshed
    fn cariage_return(&mut self) -> Option<usize> {
        let ret = Some(self.pos.line);

        self.pos.column = 0;
        if self.pos.line != self.nb_lines - 1 {
            self.pos.line += 1;
        }
        ret
    }
    /// Decrement the cursor by one
    fn backward(&mut self) -> Option<usize> {
        if self.pos.column == 0 {
            self.pos.column = self.nb_columns - 1;
            if self.pos.line != 0 {
                self.pos.line -= 1;
            }
        } else {
            self.pos.column -= 1;
        }
        None
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WriteMode {
    Dynamic,
    Fixed,
}

#[derive(Debug, Clone)]
pub struct Tty {
    echo: bool,
    buf: TerminalBuffer,
    scroll_offset: isize,
    pub cursor: Cursor,
    pub text_color: Color,
    pub write_mode: WriteMode,
}

#[derive(Debug, Copy, Clone)]
pub enum Scroll {
    Up,
    Down,
    HalfScreenDown,
    HalfScreenUp,
}

impl Tty {
    fn new(echo: bool, nb_lines: usize, nb_columns: usize, max_screen_buffer: usize) -> Self {
        Self {
            echo,
            buf: TerminalBuffer::new(nb_lines, nb_columns, max_screen_buffer),
            scroll_offset: 0,
            cursor: Cursor { pos: Default::default(), nb_lines, nb_columns, visible: false },
            text_color: Color::White,
            write_mode: WriteMode::Dynamic,
        }
    }

    fn refresh(&mut self) {
        self.print_screen(self.scroll_offset);
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
        self.print_screen(self.scroll_offset);
    }

    pub fn move_cursor(&mut self, direction: CursorDirection, q: usize) -> IoResult {
        if !self.cursor.visible {
            Ok(())
        } else {
            // Clear the Old cursor
            self.clear_cursor();
            SCREEN_MONAD.lock().refresh_text_line(self.cursor.pos.line).unwrap();

            // Apply new cursor direction
            match direction {
                CursorDirection::Right => {
                    self.buf.write_pos += q;
                    for _i in 0..q {
                        self.cursor.forward();
                    }
                }
                CursorDirection::Left => {
                    self.buf.write_pos -= q;
                    for _i in 0..q {
                        self.cursor.backward();
                    }
                }
            }

            // Draw the new cursor
            if self.echo {
                self.draw_cursor();
                SCREEN_MONAD.lock().refresh_text_line(self.cursor.pos.line).unwrap();
                Ok(())
            } else {
                Ok(())
            }
        }
    }

    pub fn set_text_color(&mut self, color: Color) {
        self.text_color = color;
    }

    /// draw cursor for the character designed by write_pos in coordinate cursor.y and cursor.x
    fn draw_cursor(&self) {
        let c = match self.buf.get_char(self.buf.write_pos) {
            None => (' ' as u8, self.text_color),
            Some(elem) => match elem {
                (0, _) => (' ' as u8, self.text_color),
                (_non_zero_char, _color) => elem,
            },
        };
        SCREEN_MONAD.lock().draw_cursor(c.0 as char, self.cursor.pos, c.1).unwrap();
    }

    /// draw cursor for the character designed by write_pos in coordinate cursor.y and cursor.x
    fn clear_cursor(&self) {
        let c = match self.buf.get_char(self.buf.write_pos) {
            None => (' ' as u8, self.text_color),
            Some(elem) => match elem {
                (0, _) => (' ' as u8, self.text_color),
                (_non_zero_char, _color) => elem,
            },
        };
        SCREEN_MONAD.lock().clear_cursor(c.0 as char, self.cursor.pos, c.1).unwrap();
    }

    /// re-print the entire screen
    pub fn print_screen(&mut self, offset: isize) {
        SCREEN_MONAD.lock().clear_screen();
        self.cursor.pos = Default::default();

        let mut _pos_last_char_writen = self.buf.write_pos;
        let start_pos = if offset > 0 {
            self.buf.draw_start_pos + offset as usize
        } else {
            self.buf.draw_start_pos.checked_sub((-offset) as usize).unwrap_or(0)
        };
        for j in (start_pos..start_pos + self.cursor.nb_columns * self.cursor.nb_lines).step_by(self.cursor.nb_columns)
        {
            for i in j..j + self.cursor.nb_columns {
                if i >= start_pos + self.cursor.nb_columns * self.cursor.nb_lines {
                    break;
                }
                match self.buf.get_char(i) {
                    None => {
                        break;
                    }
                    Some(n) if n.0 == 0 => {
                        break;
                    }
                    Some(n) if n.0 == '\n' as u8 => {
                        self.cursor.cariage_return();
                        _pos_last_char_writen = i + (self.cursor.nb_columns - (i % self.cursor.nb_columns));
                        break;
                    }
                    Some(other) => {
                        SCREEN_MONAD.lock().draw_character(other.0 as char, self.cursor.pos, other.1).unwrap();
                        self.cursor.forward();
                        _pos_last_char_writen = i + 1;
                    }
                }
            }
        }

        if self.cursor.visible && self.buf.write_pos <= _pos_last_char_writen {
            for _i in 0.._pos_last_char_writen - self.buf.write_pos {
                self.cursor.backward();
            }
            self.draw_cursor();
        }

        // Display all the fixed character buffer
        for (i, elem) in self.buf.fixed_buf.iter().enumerate() {
            match *elem {
                None => {}
                Some(e) => SCREEN_MONAD
                    .lock()
                    .draw_character(
                        e.0 as char,
                        Pos { line: i / self.cursor.nb_columns, column: i % self.cursor.nb_columns },
                        e.1,
                    )
                    .unwrap(),
            }
        }

        SCREEN_MONAD.lock().refresh_screen();
    }

    /// Refresh line or scroll
    fn map_line(&mut self, line: usize) {
        if line == self.cursor.pos.line {
            self.scroll(Scroll::Down);
            self.scroll_offset = 0;
        } else {
            SCREEN_MONAD.lock().refresh_text_line(line).unwrap();
        }
    }

    /// allow to modify globales parameters of a TTY, Usefull but very dangerous...
    pub fn modify(&mut self, mode: WriteMode, cursor_pos: Pos, color: Color) -> (WriteMode, Pos, Color) {
        let current_write_mode = self.write_mode;
        let current_cursor_pos = self.cursor.pos;
        let current_color = self.text_color;

        self.write_mode = mode;
        self.cursor.pos = cursor_pos;
        self.text_color = color;

        (current_write_mode, current_cursor_pos, current_color)
    }
}

impl core::fmt::Write for Tty {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        match self.write_mode {
            WriteMode::Dynamic => {
                if self.cursor.visible {
                    self.clear_cursor();
                }

                // Make the scroll coherency
                if self.scroll_offset != 0 {
                    self.scroll_offset = 0;
                    self.refresh();
                }

                if self.echo {
                    for c in s.chars() {
                        self.buf.write_char(c, self.text_color);
                        if c != '\n' {
                            SCREEN_MONAD.lock().draw_character(c, self.cursor.pos, self.text_color).unwrap();
                            self.cursor.forward().map(|line| self.map_line(line));
                        } else {
                            self.cursor.cariage_return().map(|line| self.map_line(line));
                        }
                    }
                    if self.cursor.pos.column != 0 {
                        SCREEN_MONAD.lock().refresh_text_line(self.cursor.pos.line).unwrap();
                    }
                    Ok(())
                } else {
                    for c in s.chars() {
                        self.buf.write_char(c, self.text_color);
                    }
                    Ok(())
                }
            }
            // Fixed character write
            WriteMode::Fixed => {
                for c in s.chars() {
                    self.buf.fixed_buf[self.cursor.pos.line * self.cursor.nb_columns + self.cursor.pos.column] =
                        Some((c as u8, self.text_color));
                    SCREEN_MONAD.lock().draw_character(c, self.cursor.pos, self.text_color).unwrap();
                    self.cursor.forward().map(|line| self.map_line(line));
                }
                if self.cursor.pos.column != 0 {
                    SCREEN_MONAD.lock().refresh_text_line(self.cursor.pos.line).unwrap();
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Terminal {
    buf: Option<KeySymb>,
    ttys: Vec<Tty>,
}

pub static mut TERMINAL: Option<Terminal> = None;

const MAX_SCREEN_BUFFER: usize = 10;

impl Terminal {
    pub fn new() -> Self {
        let screen_monad = SCREEN_MONAD.lock();
        Self {
            buf: None,
            ttys: vec![Tty::new(false, screen_monad.nb_lines, screen_monad.nb_columns, MAX_SCREEN_BUFFER); 2],
        }
    }

    fn switch_foreground_tty(&mut self, new_foreground_tty: usize) {
        self.ttys.iter_mut().find(|tty| tty.echo).map(|t| t.echo = false);
        self.ttys[new_foreground_tty].echo = true;
        self.ttys[new_foreground_tty].refresh();
    }

    pub fn get_foreground_tty(&mut self) -> Option<&mut Tty> {
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

    pub fn move_cursor(&mut self, direction: CursorDirection, q: usize) -> IoResult {
        self.get_foreground_tty().unwrap().move_cursor(direction, q)
    }

    pub fn set_foreground_fd(&mut self, fd: usize) {
        self.ttys[fd].echo = true;
    }

    pub fn get_tty(&mut self, fd: usize) -> &mut Tty {
        &mut self.ttys[fd]
    }
}

/// Usefull method to stock the character from the keyboard
pub fn stock_keysymb(keysymb: KeySymb) {
    unsafe {
        TERMINAL.as_mut().unwrap().stock_keysymb(keysymb);
    }
}

/// Extern function for initialisation
pub fn init_terminal() {
    unsafe {
        let mut term = Terminal::new();
        term.set_foreground_fd(1);
        term.get_tty(1).cursor.visible = true;
        TERMINAL = Some(term);
        KEYBOARD_DRIVER.as_mut().unwrap().bind(CallbackKeyboard::RequestKeySymb(stock_keysymb));
    }
}
