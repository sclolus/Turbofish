use super::ansi_escape_code::{AnsiColor, CursorMove, CSI};
use super::monitor::{AdvancedGraphic, Drawer, SCREEN_MONAD};
use super::{Cursor, Pos};
use alloc::collections::vec_deque::VecDeque;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::Write;

/// Description of a TTY buffer
#[derive(Debug, Clone)]
struct TerminalBuffer {
    buf: VecDeque<Vec<(u8, AnsiColor)>>,
    fixed_buf: Vec<Option<(u8, AnsiColor)>>,
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
            self.buf
                .push_back(vec![(0, AnsiColor::BLACK); self.nb_lines * self.nb_columns]);
        } else {
            let mut first = self.buf.pop_front().unwrap();
            // fresh the vec for reuse as last elem
            for c in &mut first {
                *c = (0, AnsiColor::BLACK);
            }
            self.buf.push_back(first);
            self.draw_start_pos -= self.nb_lines * self.nb_columns;
            self.write_pos -= self.nb_lines * self.nb_columns;
        }
    }

    /// Get a specified character from the buffer
    fn get_char(&self, i: usize) -> Option<(u8, AnsiColor)> {
        self.buf
            .get(i / (self.nb_lines * self.nb_columns))
            .map(|screen| screen[i % (self.nb_lines * self.nb_columns)])
    }

    /// Write a char into the buffer
    #[inline(always)]
    fn write_char(&mut self, c: char, color: AnsiColor) {
        match self
            .buf
            .get_mut(self.write_pos / (self.nb_lines * self.nb_columns))
        {
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

/// TTY handle many write mode
#[derive(Debug, Clone, Copy)]
pub enum WriteMode {
    /// Scroll may move printed characters
    Dynamic,
    /// The string is staticly fixed
    Fixed,
}

/// Base structure of a TTY
#[derive(Debug, Clone)]
pub struct Tty {
    /// TTY is it on foreground
    pub foreground: bool,
    /// Current TTY cursor
    pub cursor: Cursor,
    /// Current Text Color
    pub text_color: AnsiColor,
    /// current Write mode
    pub write_mode: WriteMode,
    buf: TerminalBuffer,
    scroll_offset: isize,
    background: Option<Vec<u8>>,
}

/// Handle different types of scroll
#[derive(Debug, Copy, Clone)]
pub enum Scroll {
    /// One line on top
    Up,
    /// One line on bottom
    Down,
    /// Half screen on top
    HalfScreenUp,
    /// Half screen on bottom
    HalfScreenDown,
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
            cursor: Cursor {
                pos: Default::default(),
                nb_lines,
                nb_columns,
                visible: false,
            },
            text_color: AnsiColor::WHITE,
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
                    let buf =
                        unsafe { slice::from_raw_parts_mut(buffer, width * height * bpp / 8) };

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
            HalfScreenUp => -((self.buf.nb_lines / 2 * self.buf.nb_columns) as isize),
            HalfScreenDown => (self.buf.nb_lines / 2 * self.buf.nb_columns) as isize,
        };
        self.scroll_offset =
            if (self.scroll_offset + add_scroll + self.buf.draw_start_pos as isize) < 0 {
                -(self.buf.draw_start_pos as isize)
            } else {
                self.scroll_offset + add_scroll
            };
        self.print_screen(self.scroll_offset);
    }

    /// Allow a shell for example to move cursor manually
    fn move_cursor(&mut self, direction: CursorMove) {
        if !self.cursor.visible || !self.foreground {
            return;
        }
        // Clear the Old cursor
        self.clear_cursor();
        SCREEN_MONAD
            .lock()
            .refresh_text_line(self.cursor.pos.line)
            .unwrap();

        // Apply new cursor direction
        match direction {
            CursorMove::Forward(q) => {
                self.buf.write_pos += q;
                for _i in 0..q {
                    self.cursor.forward();
                }
            }
            CursorMove::Backward(q) => {
                self.buf.write_pos -= q;
                for _i in 0..q {
                    self.cursor.backward();
                }
            }
            CursorMove::HorizontalAbsolute(q) => {
                self.buf.write_pos += q - self.cursor.pos.column;
                self.cursor.pos.column = q;
            }
            _ => {
                unimplemented!();
            }
        }

        // Draw the new cursor
        self.draw_cursor();
        SCREEN_MONAD
            .lock()
            .refresh_text_line(self.cursor.pos.line)
            .unwrap();
    }

    /// draw cursor for the character designed by write_pos in coordinate cursor.y and cursor.x
    fn draw_cursor(&self) {
        let c = match self.buf.get_char(self.buf.write_pos) {
            None | Some((0, _)) => (' ' as u8, self.text_color),
            Some(elem) => elem,
        };
        SCREEN_MONAD
            .lock()
            .draw_cursor(c.0 as char, self.cursor.pos, c.1)
            .unwrap();
    }

    /// draw cursor for the character designed by write_pos in coordinate cursor.y and cursor.x
    fn clear_cursor(&self) {
        let c = match self.buf.get_char(self.buf.write_pos) {
            None | Some((0, _)) => (' ' as u8, self.text_color),
            Some(elem) => elem,
        };
        SCREEN_MONAD
            .lock()
            .clear_cursor(c.0 as char, self.cursor.pos, c.1)
            .unwrap();
    }

    /// re-print the entire screen
    fn print_screen(&mut self, offset: isize) {
        SCREEN_MONAD.lock().clear_screen();
        self.cursor.pos = Default::default();

        let mut pos_last_char_writen = self.buf.write_pos;
        let start_pos = if offset > 0 {
            self.buf.draw_start_pos + offset as usize
        } else {
            self.buf
                .draw_start_pos
                .checked_sub((-offset) as usize)
                .unwrap_or(0)
        };
        for j in (start_pos..start_pos + self.cursor.nb_columns * self.cursor.nb_lines)
            .step_by(self.cursor.nb_columns)
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
                        pos_last_char_writen =
                            i + (self.cursor.nb_columns - (i % self.cursor.nb_columns));
                        break;
                    }
                    Some(other) => {
                        SCREEN_MONAD
                            .lock()
                            .draw_character(other.0 as char, self.cursor.pos, other.1)
                            .unwrap();
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
                        Pos {
                            line: i / self.cursor.nb_columns,
                            column: i % self.cursor.nb_columns,
                        },
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
        if self.cursor.visible {
            self.clear_cursor();
        }

        // Make the scroll coherency
        if self.foreground && self.scroll_offset != 0 {
            self.scroll_offset = 0;
            self.print_screen(self.scroll_offset);
        }

        use super::ansi_escape_code::*;
        use EscapedItem::*;
        for e in iter_escaped(s) {
            match e {
                Escaped(e) => match e {
                    EscapedCode::Color(color) => self.text_color = color.into(),
                    EscapedCode::CursorMove(cursor_move) => self.move_cursor(cursor_move),
                },
                Str(s) => {
                    for c in s.chars() {
                        match self.write_mode {
                            WriteMode::Dynamic => self.buf.write_char(c, self.text_color),
                            WriteMode::Fixed => {
                                self.buf.fixed_buf[self.cursor.pos.line * self.cursor.nb_columns
                                    + self.cursor.pos.column] = Some((c as u8, self.text_color))
                            }
                        };

                        if self.foreground {
                            if c != '\n' {
                                SCREEN_MONAD
                                    .lock()
                                    .draw_character(c, self.cursor.pos, self.text_color)
                                    .unwrap();
                                self.cursor.forward().map(|line| self.map_line(line));
                            } else {
                                self.cursor.cariage_return().map(|line| self.map_line(line));
                            }
                        }
                    }
                }
            }
        }
        if self.foreground && self.cursor.pos.column != 0 {
            SCREEN_MONAD
                .lock()
                .refresh_text_line(self.cursor.pos.line)
                .unwrap();
        }
        Ok(())
    }
}

