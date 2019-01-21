pub type Result = core::result::Result<(), &'static str>;

pub trait IoScreen {
    fn set_graphic_mode(&mut self, _mode: u16) -> Result {
        Ok(())
    }
    fn putchar(&mut self, c: char) -> Result;
    fn scroll_screen(&mut self) -> Result;
    fn clear_screen(&mut self) -> Result;
    fn set_text_color(&mut self, color: TextColor) -> Result;
    fn set_cursor_position(&mut self, x: usize, y: usize) -> Result;
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
