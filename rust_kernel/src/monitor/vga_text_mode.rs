use super::{Buffer, Color, Drawer, IoError, IoResult};

extern "C" {
    pub fn ft_memset(p: *mut u8, val: i32, len: usize) -> *mut u8;
    pub fn ft_memmove(dst: *mut u8, src: *mut u8, len: usize) -> *mut u8;
}

const HEIGHT: usize = 25;
const WIDTH: usize = 80;

#[derive(Debug)]
struct VgaBuffer(pub *mut u8);

unsafe impl Send for VgaBuffer {}

#[derive(Debug)]
pub struct VgaTextMode {
    memory_location: VgaBuffer,
    color: u8,
}

impl VgaTextMode {
    pub const fn new() -> Self {
        Self { memory_location: VgaBuffer(0xb8000 as *mut u8), color: 3 }
    }
    /// return window size in nb char
    pub const fn query_window_size(&self) -> (usize, usize) {
        (HEIGHT, WIDTH)
    }
}

impl Drawer for VgaTextMode {
    fn draw_character(&mut self, c: char, y: usize, x: usize) {
        let ptr = self.memory_location.0;
        let pos = x + y * WIDTH;

        unsafe {
            *ptr.add(pos * 2) = c as u8;
            *ptr.add(pos * 2 + 1) = self.color;
        }
    }
    fn scroll_screen(&mut self) {
        let ptr = self.memory_location.0;
        unsafe {
            ft_memmove(ptr, ptr.add(WIDTH * 2), WIDTH * (HEIGHT - 1) * 2);
            ft_memset(ptr.add(WIDTH * (HEIGHT - 1) * 2), 0, WIDTH * 2);
        }
    }
    fn clear_screen(&mut self, _buffers: Buffer) {
        unsafe {
            ft_memset(self.memory_location.0, 0, WIDTH * HEIGHT * 2);
        }
    }
    fn set_text_color(&mut self, color: Color) -> IoResult {
        let u8color: u8 = match color {
            Color::Blue => 11,
            Color::Green => 10,
            Color::Yellow => 14,
            Color::Cyan => 3,
            Color::Red => 4,
            Color::Magenta => 13,
            Color::White => 7,
            _ => return Err(IoError::ColorNotSupported),
        };
        self.color = u8color;
        Ok(())
    }

    fn clear_cursor(&mut self, _cursor_x: usize, _cursor_y: usize) {
        // wanted fallback
    }
    fn draw_cursor(&mut self, _cursor_x: usize, _cursor_y: usize) {
        // wanted fallback
    }
}