/// Handle a TTY with a write buffer
#[derive(Debug, Clone)]
pub struct BufferedTty {
    /// TTY contained
    pub tty: Tty,
    /// contains unfinished escaped sequence, capacity max = 256
    escaped_buf: String,
}

impl AsRef<Tty> for BufferedTty {
    fn as_ref(&self) -> &Tty {
        &self.tty
    }
}

impl AsMut<Tty> for BufferedTty {
    fn as_mut(&mut self) -> &mut Tty {
        &mut self.tty
    }
}

const ESCAPED_BUF_CAPACITY: usize = 256;

impl BufferedTty {
    /// Create a new buffered TTY
    pub fn new(tty: Tty) -> Self {
        Self {
            tty,
            escaped_buf: String::with_capacity(ESCAPED_BUF_CAPACITY),
        }
    }
}

impl Write for BufferedTty {
    /// Fill its escaped buf when s has an unfinished escaped sequence
    /// to assure to call write_str on tty with complete escaped sequence.
    fn write_str(&mut self, mut s: &str) -> core::fmt::Result {
        debug_assert_eq!(self.escaped_buf.capacity(), ESCAPED_BUF_CAPACITY);
        loop {
            if self.escaped_buf.len() == 0 {
                match s.find(CSI) {
                    Some(i) => match s[i..].find(|c: char| c.is_ascii_alphabetic()) {
                        Some(mut j) => {
                            j += i;
                            self.tty.write_str(&s[..=j])?;
                            if j + 1 < s.len() {
                                s = &s[j + 1..];
                                continue;
                            }
                            break Ok(());
                        }
                        None => {
                            if s.len() - i < self.escaped_buf.capacity() {
                                self.escaped_buf.write_str(&s[i..]).unwrap();
                                break self.tty.write_str(&s[..i]);
                            } else {
                                // if we can't stock escaped sequence, write it on tty
                                break self.tty.write_str(s);
                            }
                        }
                    },
                    None => break self.tty.write_str(s),
                }
            } else {
                match s.find(|c: char| c.is_ascii_alphabetic()) {
                    Some(i) => {
                        if i + self.escaped_buf.len() < self.escaped_buf.capacity() {
                            self.escaped_buf.write_str(&s[..=i]).unwrap();
                            self.tty.write_str(&self.escaped_buf)?;
                        } else {
                            // if we can't stock escaped sequence, write it on tty, and truncate escape_buf
                            self.tty.write_str(&self.escaped_buf)?;
                            self.tty.write_str(&s[..=i])?;
                        }
                        self.escaped_buf.truncate(0);
                        if i + 1 < s.len() {
                            s = &s[i + 1..];
                            continue;
                        }
                        break Ok(());
                    }
                    None => {
                        if s.len() + self.escaped_buf.len() < self.escaped_buf.capacity() {
                            self.escaped_buf.write_str(s).unwrap();
                        } else {
                            // if we can't stock escaped sequence, write it on tty, and truncate escape_buf
                            self.tty.write_str(&self.escaped_buf)?;
                            self.tty.write_str(s)?;
                            self.escaped_buf.truncate(0);
                        }
                        break Ok(());
                    }
                }
            }
        }
    }
}

