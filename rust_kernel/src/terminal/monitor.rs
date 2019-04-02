pub mod bmp_loader;
mod vbe_mode;
mod vga_text_mode;
use super::cursor::Pos;
use crate::terminal::ansi_escape_code::AnsiColor;

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

/// Drawer is a common trait between VGA and VBE interfaces
pub trait Drawer {
    /// return window size in nb char
    fn query_window_size(&self) -> Pos;
    fn draw_character(&mut self, c: char, position: Pos, color: AnsiColor) -> IoResult;
    fn clear_cursor(&mut self, c: char, position: Pos, color: AnsiColor) -> IoResult;
    fn draw_cursor(&mut self, c: char, position: Pos, color: AnsiColor) -> IoResult;
    fn clear_screen(&mut self);
}

/// AdvancedGraphic is only VBE compatible functions
pub trait AdvancedGraphic {
    fn refresh_screen(&mut self);
    fn refresh_text_line(&mut self, line: usize) -> IoResult;
    fn draw_graphic_buffer<T: Fn(*mut u8, usize, usize, usize) -> IoResult>(&mut self, closure: T) -> IoResult;
    fn query_graphic_infos(&self) -> Result<(usize, usize, usize), IoError>;
}

/// Manage interaction between monitor/graphic_card and software
pub struct ScreenMonad {
    drawing_mode: DrawingMode,
    size: Pos,
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
        let size = vga.query_window_size();
        Self { drawing_mode: DrawingMode::Vga(vga), size }
    }
    /// Switch between VBE mode
    pub fn switch_graphic_mode(&mut self, mode: u16) -> Result<(), VbeError> {
        let vbe = init_graphic_mode(mode)?;
        self.size = vbe.query_window_size();
        self.drawing_mode = DrawingMode::Vbe(vbe);
        Ok(())
    }
    /// Check the bounds
    fn check_bound(&self, position: Pos) -> IoResult {
        if position.line >= self.size.line || position.column >= self.size.column {
            Err(IoError::OutOfBound)
        } else {
            Ok(())
        }
    }
    pub fn is_graphic(&self) -> bool {
        match &self.drawing_mode {
            DrawingMode::Vga(_vga) => false,
            DrawingMode::Vbe(_vbe) => true,
        }
    }
}

impl Drawer for ScreenMonad {
    fn query_window_size(&self) -> Pos {
        self.size
    }
    /// Put a character into the screen
    fn draw_character(&mut self, c: char, position: Pos, color: AnsiColor) -> IoResult {
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
    fn clear_cursor(&mut self, c: char, position: Pos, color: AnsiColor) -> IoResult {
        self.check_bound(position)?;
        match &mut self.drawing_mode {
            DrawingMode::Vga(vga) => vga.clear_cursor(c, position, color),
            DrawingMode::Vbe(vbe) => vbe.clear_cursor(c, position, color),
        }
    }
    /// Draw cursor in a specified area
    fn draw_cursor(&mut self, c: char, position: Pos, color: AnsiColor) -> IoResult {
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
            DrawingMode::Vga(_vga) => Ok(()),
            DrawingMode::Vbe(vbe) => vbe.draw_graphic_buffer(closure),
        }
    }

    fn query_graphic_infos(&self) -> Result<(usize, usize, usize), IoError> {
        match &self.drawing_mode {
            DrawingMode::Vga(_vga) => Err(IoError::NotSupported),
            DrawingMode::Vbe(vbe) => vbe.query_graphic_infos(),
        }
    }
}
