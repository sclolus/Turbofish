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
    fn refresh_text_line(&mut self, x1: usize, x2: usize, y: usize);
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
    Black,
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
    fn refresh_text_line(&mut self, x1: usize, x2: usize, y: usize) {
        match &mut self.drawing_mode {
            DrawingMode::Vga(vga) => vga.refresh_text_line(x1, x2, y),
            DrawingMode::Vbe(vbe) => vbe.refresh_text_line(x1, x2, y),
        }
    }
}

impl core::fmt::Write for TextMonad {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut x_origin: usize = self.cursor.x;
        for c in s.as_bytes() {
            match *c as char {
                '\n' => {
                    if self.cursor.y + 1 == self.cursor.lines {
                        self.scroll_screen();
                    } else {
                        if self.cursor.x != x_origin {
                            self.refresh_text_line(x_origin, self.cursor.x, self.cursor.y);
                        }
                        self.cursor.y += 1;
                    }
                    self.cursor.x = 0;
                    x_origin = 0;
                }
                _ => {
                    self.draw_character(*c as char, self.cursor.y, self.cursor.x);

                    if self.cursor.x + 1 == self.cursor.columns {
                        if self.cursor.y + 1 == self.cursor.lines {
                            self.scroll_screen();
                        } else {
                            self.refresh_text_line(x_origin, self.cursor.columns, self.cursor.y);
                            self.cursor.y += 1;
                        }
                        self.cursor.x = 0;
                        x_origin = 0;
                    } else {
                        self.cursor.x += 1;
                    }
                }
            }
        }
        if self.cursor.x != x_origin {
            self.refresh_text_line(x_origin, self.cursor.x, self.cursor.y);
        }
        Ok(())
    }
}