use arrayvec::{ArrayVec, CapacityError};
use bitflags::bitflags;
use core::cmp::min;
use core::convert::TryFrom;
use keyboard::keysymb::KeySymb;
use messaging::{MessageTo, SchedulerMessage};

bitflags! {
    pub struct Lmode: u32 {
        ///    Enable echo.
        const ECHO = 0;
        ///    Echo erase character as error-correcting backspace.
        const ECHOE = 1;
        ///    Echo KILL.
        const ECHOK = 2;
        ///    Echo NL.
        const ECHONL = 3;
        ///    Canonical input (erase and kill processing).
        const ICANON = 4;
        ///    Enable extended input character processing.
        const IEXTEN = 5;
        ///    Enable signals.
        const ISIG = 6;
        ///    Disable flush after interrupt or quit.
        const NOFLSH = 7;
        ///    Send SIGTTOU for background output.
        const TOSTOP = 8;
    }
}

pub fn encode_utf8(keysymb: KeySymb, dst: &mut [u8]) -> &[u8] {
    let c = char::try_from(keysymb as u32).unwrap();
    c.encode_utf8(dst).as_bytes()
}

#[derive(Debug, Clone)]
pub struct LineDiscipline {
    pub tty: BufferedTty,
    lmode: Lmode,
    read_buffer: ArrayVec<[u8; 4096]>,
}

impl LineDiscipline {
    pub fn new(tty: BufferedTty, lmode: Lmode) -> Self {
        Self {
            lmode,
            tty,
            read_buffer: ArrayVec::new(),
        }
    }

