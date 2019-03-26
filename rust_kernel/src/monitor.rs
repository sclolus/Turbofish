#[macro_use]
pub mod macros;
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

#[derive(Debug, Copy, Clone)]
pub enum WriteMode {
    Dynamic,
    Fixed,
}

pub trait Drawer {
    fn draw_character(&mut self, c: char, y: usize, x: usize);
    fn scroll_screen(&mut self);
    fn clear_screen(&mut self);
    fn set_text_color(&mut self, color: Color) -> IoResult;
    fn clear_cursor(&mut self, x: usize, y: usize);
    fn draw_cursor(&mut self, x: usize, y: usize);
}

trait AdvancedGraphic {
    fn refresh_text_line(&mut self, x1: usize, x2: usize, y: usize);
    fn draw_graphic_buffer<T: Fn(*mut u8, usize, usize, usize) -> IoResult>(&mut self, closure: T) -> IoResult;
    fn set_write_mode(&mut self, write_mode: WriteMode) -> IoResult;
}

#[derive(Debug, Copy, Clone)]
pub enum Color {
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

/// x,y,lines,columns are in unit of char
#[derive(Debug, Copy, Clone)]
struct Cursor {
    pub x: usize,
    pub y: usize,
    pub columns: usize,
    pub lines: usize,
}

/// Enum exported for cursor special API
#[derive(Debug)]
pub enum CursorDirection {
    Left,
    Right,
}

#[derive(Debug)]
enum DrawingMode {
    Vga(VgaTextMode),
    Vbe(VbeMode),
}

/// Control the cursor and can put text on screen thanks to its drawer slave
#[derive(Debug)]
pub struct ScreenMonad {
    drawing_mode: DrawingMode,
    cursor: Cursor,
}

pub static mut SCREEN_MONAD: ScreenMonad = ScreenMonad::new();

impl ScreenMonad {
    // public methods
    /// default VGA_TEXT_MODE
    const fn new() -> Self {
        let vga = VgaTextMode::new();
        let (lines, columns) = vga.query_window_size();
        Self { drawing_mode: DrawingMode::Vga(vga), cursor: Cursor { x: 0, y: 0, columns, lines } }
    }
    /// Switch between VBE mode
    pub fn switch_graphic_mode(&mut self, mode: Option<u16>) -> Result<(), VbeError> {
        let vbe = init_graphic_mode(mode)?;
        let (lines, columns) = vbe.query_window_size();
        self.drawing_mode = DrawingMode::Vbe(vbe);
        self.cursor = Cursor { x: 0, y: 0, columns, lines };
        Ok(())
    }
    /// basic, simple
    pub fn set_text_color(&mut self, color: Color) -> IoResult {
        Drawer::set_text_color(self, color)
    }
    /// void the screen
    pub fn clear_screen(&mut self) {
        Drawer::clear_screen(self);
    }
    /// fill the graphic buffer with a custom fn
    pub fn draw_graphic_buffer<T: Fn(*mut u8, usize, usize, usize) -> IoResult>(&mut self, closure: T) -> IoResult {
        AdvancedGraphic::draw_graphic_buffer(self, closure)
    }
    pub fn set_write_mode(&mut self, write_mode: WriteMode) -> IoResult {
        AdvancedGraphic::set_write_mode(self, write_mode)
    }
    /// set manualy position of cursor
    pub fn set_cursor_position(&mut self, x: usize, y: usize) -> IoResult {
        if x >= self.cursor.columns || y >= self.cursor.lines {
            Err(IoError::CursorOutOfBound)
        } else {
            self.cursor.x = x;
            self.cursor.y = y;
            Ok(())
        }
    }
    /// Erase and Replace graphical cursor
    pub fn move_graphical_cursor(&mut self, direction: CursorDirection, q: usize) -> IoResult {
        // Erase Old cursor
        Drawer::clear_cursor(self, self.cursor.x, self.cursor.y);
        match direction {
            CursorDirection::Right => self.cursor_move_right(q)?,
            CursorDirection::Left => self.cursor_move_left(q)?,
        }
        // create new cursor
        Drawer::draw_cursor(self, self.cursor.x, self.cursor.y);
        Ok(())
    }
    /// get the cursor position
    pub fn get_cursor_position(&mut self) -> (usize, usize) {
        (self.cursor.x, self.cursor.y)
    }
    // private methods
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
    /// move cursor to the right
    fn cursor_move_right(&mut self, q: usize) -> IoResult {
        if q == 0 {
            Ok(())
        } else if self.cursor.y == self.cursor.lines {
            Err(IoError::CursorOutOfBound)
        } else {
            self.cursor.x += 1;
            if self.cursor.x == self.cursor.columns {
                self.cursor.x = 0;
                self.cursor.y += 1;
            }
            self.cursor_move_right(q - 1)
        }
    }
    /// move cursor to the left
    fn cursor_move_left(&mut self, q: usize) -> IoResult {
        if q == 0 {
            Ok(())
        } else if self.cursor.x == 0 && self.cursor.y == 0 {
            Err(IoError::CursorOutOfBound)
        } else {
            if self.cursor.x == 0 {
                self.cursor.x = self.cursor.columns - 1;
                self.cursor.y -= 1;
            } else {
                self.cursor.x -= 1;
            }
            self.cursor_move_left(q - 1)
        }
    }
}

/// private
impl Drawer for ScreenMonad {
    /// put a character into the screen
    fn draw_character(&mut self, c: char, y: usize, x: usize) {
        match &mut self.drawing_mode {
            DrawingMode::Vga(vga) => vga.draw_character(c, y, x),
            DrawingMode::Vbe(vbe) => vbe.draw_character(c, y, x),
        }
    }
    /// just scroll a bit
    fn scroll_screen(&mut self) {
        match &mut self.drawing_mode {
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
    fn set_text_color(&mut self, color: Color) -> IoResult {
        match &mut self.drawing_mode {
            DrawingMode::Vga(vga) => vga.set_text_color(color),
            DrawingMode::Vbe(vbe) => vbe.set_text_color(color),
        }
    }

    /// basic, simple
    fn clear_cursor(&mut self, x: usize, y: usize) {
        match &mut self.drawing_mode {
            DrawingMode::Vga(vga) => vga.clear_cursor(x, y),
            DrawingMode::Vbe(vbe) => vbe.clear_cursor(x, y),
        }
    }
    /// basic, simple
    fn draw_cursor(&mut self, x: usize, y: usize) {
        match &mut self.drawing_mode {
            DrawingMode::Vga(vga) => vga.draw_cursor(x, y),
            DrawingMode::Vbe(vbe) => vbe.draw_cursor(x, y),
        }
    }
}

/// private
impl AdvancedGraphic for ScreenMonad {
    /// command a refresh for selected graphic area
    fn refresh_text_line(&mut self, x1: usize, x2: usize, y: usize) {
        match &mut self.drawing_mode {
            DrawingMode::Vga(_vga) => (),
            DrawingMode::Vbe(vbe) => vbe.refresh_text_line(x1, x2, y),
        }
    }
    /// fill the graphic buffer with a custom function
    fn draw_graphic_buffer<T: Fn(*mut u8, usize, usize, usize) -> IoResult>(&mut self, closure: T) -> IoResult {
        match &mut self.drawing_mode {
            DrawingMode::Vga(_vga) => Err(IoError::GraphicModeNotFounded),
            DrawingMode::Vbe(vbe) => vbe.draw_graphic_buffer(closure),
        }
    }
    fn set_write_mode(&mut self, write_mode: WriteMode) -> IoResult {
        match &mut self.drawing_mode {
            DrawingMode::Vga(_vga) => Err(IoError::GraphicModeNotFounded),
            DrawingMode::Vbe(vbe) => vbe.set_write_mode(write_mode),
        }
    }
}

/// common Write implementation
impl core::fmt::Write for ScreenMonad {
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
