use core::fmt::Result;
use core::fmt::Write;

#[derive(Debug)]
pub struct VgaTerminal {
    width:isize,
    height:isize,
    x:isize,
    y:isize,
    color:u8,
}

impl Write for VgaTerminal {
    fn write_str(&mut self, s: &str) -> Result {
        for c in s.as_bytes() {
            match *c as char {
                '\n' => {
                    self.x = 1;
                    self.y = self.y + 1;
                }
                _ => {
                    putchar(self.x - 1 + (self.y - 1) * self.width, *c as char, self.color);
                    self.x = self.x + 1;
                    if self.x > self.width {
                        self.y = self.y + 1;
                        self.x = 1;
                    }
                }
            }
        }
        Ok(())
    }
}

pub fn putstring(mut pos: isize, s:&str, color: u8) -> isize {

    for c in s.as_bytes() {
        pos = putchar(pos, *c as char, color);
    }
    pos
}

pub fn putchar(pos:isize, c:char, color:u8) -> isize {
    let ptr = 0xB8000 as *mut u8;

    unsafe {
        *ptr.offset(pos * 2) = c as u8;
        *ptr.offset(pos * 2 + 1) = color;
    }
    pos + 1
}

pub static mut VGA_TERM: VgaTerminal = VgaTerminal {width: 80, height: 25, x: 1, y: 1, color: 3};

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