    /// handle directly some keysymb to control the terminal
    pub fn handle_tty_control(&mut self, keysymb: KeySymb) -> bool {
        match keysymb {
            KeySymb::Control_p => self.tty.as_mut().scroll(Scroll::Up),
            KeySymb::Control_n => self.tty.as_mut().scroll(Scroll::Down),
            KeySymb::Control_b => self.tty.as_mut().scroll(Scroll::HalfScreenUp),
            KeySymb::Control_d => self.tty.as_mut().scroll(Scroll::HalfScreenDown),
            _ => {
                return false;
            }
        };
        true
    }

    /// write in the read buffer the keysymb read from the keyboard
    /// Send a message if read is ready, depending of the lmode
    pub fn write_input(&mut self, buff: &[KeySymb]) -> Result<(), CapacityError<u8>> {
        let mut encode_buff = [0; 8];
        for key in buff {
            // bug enter trigger a Return but we want a Linefeed here
            let key = if *key == KeySymb::Return {
                KeySymb::Linefeed
            } else {
                *key
            };
            if !self.handle_tty_control(key) {
                // handle special keys in canonical mode
                if self.lmode.contains(Lmode::ICANON) {
                    // handle delete key
                    if key == KeySymb::Delete {
                        self.read_buffer.pop();
                        self.tty.as_mut().move_cursor(CursorMove::Backward(1));
                        self.tty.write_char('\0').unwrap();
                        self.tty.as_mut().move_cursor(CursorMove::Backward(1));
                        continue;
                    }
                    // handle kill key
                    if key == KeySymb::Control_u {
                        self.tty
                            .as_mut()
                            .move_cursor(CursorMove::HorizontalAbsolute(0));
                        if let Some(index) = self.read_buffer.iter().position(|c| *c == '\n' as u8)
                        {
                            self.read_buffer.truncate(index);
                        } else {
                            self.read_buffer.clear();
                        }

                        for _ in 0..self.tty.as_mut().cursor.nb_columns - 1 {
                            self.tty
                                .as_mut()
                                .write_char('\0')
                                .expect("failed to write \0");
                        }
                        self.tty
                            .as_mut()
                            .move_cursor(CursorMove::HorizontalAbsolute(0));

                        continue;
                    }
                }
                let b = encode_utf8(key, &mut encode_buff);
                for elem in b {
                    self.read_buffer.try_push(*elem)?;
                }
                if (self.lmode.contains(Lmode::ICANON) && key == KeySymb::Linefeed)
                    || !self.lmode.contains(Lmode::ICANON)
                {
                    messaging::push_message(MessageTo::Scheduler {
                        content: SchedulerMessage::SomethingToRead,
                    });
                }
                if self.lmode.contains(Lmode::ECHO) {
                    self.write(b);
                    self.tty.as_mut().move_cursor(CursorMove::Forward(0));
                    // self.tty.as_mut().draw_cursor();
                }
            }
        }
        Ok(())
    }
    /// read (from a process) on the tty
    /// return the number of bytes readen
    pub fn read(&mut self, output: &mut [u8]) -> usize {
        // print!("read buffer: ");
        // for c in &self.read_buffer {
        //     print!("{}", *c as char);
        // }
        // print!("\n");
        if self.lmode.contains(Lmode::ICANON) {
            if let Some(index) = self.read_buffer.iter().position(|c| *c == '\n' as u8) {
                //  In canonical mode, we only read until the '\n' and at most output.len bytes
                let len_data_to_read = min(index + 1, output.len());
                for (dest, src) in output
                    .iter_mut()
                    .zip(self.read_buffer.drain(0..len_data_to_read))
                {
                    *dest = src;
                }
                return len_data_to_read;
            }
        } else {
            let len_data_to_read = min(self.read_buffer.len(), output.len());
            for (dest, src) in output
                .iter_mut()
                .zip(self.read_buffer.drain(0..len_data_to_read))
            {
                *dest = src;
            }
            return len_data_to_read;
        }
        return 0;
    }

    /// write on the tty
    pub fn write(&mut self, s: &[u8]) -> usize {
        let s = core::str::from_utf8(s).expect("bad utf8");
        self.tty.write_str(s).expect("write failed");
        s.len()
    }
    pub fn get_tty(&self) -> &Tty {
        &self.tty.tty
    }
    pub fn get_tty_mut(&mut self) -> &mut Tty {
        &mut self.tty.tty
    }
}
