use super::{Color, Drawer, IoResult, Pos};

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
    pub const fn query_window_size(&self) -> (usize, usize) {
        (HEIGHT, WIDTH)
    }
}

impl Drawer for VgaTextMode {
    fn draw_character(&mut self, c: char, position: Pos, color: Color) -> IoResult {
        self.memory_location[position.column + position.line * WIDTH] = (c as u8, Into::<VgaColor>::into(color).0);
        Ok(())
    }

    fn clear_screen(&mut self) {
        unsafe {
            ft_memset(self.memory_location.as_mut_ptr() as *mut u8, 0, WIDTH * HEIGHT * 2);
        }
    }

    fn clear_cursor(&mut self, _c: char, _position: Pos, _color: Color) -> IoResult {
        // wanted fallback
        Ok(())
    }

    fn draw_cursor(&mut self, _c: char, _position: Pos, _color: Color) -> IoResult {
        // wanted fallback
        Ok(())
    }
}
#[derive(Debug, Copy, Clone)]
pub struct VgaColor(pub u8);

impl From<Color> for VgaColor {
    fn from(c: Color) -> Self {
        match c {
            Color::Red => VgaColor(4),
            Color::Green => VgaColor(10),
            Color::Blue => VgaColor(11),
            Color::Yellow => VgaColor(14),
            Color::Cyan => VgaColor(3),
            Color::Magenta => VgaColor(13),
            Color::White => VgaColor(7),
            _ => Color::White.into(),
        }
    }
}

extern "C" {
    pub fn ft_memset(p: *mut u8, val: i32, len: usize) -> *mut u8;
}
