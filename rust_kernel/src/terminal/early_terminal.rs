//! This module is made for Basic VGA output, it dont require dynamic allocation and no unsafe trick

use crate::monitor::{Color, Drawer, Pos, SCREEN_MONAD};

use super::Cursor;

/// Classic height of default VGA screen
const HEIGHT: usize = 25;
/// Class width of default VGA screen
const WIDTH: usize = 80;

/// Main structure definition
#[derive(Copy, Clone)]
pub struct EarlyTerminal {
    cursor: Cursor,
    text_color: Color,
    buf: [Option<(u8, Color)>; WIDTH * HEIGHT],
}

/// Custom implementation of Debug trait
impl core::fmt::Debug for EarlyTerminal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?} {:?} and some boeufs ...", self.cursor, self.text_color)
    }
}

/*
/// Base cursor structure
#[derive(Debug, Copy, Clone)]
pub struct Cursor {
    y: usize,
    x: usize,
    lines: usize,
    columns: usize,
}
*/

/// Main globale
pub static mut EARLY_TERMINAL: EarlyTerminal = EarlyTerminal::new();

/// Early terminal is made just for VGA mode at the beginning of the program
impl EarlyTerminal {
    pub const fn new() -> Self {
        Self {
            cursor: Cursor { pos: Pos { line: 0, column: 0 }, nb_lines: HEIGHT, nb_columns: WIDTH, visible: true },
            text_color: Color::White,
            buf: [None; WIDTH * HEIGHT],
        }
    }

    /// Set a new text color
    pub fn set_text_color(&mut self, color: Color) {
        self.text_color = color;
    }

    /// Scroll screen
    fn scroll_vga_screen(&mut self) {
        let m = self.cursor.nb_columns * (self.cursor.nb_lines - 1);
        self.buf.copy_within(self.cursor.nb_columns.., 0);
        for elem in self.buf[m..].iter_mut() {
            *elem = None;
        }
        for (i, elem) in self.buf.iter().enumerate() {
            let (c, color) = match *elem {
                Some(e) => e,
                None => (' ' as u8, Color::White),
            };
            SCREEN_MONAD.lock().draw_character(c as char, Pos { line: i / WIDTH, column: i % WIDTH }, color).unwrap();
        }
    }
}

/// Common implementation of write
impl core::fmt::Write for EarlyTerminal {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.as_bytes() {
            match *c as char {
                '\n' => {
                    if let Some(line) = self.cursor.cariage_return() {
                        if line == self.cursor.nb_columns - 1 {
                            self.scroll_vga_screen();
                        }
                    }
                }
                _ => {
                    self.buf[self.cursor.pos.line * WIDTH + self.cursor.pos.column] = Some((*c as u8, self.text_color));
                    SCREEN_MONAD.lock().draw_character(*c as char, self.cursor.pos, self.text_color).unwrap();

                    if let Some(line) = self.cursor.forward() {
                        if line == self.cursor.nb_columns - 1 {
                            self.scroll_vga_screen();
                        }
                    }
                }
            };
        }
        Ok(())
    }
}
