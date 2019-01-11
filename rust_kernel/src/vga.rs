use core::fmt::Result;
use core::fmt::Write;

const VGA_MEM_LOCATION: usize = 0xb8000;

#[derive(Debug)]
pub struct VgaTerminal {
    width:usize,
    height:usize,
    x:usize,
    y:usize,
    color:u8,
}

pub static mut VGA_TERM: VgaTerminal = VgaTerminal {width: 80, height: 25, x: 1, y: 1, color: 3};

impl Write for VgaTerminal {
fn write_str(&mut self, s: &str) -> Result<> {
        for c in s.as_bytes() {
            match *c as char {
                '\n' => {
                    self.x = 1;
                    self.y = self.y + 1;
                    if self.y > self.height {
                        scroll_screen();
                    }
                }
                _ => {
                    putchar(self.x - 1 + (self.y - 1) * self.width, *c as char, self.color);
                    self.x = self.x + 1;
                    if self.x > self.width {
                        self.y = self.y + 1;
                        self.x = 1;
                        if self.y > self.height {
                            scroll_screen();
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

fn putchar(pos:usize, c:char, color:u8) -> usize {
    let ptr = VGA_MEM_LOCATION as *mut u8;

    unsafe {
        *ptr.add(pos * 2) = c as u8;
        *ptr.add(pos * 2 + 1) = color;
    }
    pos + 1
}

fn scroll_screen() -> (){
    use crate::support::memmove;
    use crate::support::memset;

    let ptr = VGA_MEM_LOCATION as *mut u8;
    unsafe {
        memmove(ptr, ptr.add(VGA_TERM.width * 2), VGA_TERM.width * (VGA_TERM.height - 1) * 2);
        memset(ptr.add(VGA_TERM.width * (VGA_TERM.height - 1) * 2), 0, VGA_TERM.width * 2);
        VGA_TERM.y -= 1;
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

/* Keep in mind that Rust use SSE feature when it used with some optimization level */
pub fn clear_screen() -> () {
    use crate::support::memset;
    unsafe {
        memset(VGA_MEM_LOCATION as *mut u8, 0, VGA_TERM.width * VGA_TERM.height * 2);
        VGA_TERM.x = 1;
        VGA_TERM.y = 1;
    }
}
