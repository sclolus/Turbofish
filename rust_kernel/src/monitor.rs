mod vga_text_mode;
use vga_text_mode::*;
mod vbe_mode;
use vbe_mode::*;

pub type IoResult = core::result::Result<(), IoError>;

#[derive(Debug, Copy, Clone)]
pub enum IoError {
    ColorNotSupported,
    CursorOutOfBound,
}

pub trait Drawer {
    fn draw_character(&self, c: char, y: usize, x: usize);
    fn scroll_screen(&self);
    fn clear_screen(&mut self);
    fn set_text_color(&mut self, color: TextColor) -> IoResult;
}

#[derive(Debug, Copy, Clone)]
pub enum TextColor {
    Red,
    Green,
    Yellow,
    Cyan,
    Brown,
    Magenta,
    Blue,
    White,
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => ({
        unsafe {
            core::fmt::write(&mut $crate::monitor::TEXT_MONAD, format_args!($($arg)*)).unwrap();
            core::fmt::write(&mut $crate::monitor::TEXT_MONAD, format_args!("\n")).unwrap();
        }
    })
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        unsafe {
            core::fmt::write(&mut $crate::monitor::TEXT_MONAD, format_args!($($arg)*)).unwrap();
        }
    })
}

/// x,y,lines,columns are in unit of char
#[derive(Debug, Copy, Clone)]
struct Cursor {
    pub x: usize,
    pub y: usize,
    pub columns: usize,
    pub lines: usize,
}

impl Cursor {
    pub fn set_position(&mut self, x: usize, y: usize) -> IoResult {
        if x >= self.columns || y >= self.lines {
            Err(IoError::CursorOutOfBound)
        } else {
            self.x = x;
            self.y = y;
            Ok(())
        }
    }
    /// advance cursor by 1, return false if need to scroll
    pub fn forward(&mut self) -> bool {
        self.x = self.x + 1;
        if self.x == self.columns {
            self.cariage_return()
        } else {
            true
        }
    }
    /// return false if need to scroll
    pub fn cariage_return(&mut self) -> bool {
        self.x = 0;
        if self.y + 1 == self.lines {
            false
        } else {
            self.y += 1;
            true
        }
    }
}

enum DrawingMode {
    Vga(VgaTextMode),
    Vbe(VbeMode),
}

/// Control the cursor and can put text on screen thanks to its drawer slave
pub struct TextMonad {
    drawing_mode: DrawingMode,
    cursor: Cursor,
}

pub static mut TEXT_MONAD: TextMonad = TextMonad::new();

impl TextMonad {
    const fn new() -> Self {
        let vga = VgaTextMode::new();
        let (lines, columns) = vga.query_window_size();
        Self { drawing_mode: DrawingMode::Vga(vga), cursor: Cursor { x: 0, y: 0, columns, lines } }
    }
    pub fn switch_graphic_mode(&mut self, mode: Option<u16>) -> Result<(), VbeError> {
        let vbe = init_graphic_mode(mode)?;
        self.drawing_mode = DrawingMode::Vbe(vbe);
        let (lines, columns) = vbe.query_window_size();
        self.cursor = Cursor { x: 0, y: 0, columns, lines };
        Ok(())
    }
}

impl Drawer for TextMonad {
    fn draw_character(&self, c: char, y: usize, x: usize) {
        match &self.drawing_mode {
            DrawingMode::Vga(vga) => vga.draw_character(c, y, x),
            DrawingMode::Vbe(vbe) => vbe.draw_character(c, y, x),
        }
    }
    fn scroll_screen(&self) {
        match &self.drawing_mode {
            DrawingMode::Vga(vga) => vga.scroll_screen(),
            DrawingMode::Vbe(vbe) => vbe.scroll_screen(),
        }
    }
    fn clear_screen(&mut self) {
        match &mut self.drawing_mode {
            DrawingMode::Vga(vga) => vga.clear_screen(),
            DrawingMode::Vbe(vbe) => vbe.clear_screen(),
        }
        self.cursor.set_position(0, 0).unwrap();
    }
    fn set_text_color(&mut self, color: TextColor) -> IoResult {
        match &mut self.drawing_mode {
            DrawingMode::Vga(vga) => vga.set_text_color(color),
            DrawingMode::Vbe(vbe) => vbe.set_text_color(color),
        }
    }
}

impl core::fmt::Write for TextMonad {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.as_bytes() {
            match *c as char {
                '\n' => {
                    if !self.cursor.cariage_return() {
                        self.scroll_screen();
                    }
                }
                _ => {
                    self.draw_character(*c as char, self.cursor.y, self.cursor.x);
                    if !self.cursor.forward() {
                        self.scroll_screen();
                    }
                }
            }
        }
        Ok(())
    }
}
