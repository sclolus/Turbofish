use crate::monitor::core_monitor::*;
use core::fmt::Write;

#[derive(Debug)]
pub struct VbeMode {
    x:usize,
    y:usize,
    width:usize,
    height:usize,
}

pub static mut SVGA_VBE: VbeMode =
    VbeMode {x: 0, y: 0, width: 0, height: 0};

impl IoScreen for VbeMode {
    fn putchar(&mut self, _c:char) -> Result {
        Ok(())
    }
    fn scroll_screen(&mut self) -> Result {
        Ok(())
    }
    fn clear_screen(&mut self) -> Result {
        Ok(())
    }
    fn set_text_color(&mut self, _color:TextColor) -> Result {
        Ok(())
    }
    fn set_cursor_position(&mut self, _x:usize, _y:usize) -> Result {
        Ok(())
    }
}

impl Write for VbeMode {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.as_bytes() {
            match *c as char {
                '\n' => {
                    self.x = 0;
                    self.y = self.y + 1;
                    if self.y == self.height {
                        self.scroll_screen().unwrap();
                    }
                }
                _ => {
                    self.putchar(*c as char).unwrap();
                    self.x = self.x + 1;
                    if self.x == self.width {
                        self.x = 0;
                        self.y = self.y + 1;
                        if self.y == self.height {
                            self.scroll_screen().unwrap();;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
