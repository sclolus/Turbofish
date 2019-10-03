mod font;
pub use font::*;
mod init;
pub use init::*;
mod rgb;
use rgb::RGB;

use super::{AdvancedGraphic, Drawer, IoError, IoResult, Pos};
use alloc::vec;
use alloc::vec::Vec;
use ansi_escape_code::AnsiColor;

#[derive(Debug, Clone)]
pub struct VbeMode {
    /// linear frame buffer address
    linear_frame_buffer: LinearFrameBuffer,
    /// double framebuffer location
    db_frame_buffer: Vec<u8>,
    /// graphic buffer location
    graphic_buffer: Vec<u8>,
    /// in pixel
    width: usize,
    /// in pixel
    height: usize,
    /// in bytes
    bytes_per_pixel: usize,
    /// bytes_per_line
    pitch: usize,
    /// in pixel
    char_height: usize,
    /// in pixel
    char_width: usize,
    /// characters per columns
    columns: usize,
    /// number of characters lines
    lines: usize,
    /// Some informations about graphic mode
    mode_info: ModeInfo,
    /// Some informations about how the screen manage display
    crtc_info: Option<CrtcInfo>,
}

#[derive(Debug, Copy, Clone)]
struct LinearFrameBuffer(pub *mut u8);

unsafe impl Send for LinearFrameBuffer {}

impl VbeMode {
    pub fn new(
        linear_frame_buffer: *mut u8,
        width: usize,
        height: usize,
        bpp: usize,
        mode_info: ModeInfo,
    ) -> Self {
        let bytes_per_pixel: usize = bpp / 8;
        let screen_size: usize = bytes_per_pixel * width * height;
        let columns: usize = unsafe { width / _font_width };
        let lines: usize = unsafe { height / _font_height };
        Self {
            linear_frame_buffer: LinearFrameBuffer(linear_frame_buffer),
            // Never trust the borrow checker ! Adding 1 for 24bpp mode
            db_frame_buffer: vec![0; screen_size + 1],
            graphic_buffer: vec![0; screen_size + 1],
            width,
            height,
            bytes_per_pixel,
            pitch: width * bytes_per_pixel,
            char_width: unsafe { _font_width },
            char_height: unsafe { _font_height },
            columns: columns,
            lines: lines,
            mode_info,
            crtc_info: None,
        }
    }

    /// put pixel at position y, x in pixel unit
    #[inline(always)]
    fn put_pixel(&mut self, y: usize, x: usize, color: RGB) {
        let loc = y * self.pitch + x * self.bytes_per_pixel;
        unsafe {
            // Be carefull, in 24 bpp mode, the last pixel overflow by one byte !
            *(self.db_frame_buffer.as_mut_ptr().add(loc) as *mut u32) = color.0 as u32;
        }
    }

    /// write a single character with common rules
    #[inline(always)]
    fn write_char(&mut self, char_font: &[u8], line: usize, column: usize, color: RGB) {
        let mut y = line * self.char_height;
        let mut x;
        for l in char_font {
            x = column * self.char_width;
            for shift in (0..8).rev() {
                if *l & 1 << shift != 0 {
                    self.put_pixel(y, x, color);
                }
                x += 1;
            }
            y += 1;
        }
    }

    /// write a cursor with common rules
    #[inline(always)]
    fn write_cursor(&mut self, char_font: &[u8], line: usize, column: usize, color: RGB) {
        let mut y = line * self.char_height;
        let mut x;
        for l in char_font {
            x = column * self.char_width;
            for shift in (0..8).rev() {
                if *l & 1 << shift == 0 {
                    self.put_pixel(y, x, color);
                }
                x += 1;
            }
            y += 1;
        }
    }

    /// copy one bounded area line from graphic buffer to db frame buffer
    fn copy_graphic_buffer_line_area(&mut self, line: usize, column_1: usize, column_2: usize) {
        for i in 0..self.char_height {
            let o1 = (line * self.char_height + i) * self.pitch
                + column_1 * self.char_width * self.bytes_per_pixel;
            let o2 = o1 + (column_2 - column_1) * self.char_width * self.bytes_per_pixel;
            self.db_frame_buffer[o1..o2].copy_from_slice(&self.graphic_buffer[o1..o2]);
        }
    }
}

impl Drawer for VbeMode {
    /// return window size in nb char
    fn query_window_size(&self) -> Pos {
        Pos {
            line: self.height / self.char_height,
            column: self.width / self.char_width,
        }
    }

    #[inline(always)]
    fn draw_character(&mut self, c: char, position: Pos, color: AnsiColor) -> IoResult {
        self.copy_graphic_buffer_line_area(position.line, position.column, position.column + 1);

        let font = unsafe { _font.get_char(c as u8) };
        self.write_char(font, position.line, position.column, color.into());
        Ok(())
    }

    fn clear_screen(&mut self) {
        // Copy the entire graphic buffer
        unsafe {
            _sse2_memcpy(
                self.db_frame_buffer.as_mut_ptr(),
                self.graphic_buffer.as_ptr(),
                self.pitch * self.height,
            );
        }
    }

    fn clear_cursor(&mut self, c: char, position: Pos, color: AnsiColor) -> IoResult {
        self.draw_character(c, position, color)
    }

    fn draw_cursor(&mut self, c: char, position: Pos, color: AnsiColor) -> IoResult {
        self.copy_graphic_buffer_line_area(position.line, position.column, position.column + 1);

        let font = unsafe { _font.get_char(c as u8) };
        self.write_cursor(font, position.line, position.column, color.into());
        Ok(())
    }
}

impl AdvancedGraphic for VbeMode {
    /// refresh framebuffer
    fn refresh_screen(&mut self) {
        unsafe {
            _sse2_memcpy(
                self.linear_frame_buffer.0,
                self.db_frame_buffer.as_ptr(),
                self.pitch * self.height,
            );
        }
    }

    /// Display an entire line in the screen: Be carefull, all characters after end are cleared !
    fn refresh_text_line(&mut self, line: usize) -> IoResult {
        let offset = line * self.pitch * self.char_height;
        unsafe {
            _sse2_memcpy(
                self.linear_frame_buffer.0.add(offset),
                self.db_frame_buffer.as_ptr().add(offset),
                self.pitch * self.char_height,
            );
        }
        Ok(())
    }

    /// Expose graphic buffer
    fn draw_graphic_buffer<T: Fn(*mut u8, usize, usize, usize) -> IoResult>(
        &mut self,
        closure: T,
    ) -> IoResult {
        closure(
            self.graphic_buffer.as_mut_ptr(),
            self.width,
            self.height,
            self.bytes_per_pixel * 8,
        )?;
        self.clear_screen();
        Ok(())
    }

    fn query_graphic_infos(&self) -> Result<(usize, usize, usize), IoError> {
        Ok((self.height, self.width, self.bytes_per_pixel * 8))
    }
}

extern "C" {
    /* Fast and Furious ASM SSE2 method to copy entire buffers */
    fn _sse2_memcpy(dst: *mut u8, src: *const u8, len: usize) -> ();
}
