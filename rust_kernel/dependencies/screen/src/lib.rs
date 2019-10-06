//! Handle the monitor with different graphic modes
#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod bmp_loader;
mod vbe_mode;
mod vga_text_mode;
use ansi_escape_code::{AnsiColor, Pos};

use vbe_mode::{init_graphic_mode, VbeError, VbeMode};
use vga_text_mode::VgaTextMode;

/// IoResult is just made to handle module errors
pub type IoResult = core::result::Result<(), IoError>;

/// Common errors for this module
#[derive(Debug, Copy, Clone)]
pub enum IoError {
    /// Not a valid position
    OutOfBound,
    /// Cannot apply the selected color
    ColorNotSupported,
    /// Common error Variant
    NotSupported,
}

/// Drawer is a common trait between VGA and VBE interfaces
pub trait Drawer {
    /// Return window size in nb char
    fn query_window_size(&self) -> Pos;
    /// Draw a character
    fn draw_character(&mut self, c: char, position: Pos, color: AnsiColor) -> IoResult;
    /// Erase a cursor (must specify color and character)
    fn clear_cursor(&mut self, c: char, position: Pos, color: AnsiColor) -> IoResult;
    /// Draw a cursor (must specify color and character)
    fn draw_cursor(&mut self, c: char, position: Pos, color: AnsiColor) -> IoResult;
    /// Clear the entire screen
    fn clear_screen(&mut self);
}

/// AdvancedGraphic is only VBE compatible functions
pub trait AdvancedGraphic {
    /// Refresh all the screen
    fn refresh_screen(&mut self);
    /// Command a refresh for selected graphic area
    fn refresh_text_line(&mut self, line: usize) -> IoResult;
    /// Fill the graphic buffer with a custom function
    fn draw_graphic_buffer<T: Fn(*mut u8, usize, usize, usize) -> IoResult>(
        &mut self,
        closure: T,
    ) -> IoResult;
    /// Query info about screeb composition
    fn query_graphic_infos(&self) -> Result<(usize, usize, usize), IoError>;
}

/// Manage interaction between monitor/graphic_card and software
pub struct ScreenMonad {
    drawing_mode: DrawingMode,
    size: Pos,
}

pub enum DrawingMode {
    Vga(VgaTextMode),
    Vbe(VbeMode),
}

impl ScreenMonad {
    /// default is vga
    pub fn new() -> Self {
        let vga = VgaTextMode::new();
        let size = vga.query_window_size();
        Self {
            drawing_mode: DrawingMode::Vga(vga),
            size,
        }
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
    /// Ask if a mode is graphic
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
    fn draw_character(&mut self, c: char, position: Pos, color: AnsiColor) -> IoResult {
        self.check_bound(position)?;
        match &mut self.drawing_mode {
            DrawingMode::Vga(vga) => vga.draw_character(c, position, color),
            DrawingMode::Vbe(vbe) => vbe.draw_character(c, position, color),
        }
    }
    fn clear_screen(&mut self) {
        match &mut self.drawing_mode {
            DrawingMode::Vga(vga) => vga.clear_screen(),
            DrawingMode::Vbe(vbe) => vbe.clear_screen(),
        }
    }
    fn clear_cursor(&mut self, c: char, position: Pos, color: AnsiColor) -> IoResult {
        self.check_bound(position)?;
        match &mut self.drawing_mode {
            DrawingMode::Vga(vga) => vga.clear_cursor(c, position, color),
            DrawingMode::Vbe(vbe) => vbe.clear_cursor(c, position, color),
        }
    }
    fn draw_cursor(&mut self, c: char, position: Pos, color: AnsiColor) -> IoResult {
        self.check_bound(position)?;
        match &mut self.drawing_mode {
            DrawingMode::Vga(vga) => vga.draw_cursor(c, position, color),
            DrawingMode::Vbe(vbe) => vbe.draw_cursor(c, position, color),
        }
    }
}

impl AdvancedGraphic for ScreenMonad {
    fn refresh_screen(&mut self) {
        match &mut self.drawing_mode {
            DrawingMode::Vga(_vga) => (),
            DrawingMode::Vbe(vbe) => vbe.refresh_screen(),
        }
    }
    fn refresh_text_line(&mut self, line: usize) -> IoResult {
        self.check_bound(Pos { line, column: 0 })?;
        match &mut self.drawing_mode {
            DrawingMode::Vga(_vga) => Ok(()),
            DrawingMode::Vbe(vbe) => vbe.refresh_text_line(line),
        }
    }
    fn draw_graphic_buffer<T: Fn(*mut u8, usize, usize, usize) -> IoResult>(
        &mut self,
        closure: T,
    ) -> IoResult {
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
