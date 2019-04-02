use super::{Drawer, IoResult, Pos};
use crate::terminal::ansi_escape_code::{AnsiColor, StandardColor};

const HEIGHT: usize = 25;
const WIDTH: usize = 80;

pub struct VgaTextMode {
    memory_location: &'static mut [(u8, u8); WIDTH * HEIGHT],
}

impl VgaTextMode {
    pub fn new() -> Self {
        unsafe { Self { memory_location: &mut *(0xb8000 as *mut [(u8, u8); WIDTH * HEIGHT]) } }
    }

    /// return window size in nb char
    pub const fn query_window_size(&self) -> (usize, usize, Option<usize>, Option<usize>, Option<usize>) {
        (HEIGHT, WIDTH, None, None, None)
    }
}

impl Drawer for VgaTextMode {
    fn draw_character(&mut self, c: char, position: Pos, color: AnsiColor) -> IoResult {
        self.memory_location[position.column + position.line * WIDTH] = (c as u8, Into::<VgaColor>::into(color).0);
        Ok(())
    }

    fn clear_screen(&mut self) {
        unsafe {
            ft_memset(self.memory_location.as_mut_ptr() as *mut u8, 0, WIDTH * HEIGHT * 2);
        }
    }

    fn clear_cursor(&mut self, _c: char, _position: Pos, _color: AnsiColor) -> IoResult {
        // wanted fallback
        Ok(())
    }

    fn draw_cursor(&mut self, _c: char, _position: Pos, _color: AnsiColor) -> IoResult {
        // wanted fallback
        Ok(())
    }
}
#[derive(Debug, Copy, Clone)]
pub struct VgaColor(pub u8);

impl From<StandardColor> for VgaColor {
    fn from(c: StandardColor) -> Self {
        use StandardColor::*;
        match c {
            //TODO: Where to find the code ?
            Black => VgaColor(0),
            Red => VgaColor(4),
            Green => VgaColor(10),
            Blue => VgaColor(11),
            Yellow => VgaColor(14),
            Cyan => VgaColor(3),
            Magenta => VgaColor(13),
            White => VgaColor(7),
        }
    }
}

impl From<AnsiColor> for VgaColor {
    fn from(c: AnsiColor) -> Self {
        match c {
            // Convert only the 8 Standard Ansi color
            AnsiColor::Standard(c) => c.into(),
            // Otherwise set default to white
            _ => VgaColor(7),
        }
    }
}

extern "C" {
    pub fn ft_memset(p: *mut u8, val: i32, len: usize) -> *mut u8;
}
