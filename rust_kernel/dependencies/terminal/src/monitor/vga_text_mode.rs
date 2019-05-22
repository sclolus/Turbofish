use super::{Drawer, IoResult, Pos};
use crate::ansi_escape_code::{AnsiColor, StandardColor};
use io::{Io, Pio};

const HEIGHT: usize = 25;
const WIDTH: usize = 80;
const VGA_MEM_LOCATION: *mut u8 = 0xb8000 as *mut u8;

pub struct VgaTextMode {
    memory_location: &'static mut [(u8, u8); WIDTH * HEIGHT],
}

impl VgaTextMode {
    // see https://wiki.osdev.org/Text_Mode_Cursor for info about cursor
    const CURSOR_INDEX_REGISTER: u16 = 0x3D4;
    const CURSOR_DATA_REGISTER: u16 = 0x3D5;

    pub fn new() -> Self {
        Self { memory_location: unsafe { &mut *(VGA_MEM_LOCATION as *mut [(u8, u8); WIDTH * HEIGHT]) } }
    }
}

impl Drawer for VgaTextMode {
    fn query_window_size(&self) -> Pos {
        Pos { line: HEIGHT, column: WIDTH }
    }
    fn draw_character(&mut self, c: char, position: Pos, color: AnsiColor) -> IoResult {
        self.memory_location[position.column + position.line * WIDTH] = (c as u8, Into::<VgaColor>::into(color).0);
        Ok(())
    }

    fn clear_screen(&mut self) {
        // get the current cursor position
        Pio::<u8>::new(Self::CURSOR_INDEX_REGISTER).write(0x0F);
        let column = Pio::<u8>::new(Self::CURSOR_DATA_REGISTER).read();
        Pio::<u8>::new(Self::CURSOR_INDEX_REGISTER).write(0x0E);
        let line = Pio::<u8>::new(Self::CURSOR_DATA_REGISTER).read();

        // clear the cursor
        self.clear_cursor(' ', Pos { line: line as usize, column: column as usize }, AnsiColor::WHITE).unwrap();

        // fill screen with white non visible cells (cursor will be white)
        unsafe {
            _screencpy_des_familles(
                VGA_MEM_LOCATION,
                VgaCell { character: ' ' as u8, color: Into::<VgaColor>::into(AnsiColor::WHITE).0 },
                WIDTH * HEIGHT,
            );
        }
    }

    fn clear_cursor(&mut self, _c: char, _position: Pos, _color: AnsiColor) -> IoResult {
        // Disable cursor
        Pio::<u8>::new(Self::CURSOR_INDEX_REGISTER).write(0x0A);
        Pio::<u8>::new(Self::CURSOR_DATA_REGISTER).write(0x20);
        Ok(())
    }

    fn draw_cursor(&mut self, _c: char, position: Pos, _color: AnsiColor) -> IoResult {
        // set cursor position
        let absolute_pos: u16 = (position.column + position.line * WIDTH) as u16;

        Pio::<u8>::new(Self::CURSOR_INDEX_REGISTER).write(0x0F);
        Pio::<u8>::new(Self::CURSOR_DATA_REGISTER).write(absolute_pos as u8);

        Pio::<u8>::new(Self::CURSOR_INDEX_REGISTER).write(0x0E);
        Pio::<u8>::new(Self::CURSOR_DATA_REGISTER).write(((absolute_pos >> 8) & 0xff) as u8);

        // Enable cursor
        Pio::<u8>::new(Self::CURSOR_INDEX_REGISTER).write(0x0A); // start scanline
        Pio::<u8>::new(Self::CURSOR_DATA_REGISTER).write((Pio::<u8>::new(0x3D5).read()) & 0xC0 | 0);

        Pio::<u8>::new(Self::CURSOR_INDEX_REGISTER).write(0x0B); // end scanline
        Pio::<u8>::new(Self::CURSOR_DATA_REGISTER).write((Pio::<u8>::new(0x3D5).read()) & 0xE0 | 15);
        Ok(())
    }
}
#[derive(Debug, Copy, Clone)]
pub struct VgaColor(pub u8);

impl From<StandardColor> for VgaColor {
    fn from(c: StandardColor) -> Self {
        use StandardColor::*;
        match c {
            //TODO: Where to find the code ?
            Black => VgaColor(0),
            Red => VgaColor(4),
            Green => VgaColor(10),
            Blue => VgaColor(11),
            Yellow => VgaColor(14),
            Cyan => VgaColor(3),
            Magenta => VgaColor(13),
            White => VgaColor(7),
        }
    }
}

impl From<AnsiColor> for VgaColor {
    fn from(c: AnsiColor) -> Self {
        match c {
            // Convert only the 8 Standard Ansi color
            AnsiColor::Standard(c) => c.into(),
            // Otherwise set default to white
            _ => VgaColor(7),
        }
    }
}

#[repr(C)]
struct VgaCell {
    character: u8,
    color: u8,
}

extern "C" {
    fn _screencpy_des_familles(dst: *mut u8, pattern: VgaCell, nb_copies: usize);
}
