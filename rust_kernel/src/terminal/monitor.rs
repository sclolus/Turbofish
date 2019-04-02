pub mod bmp_loader;
mod vbe_mode;
mod vga_text_mode;
use super::cursor::Pos;

use crate::Spinlock;
use lazy_static::lazy_static;
use vbe_mode::*;
use vga_text_mode::*;

/// IoResult is just made to handle module errors
pub type IoResult = core::result::Result<(), IoError>;

/// Common errors for this module
#[derive(Debug, Copy, Clone)]
pub enum IoError {
    OutOfBound,
    ColorNotSupported,
    GraphicModeNotFounded,
    NotSupported,
}

/// Human readable colors
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

/// Drawer is a common trait between VGA and VBE interfaces
pub trait Drawer {
    fn draw_character(&mut self, c: char, position: Pos, color: Color) -> IoResult;
    fn clear_cursor(&mut self, c: char, position: Pos, color: Color) -> IoResult;
    fn draw_cursor(&mut self, c: char, position: Pos, color: Color) -> IoResult;
    fn clear_screen(&mut self);
}

/// AdvancedGraphic is only VBE compatible functions
pub trait AdvancedGraphic {
    fn refresh_screen(&mut self);
    fn refresh_text_line(&mut self, line: usize) -> IoResult;
    fn draw_graphic_buffer<T: Fn(*mut u8, usize, usize, usize) -> IoResult>(&mut self, closure: T) -> IoResult;
}

/// Manage interaction between monitor/graphic_card and software
pub struct ScreenMonad {
    drawing_mode: DrawingMode,
    pub nb_lines: usize,
    pub nb_columns: usize,
    pub height: Option<usize>,
    pub width: Option<usize>,
    pub bpp: Option<usize>,
}

enum DrawingMode {
    Vga(VgaTextMode),
    Vbe(VbeMode),
}

lazy_static! {
    /// Output monad
    pub static ref SCREEN_MONAD: Spinlock<ScreenMonad> = Spinlock::new(ScreenMonad::new());
}

impl ScreenMonad {
    /// default is vga
    fn new() -> Self {
        let vga = VgaTextMode::new();
        let (lines, columns, _, _, _) = vga.query_window_size();
        Self {
            drawing_mode: DrawingMode::Vga(vga),
            nb_lines: lines,
            nb_columns: columns,
            height: None,
            width: None,
            bpp: None,
        }
    }
    /// Switch between VBE mode
    pub fn switch_graphic_mode(&mut self, mode: u16) -> Result<(), VbeError> {
        let vbe = init_graphic_mode(mode)?;
        let (lines, columns, height, width, bpp) = vbe.query_window_size();
        self.drawing_mode = DrawingMode::Vbe(vbe);
        self.nb_lines = lines;
        self.nb_columns = columns;
        self.height = height;
        self.width = width;
        self.bpp = bpp;
        Ok(())
    }
    /// Check the bounds
    fn check_bound(&self, position: Pos) -> IoResult {
        if position.line >= self.nb_lines || position.column >= self.nb_columns {
            Err(IoError::OutOfBound)
        } else {
            Ok(())
        }
    }
}

impl Drawer for ScreenMonad {
    /// Put a character into the screen
    fn draw_character(&mut self, c: char, position: Pos, color: Color) -> IoResult {
        self.check_bound(position)?;
        match &mut self.drawing_mode {
            DrawingMode::Vga(vga) => vga.draw_character(c, position, color),
            DrawingMode::Vbe(vbe) => vbe.draw_character(c, position, color),
        }
    }
    /// Fflush the screen
    fn clear_screen(&mut self) {
        match &mut self.drawing_mode {
            DrawingMode::Vga(vga) => vga.clear_screen(),
            DrawingMode::Vbe(vbe) => vbe.clear_screen(),
        }
    }
    /// Clear cursor in a specified area
    fn clear_cursor(&mut self, c: char, position: Pos, color: Color) -> IoResult {
        self.check_bound(position)?;
        match &mut self.drawing_mode {
            DrawingMode::Vga(vga) => vga.clear_cursor(c, position, color),
            DrawingMode::Vbe(vbe) => vbe.clear_cursor(c, position, color),
        }
    }
    /// Draw cursor in a specified area
    fn draw_cursor(&mut self, c: char, position: Pos, color: Color) -> IoResult {
        self.check_bound(position)?;
        match &mut self.drawing_mode {
            DrawingMode::Vga(vga) => vga.draw_cursor(c, position, color),
            DrawingMode::Vbe(vbe) => vbe.draw_cursor(c, position, color),
        }
    }
}

impl AdvancedGraphic for ScreenMonad {
    /// Refresh all the screen
    fn refresh_screen(&mut self) {
        match &mut self.drawing_mode {
            DrawingMode::Vga(_vga) => (),
            DrawingMode::Vbe(vbe) => vbe.refresh_screen(),
        }
    }
    /// Command a refresh for selected graphic area
    fn refresh_text_line(&mut self, line: usize) -> IoResult {
        self.check_bound(Pos { line, column: 0 })?;
        match &mut self.drawing_mode {
            DrawingMode::Vga(_vga) => Ok(()),
            DrawingMode::Vbe(vbe) => vbe.refresh_text_line(line),
        }
    }
    /// Fill the graphic buffer with a custom function
    fn draw_graphic_buffer<T: Fn(*mut u8, usize, usize, usize) -> IoResult>(&mut self, closure: T) -> IoResult {
        match &mut self.drawing_mode {
            DrawingMode::Vga(_vga) => Err(IoError::GraphicModeNotFounded),
            DrawingMode::Vbe(vbe) => vbe.draw_graphic_buffer(closure),
        }
    }
}
