use crate::terminal::monitor::{AdvancedGraphic, Drawer, SCREEN_MONAD};
use crate::terminal::{Color, Cursor, Pos};
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
    fn new(nb_lines: usize, nb_columns: usize, buf_max_capacity: usize) -> Self {
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
    fn make_place(&mut self) {
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
    fn write_char(&mut self, c: char, color: Color) {
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

#[derive(Debug, Clone, Copy)]
pub enum WriteMode {
    Dynamic,
    Fixed,
}

#[derive(Debug, Clone)]
pub struct Tty {
    pub foreground: bool,
    pub cursor: Cursor,
    pub text_color: Color,
    pub write_mode: WriteMode,
    buf: TerminalBuffer,
    scroll_offset: isize,
    background: Option<Vec<u8>>,
}

#[derive(Debug, Copy, Clone)]
pub enum Scroll {
    Up,
    Down,
    HalfScreenDown,
    HalfScreenUp,
}

/// Implementation a a unique TTY
impl Tty {
    /// Constructor
    pub fn new(
        foreground: bool,
        nb_lines: usize,
        nb_columns: usize,
        max_screen_buffer: usize,
        background: Option<Vec<u8>>,
    ) -> Self {
        Self {
            foreground,
            buf: TerminalBuffer::new(nb_lines, nb_columns, max_screen_buffer),
            scroll_offset: 0,
            cursor: Cursor { pos: Default::default(), nb_lines, nb_columns, visible: false },
            text_color: Color::White,
            write_mode: WriteMode::Dynamic,
            background,
        }
    }

    /// Display the tty, mut be called when new activation of tty or switch
    pub fn refresh(&mut self) {
        SCREEN_MONAD
            .lock()
            .draw_graphic_buffer(|buffer: *mut u8, width: usize, height: usize, bpp: usize| {
                if let Some(background) = &self.background {
                    use core::slice;
                    let buf = unsafe { slice::from_raw_parts_mut(buffer, width * height * bpp / 8) };

                    for (i, elem) in buf.iter_mut().enumerate() {
                        *elem = background[i];
                    }
                }
                Ok(())
            })
            .unwrap();
        self.print_screen(self.scroll_offset);
    }

    /// set the background buffer
    pub fn set_background_buffer(&mut self, v: Vec<u8>) {
        self.background = Some(v);
    }

    /// Internal scroll
    pub fn scroll(&mut self, scroll: Scroll) {
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

    /// Allow a shell for example to move cursor manually
    pub fn move_cursor(&mut self, direction: CursorDirection, q: usize) {
        if !self.cursor.visible || !self.foreground {
            return;
        }
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
        self.draw_cursor();
        SCREEN_MONAD.lock().refresh_text_line(self.cursor.pos.line).unwrap();
    }

    /// Simple and basic
    pub fn set_text_color(&mut self, color: Color) {
        self.text_color = color;
    }

    /// draw cursor for the character designed by write_pos in coordinate cursor.y and cursor.x
    pub fn draw_cursor(&self) {
        let c = match self.buf.get_char(self.buf.write_pos) {
            None | Some((0, _)) => (' ' as u8, self.text_color),
            Some(elem) => elem,
        };
        SCREEN_MONAD.lock().draw_cursor(c.0 as char, self.cursor.pos, c.1).unwrap();
    }

    /// draw cursor for the character designed by write_pos in coordinate cursor.y and cursor.x
    pub fn clear_cursor(&self) {
        let c = match self.buf.get_char(self.buf.write_pos) {
            None | Some((0, _)) => (' ' as u8, self.text_color),
            Some(elem) => elem,
        };
        SCREEN_MONAD.lock().clear_cursor(c.0 as char, self.cursor.pos, c.1).unwrap();
    }

    /// re-print the entire screen
    pub fn print_screen(&mut self, offset: isize) {
        SCREEN_MONAD.lock().clear_screen();
        self.cursor.pos = Default::default();

        let mut pos_last_char_writen = self.buf.write_pos;
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
                        pos_last_char_writen = i + (self.cursor.nb_columns - (i % self.cursor.nb_columns));
                        break;
                    }
                    Some(other) => {
                        SCREEN_MONAD.lock().draw_character(other.0 as char, self.cursor.pos, other.1).unwrap();
                        self.cursor.forward();
                        pos_last_char_writen = i + 1;
                    }
                }
            }
        }

        if self.cursor.visible && self.buf.write_pos <= pos_last_char_writen {
            for _i in 0..pos_last_char_writen - self.buf.write_pos {
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
}

/// TTY implement some methods of writing
/// Dynamic: Classic behavior with buffering and scroll
/// Fixed: The text is always printed on screen
impl Write for Tty {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        match self.write_mode {
            WriteMode::Dynamic => {
                if self.cursor.visible {
                    self.clear_cursor();
                }

                // Make the scroll coherency
                if self.scroll_offset != 0 {
                    self.scroll_offset = 0;
                    self.print_screen(self.scroll_offset);
                }

                if self.foreground {
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
