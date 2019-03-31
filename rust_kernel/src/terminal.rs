#[macro_use]
pub mod macros;

pub mod ansi_escape_code;

pub mod early_terminal;
pub use early_terminal::EARLY_TERMINAL;

pub mod cursor;
pub use cursor::{Cursor, Pos};

pub mod monitor;
pub use self::monitor::Color;

mod tty;
pub use tty::{CursorDirection, Scroll, Tty, WriteMode};

mod log;

use self::monitor::SCREEN_MONAD;
use self::monitor::{bmp_loader, bmp_loader::BmpImage};

use crate::drivers::keyboard::keysymb::KeySymb;
use crate::drivers::keyboard::{CallbackKeyboard, KEYBOARD_DRIVER};
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::Write;

#[derive(Debug, Clone)]
pub struct Terminal {
    buf: Option<KeySymb>,
    ttys: Vec<Tty>,
}

pub static mut TERMINAL: Option<Terminal> = None;

const MAX_SCREEN_BUFFER: usize = 10;

impl Terminal {
    pub fn new() -> Self {
        let screen_monad = SCREEN_MONAD.lock();
        Self {
            buf: None,
            ttys: vec![Tty::new(false, screen_monad.nb_lines, screen_monad.nb_columns, MAX_SCREEN_BUFFER, None); 2],
        }
    }

    fn switch_foreground_tty(&mut self, new_foreground_tty: usize) {
        self.ttys.iter_mut().find(|tty| tty.foreground).map(|t| t.foreground = false);
        self.ttys[new_foreground_tty].foreground = true;
        self.ttys[new_foreground_tty].refresh();
    }

    pub fn get_foreground_tty(&mut self) -> Option<&mut Tty> {
        self.ttys.iter_mut().find(|tty| tty.foreground)
    }

    fn handle_macros(&mut self) {
        match self.buf {
            Some(KeySymb::F1) => self.switch_foreground_tty(1),
            Some(KeySymb::F2) => self.switch_foreground_tty(0),
            Some(KeySymb::Control_p) => self.get_foreground_tty().unwrap().scroll(Scroll::Up),
            Some(KeySymb::Control_n) => self.get_foreground_tty().unwrap().scroll(Scroll::Down),
            Some(KeySymb::Control_b) => self.get_foreground_tty().unwrap().scroll(Scroll::HalfScreenUp),
            Some(KeySymb::Control_d) => self.get_foreground_tty().unwrap().scroll(Scroll::HalfScreenDown),
            _ => {
                return;
            }
        };
        self.buf = None;
    }

    fn stock_keysymb(&mut self, keysymb: KeySymb) {
        self.buf = Some(keysymb);
    }

    pub fn read(&mut self, buf: &mut [KeySymb], tty: usize) -> usize {
        self.handle_macros();
        if !self.ttys[tty].foreground {
            return 0;
        }
        if let Some(key) = self.buf {
            buf[0] = key;
            self.buf = None;
            return 1;
        }
        return 0;
    }

    pub fn write_str(&mut self, fd: usize, s: &str) {
        self.ttys[fd].write_str(s).unwrap();
    }

    pub fn move_cursor(&mut self, direction: CursorDirection, q: usize) {
        self.get_foreground_tty().unwrap().move_cursor(direction, q)
    }

    pub fn get_tty(&mut self, fd: usize) -> &mut Tty {
        &mut self.ttys[fd]
    }

    pub fn set_text_color(&mut self, color: Color) {
        self.get_foreground_tty().unwrap().set_text_color(color);
    }
}

/// Usefull method to stock the character from the keyboard
pub fn stock_keysymb(keysymb: KeySymb) {
    unsafe {
        TERMINAL.as_mut().unwrap().stock_keysymb(keysymb);
    }
}

extern "C" {
    static _wanggle_bmp_start: BmpImage;
    static _univers_bmp_start: BmpImage;
}

/// Extern function for initialisation
pub fn init_terminal() {
    SCREEN_MONAD.lock().switch_graphic_mode(Some(0x118)).unwrap();
    let mut term = Terminal::new();
    term.get_tty(1).cursor.visible = true;

    term.switch_foreground_tty(1);

    unsafe {
        TERMINAL = Some(term);
    }

    let screen_monad = SCREEN_MONAD.lock();
    let size = screen_monad.width.unwrap() * screen_monad.height.unwrap() * screen_monad.bpp.unwrap() / 8;

    let mut v: Vec<u8> = vec![42; size];
    bmp_loader::draw_image(
        unsafe { &_wanggle_bmp_start },
        v.as_mut_ptr(),
        screen_monad.width.unwrap(),
        screen_monad.height.unwrap(),
        screen_monad.bpp.unwrap(),
    )
    .unwrap();
    unsafe {
        TERMINAL.as_mut().unwrap().get_tty(1).set_background_buffer(v);
    }

    let mut v: Vec<u8> = vec![84; size];
    bmp_loader::draw_image(
        unsafe { &_univers_bmp_start },
        v.as_mut_ptr(),
        screen_monad.width.unwrap(),
        screen_monad.height.unwrap(),
        screen_monad.bpp.unwrap(),
    )
    .unwrap();
    unsafe {
        TERMINAL.as_mut().unwrap().get_tty(0).set_background_buffer(v);
    }

    // unlock mutex
    drop(screen_monad);

    unsafe {
        TERMINAL.as_mut().unwrap().get_foreground_tty().unwrap().refresh();
        KEYBOARD_DRIVER.as_mut().unwrap().bind(CallbackKeyboard::RequestKeySymb(stock_keysymb));
    }
    self::log::init().unwrap();
    ::log::info!("Terminal has been initialized");
}
