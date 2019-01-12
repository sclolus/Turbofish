use core::fmt::Result;
use core::fmt::Write;

trait IoScreen {
    fn putchar(&mut self, c:char) -> Result;
    fn scroll_screen(&mut self) -> Result;
    fn clear_screen(&mut self) -> Result;
    fn set_text_color(&mut self, color:TextColor) -> Result;
}

pub enum TextColor {
    Red,
    Green,
    Yellow,
    Cyan,
    Brown,
    Magenta,
    Blue,
    White,
}

#[derive(Debug)]
pub struct VgaTerminal {
    memory_location: *mut u8,
    width:usize,
    height:usize,
    x:usize,
    y:usize,
    color:u8,
}

pub static mut VGA_TERM: VgaTerminal =
    VgaTerminal {memory_location: 0xb8000 as *mut u8, width: 80, height: 25, x: 0, y: 0, color: 3};

impl IoScreen for VgaTerminal {
    fn putchar(&mut self, c:char) -> Result {
        let ptr = self.memory_location;
        let pos = self.x + self.y * self.width;

        unsafe {
            *ptr.add(pos * 2) = c as u8;
            *ptr.add(pos * 2 + 1) = self.color;
        }
        Ok(())
    }
    fn scroll_screen(&mut self) -> Result {
        use crate::support::memmove;
        use crate::support::memset;

        let ptr = self.memory_location;
        unsafe {
            memmove(ptr, ptr.add(self.width * 2), self.width * (self.height - 1) * 2);
            memset(ptr.add(self.width * (self.height - 1) * 2), 0, self.width * 2);
        }
        self.y -= 1;
        Ok(())
    }
    fn clear_screen(&mut self) -> Result {
        use crate::support::memset;
        unsafe {
            memset(self.memory_location, 0, self.width * self.height * 2);
        }
        self.x = 0;
        self.y = 0;
        Ok(())
    }
    fn set_text_color(&mut self, color:TextColor) -> Result {
        self.color = match color {
            TextColor::Blue => 11,
            TextColor::Green => 10,
            TextColor::Yellow => 14,
            TextColor::Cyan => 3,
            TextColor::Red => 4,
            TextColor::Magenta => 13,
            TextColor::White => 7,
            _ => self.color,
        };
        Ok(())
    }
}

impl Write for VgaTerminal {
    fn write_str(&mut self, s: &str) -> Result {
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

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => ({
        unsafe {
            core::fmt::write(&mut $crate::vga::VGA_TERM, format_args_nl!($($arg)*)).unwrap();
        }
    })
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        unsafe {
            core::fmt::write(&mut $crate::vga::VGA_TERM, format_args!($($arg)*)).unwrap();
        }
    })
}

pub fn clear_screen() -> () {
    unsafe {
        VGA_TERM.clear_screen().unwrap();
    }
}

pub fn set_text_color(s: &'static str) -> Result {
    let color: TextColor = match s {
        "red" => TextColor::Red,
        "green" => TextColor::Green,
        "yellow" => TextColor::Yellow,
        "cyan" => TextColor::Cyan,
        "brown" => TextColor::Brown,
        "magenta" => TextColor::Magenta,
        "blue" => TextColor::Blue,
        "white" => TextColor::White,
        _ => TextColor::White,
    };
    unsafe {
        match VGA_TERM.set_text_color(color) {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
