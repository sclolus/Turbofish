pub mod core_monitor;
use crate::monitor::core_monitor::*;
pub mod vga_text_mode;
use crate::monitor::vga_text_mode::*;
pub mod vbe_mode;
//use crate::monitor::vbe_mode::*;

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => ({
        unsafe {
            core::fmt::write(&mut $crate::monitor::vga_text_mode::VGA_TEXT, format_args!($($arg)*)).unwrap();
            core::fmt::write(&mut $crate::monitor::vga_text_mode::VGA_TEXT, format_args!("\n")).unwrap();
        }
    })
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        unsafe {
            core::fmt::write(&mut $crate::monitor::vga_text_mode::VGA_TEXT, format_args!($($arg)*)).unwrap();
        }
    })
}

pub fn clear_screen() -> () {
    unsafe {
        VGA_TEXT.clear_screen().unwrap();
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
        _ => return Err("color not defined"),
    };
    unsafe {
        match VGA_TEXT.set_text_color(color) {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

pub fn set_cursor_position(x:usize, y:usize) -> Result {
    unsafe {
        match VGA_TEXT.set_cursor_position(x, y) {
            Ok(()) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
