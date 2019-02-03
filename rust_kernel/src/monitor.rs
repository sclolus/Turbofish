mod vga_text_mode;
use vga_text_mode::*;
mod vbe_mode;
use vbe_mode::*;

pub mod bmp_loader;

pub type IoResult = core::result::Result<(), IoError>;

#[derive(Debug, Copy, Clone)]
pub enum IoError {
    ColorNotSupported,
    CursorOutOfBound,
    GraphicModeNotFounded,
    NotSupported,
}

trait Drawer {
    fn draw_character(&self, c: char, y: usize, x: usize);
    fn scroll_screen(&self);
    fn clear_screen(&mut self);
    fn set_text_color(&mut self, color: ColorName) -> IoResult;
}

trait CursorControler {
    fn set_cursor_position(&mut self, x: usize, y: usize) -> IoResult;
    fn is_cursor_moved(&mut self, x_origin: usize) -> usize;
    fn cursor_forward(&mut self, x_origin: usize) -> usize;
    fn cursor_cariage_return(&mut self, x_origin: usize) -> usize;
}

trait AdvancedGraphic {
    fn refresh_text_line(&mut self, x1: usize, x2: usize, y: usize);
    fn draw_graphic_buffer(&mut self, f: fn(*mut u8, usize, usize, usize) -> IoResult) -> IoResult;
}

#[derive(Debug, Copy, Clone)]
pub enum ColorName {
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

/// public
impl TextMonad {
    /// default VGA_TEXT_MODE
    const fn new() -> Self {
        let vga = VgaTextMode::new();
        let (lines, columns) = vga.query_window_size();
        Self { drawing_mode: DrawingMode::Vga(vga), cursor: Cursor { x: 0, y: 0, columns, lines } }
    }
    /// Switch between VBE mode
    pub fn switch_graphic_mode(&mut self, mode: Option<u16>) -> Result<(), VbeError> {
        let vbe = init_graphic_mode(mode)?;
        self.drawing_mode = DrawingMode::Vbe(vbe);
        let (lines, columns) = vbe.query_window_size();
        self.cursor = Cursor { x: 0, y: 0, columns, lines };
        Ok(())
    }
    /// basic, simple
    pub fn set_text_color(&mut self, color: ColorName) -> IoResult {
        Drawer::set_text_color(self, color)
    }
    /// void the screen
    pub fn clear_screen(&mut self) {
        Drawer::clear_screen(self);
    }
    /// set manualy position of cursor
    pub fn set_cursor_position(&mut self, x: usize, y: usize) -> IoResult {
        CursorControler::set_cursor_position(self, x, y)
    }
    /// fill the graphic buffer with a custom fn
    pub fn draw_graphic_buffer(&mut self, f: fn(*mut u8, usize, usize, usize) -> IoResult) -> IoResult {
        AdvancedGraphic::draw_graphic_buffer(self, f)
    }
}

/// private
impl Drawer for TextMonad {
    /// put a character into the screen
    fn draw_character(&self, c: char, y: usize, x: usize) {
        match &self.drawing_mode {
            DrawingMode::Vga(vga) => vga.draw_character(c, y, x),
            DrawingMode::Vbe(vbe) => vbe.draw_character(c, y, x),
        }
    }
    /// just scroll a bit
    fn scroll_screen(&self) {
        match &self.drawing_mode {
            DrawingMode::Vga(vga) => vga.scroll_screen(),
            DrawingMode::Vbe(vbe) => vbe.scroll_screen(),
        }
    }
    /// void the screen
    fn clear_screen(&mut self) {
        match &mut self.drawing_mode {
            DrawingMode::Vga(vga) => vga.clear_screen(),
            DrawingMode::Vbe(vbe) => vbe.clear_screen(),
        }
        self.set_cursor_position(0, 0).unwrap();
    }
    /// basic, simple
    fn set_text_color(&mut self, color: ColorName) -> IoResult {
        match &mut self.drawing_mode {
            DrawingMode::Vga(vga) => vga.set_text_color(color),
            DrawingMode::Vbe(vbe) => vbe.set_text_color(color),
        }
    }
}

/// private
impl CursorControler for TextMonad {
    /// set manualy position of cursor
    fn set_cursor_position(&mut self, x: usize, y: usize) -> IoResult {
        if x >= self.cursor.columns || y >= self.cursor.lines {
            Err(IoError::CursorOutOfBound)
        } else {
            self.cursor.x = x;
            self.cursor.y = y;
            Ok(())
        }
    }
    /// check if cursor has moved
    fn is_cursor_moved(&mut self, x_origin: usize) -> usize {
        if self.cursor.x != x_origin {
            self.refresh_text_line(x_origin, self.cursor.x, self.cursor.y);
        }
        self.cursor.x
    }
    /// advance cursor by 1
    fn cursor_forward(&mut self, x_origin: usize) -> usize {
        if self.cursor.x + 1 == self.cursor.columns {
            self.cursor_cariage_return(x_origin)
        } else {
            self.cursor.x += 1;
            x_origin
        }
    }
    /// new line
    fn cursor_cariage_return(&mut self, x_origin: usize) -> usize {
        if self.cursor.y + 1 == self.cursor.lines {
            self.scroll_screen();
        } else {
            self.refresh_text_line(x_origin, self.cursor.x, self.cursor.y);
            self.cursor.y += 1;
        }
        self.cursor.x = 0;
        0
    }
}

/// private
impl AdvancedGraphic for TextMonad {
    /// command a refresh for selected graphic area
    fn refresh_text_line(&mut self, x1: usize, x2: usize, y: usize) {
        match &mut self.drawing_mode {
            DrawingMode::Vga(_vga) => (),
            DrawingMode::Vbe(vbe) => vbe.refresh_text_line(x1, x2, y),
        }
    }
    /// fill the graphic buffer with a custom function
    fn draw_graphic_buffer(&mut self, f: fn(*mut u8, usize, usize, usize) -> IoResult) -> IoResult {
        match &mut self.drawing_mode {
            DrawingMode::Vga(_vga) => Err(IoError::GraphicModeNotFounded),
            DrawingMode::Vbe(vbe) => vbe.draw_graphic_buffer(f),
        }
    }
}

/// common Write implementation
impl core::fmt::Write for TextMonad {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut x_origin: usize = self.cursor.x;
        for c in s.as_bytes() {
            match *c as char {
                '\n' => x_origin = self.cursor_cariage_return(x_origin),
                _ => {
                    self.draw_character(*c as char, self.cursor.y, self.cursor.x);
                    x_origin = self.cursor_forward(x_origin);
                }
            }
        }
        self.is_cursor_moved(x_origin);
        Ok(())
    }
}
