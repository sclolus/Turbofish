#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![no_std]
#![feature(compiler_builtins_lib)]

pub mod support; // For Rust lang items
use core::fmt::Arguments;
use core::fmt::write;
use core::fmt::Write;
use core::fmt::Result;

#[no_mangle]
pub extern "C" fn kmain() {
    let mut pos = putchar_vga(1, 'H', 12);
    pos = putchar_vga(pos, 'E', 13);
    pos = putchar_vga(pos, 'L', 14);
    pos = putchar_vga(pos, 'L', 15);
    pos = putchar_vga(pos, 'O', 16);
    pos = putchar_vga(pos, ' ', 17);
    pos = putchar_vga(pos, 'W', 18);
    pos = putchar_vga(pos, 'O', 19);
    pos = putchar_vga(pos, 'R', 20);
    pos = putchar_vga(pos, 'L', 21);
    pos = putchar_vga(pos, 'D', 22);
    pos = putchar_vga(pos, '!', 23);
    pos = putstring_vga(pos, "LALA", 8);
    let mut vga = Vga {};
    write(&mut vga, format_args!("hello {:?}", 12));
    loop { }
}

struct Vga {
}

impl Write for Vga {
    fn write_str(&mut self, s: &str) -> Result {
        putstring_vga(140, s, 8);
        Ok(())
    }
}

fn putstring_vga(mut pos: isize, s:&str, color: u8) -> isize {
    for c in s.as_bytes() {
        pos = putchar_vga(pos, *c as char, color);
    }
    pos
}

fn putchar_vga(pos:isize, c:char, color:u8) -> isize {
    let ptr = 0xB8000 as *mut u8;

    unsafe {
        *ptr.offset(pos * 2) = c as u8;
        *ptr.offset(pos * 2 + 1) = color;
    }
    pos + 1
}
