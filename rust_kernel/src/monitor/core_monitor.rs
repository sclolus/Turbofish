pub type Result = core::result::Result<(), &'static str>;

pub trait IoScreen {
    fn putchar(&mut self, c:char) -> Result;
    fn scroll_screen(&mut self) -> Result;
    fn clear_screen(&mut self) -> Result;
    fn set_text_color(&mut self, color:TextColor) -> Result;
    fn set_cursor_position(&mut self, x:usize, y:usize) -> Result;
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
