//! Kernel tty manager

#![deny(missing_docs)]

#[macro_use]
mod macros;

pub mod ansi_escape_code;

pub mod early_terminal;
pub use early_terminal::EARLY_TERMINAL;

pub mod cursor;
pub use cursor::{Cursor, Pos};

pub mod monitor;

mod tty;
pub use tty::{BufferedTty, Scroll, Tty, WriteMode};

mod log;

use self::monitor::SCREEN_MONAD;
use self::monitor::{bmp_loader, bmp_loader::BmpImage};

use crate::terminal::monitor::{AdvancedGraphic, Drawer};
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::Write;
use keyboard::keysymb::KeySymb;
use keyboard::{CallbackKeyboard, KEYBOARD_DRIVER};

/// Main structure of the terminal center
#[derive(Debug, Clone)]
pub struct Terminal {
    buf: Option<KeySymb>,
    ttys: Vec<BufferedTty>,
}

/// No initialized at the beginning
pub static mut TERMINAL: Option<Terminal> = None;

const MAX_SCREEN_BUFFER: usize = 10;

impl Terminal {
    /// Construct all the TTY
    pub fn new() -> Self {
        let size = SCREEN_MONAD.lock().query_window_size();
        Self {
            buf: None,
            ttys: vec![BufferedTty::new(Tty::new(false, size.line, size.column, MAX_SCREEN_BUFFER, None)); 2],
        }
    }

    fn switch_foreground_tty(&mut self, new_foreground_tty: usize) {
        self.ttys.iter_mut().find(|btty| btty.tty.foreground).map(|btty| btty.tty.foreground = false);
        self.ttys[new_foreground_tty].tty.foreground = true;
        self.ttys[new_foreground_tty].tty.refresh();
    }

    /// Get the foregounded TTY
    pub fn get_foreground_tty(&mut self) -> Option<&mut BufferedTty> {
        self.ttys.iter_mut().find(|btty| btty.tty.foreground)
    }

    fn handle_macros(&mut self) {
        match self.buf {
            Some(KeySymb::F1) => self.switch_foreground_tty(1),
            Some(KeySymb::F2) => self.switch_foreground_tty(0),
            Some(KeySymb::Control_p) => self.get_foreground_tty().unwrap().tty.scroll(Scroll::Up),
            Some(KeySymb::Control_n) => self.get_foreground_tty().unwrap().tty.scroll(Scroll::Down),
            Some(KeySymb::Control_b) => self.get_foreground_tty().unwrap().tty.scroll(Scroll::HalfScreenUp),
            Some(KeySymb::Control_d) => self.get_foreground_tty().unwrap().tty.scroll(Scroll::HalfScreenDown),
            _ => {
                return;
            }
        };
        self.buf = None;
    }

    fn stock_keysymb(&mut self, keysymb: KeySymb) {
        self.buf = Some(keysymb);
    }

    /// Read a Key from the buffer and throw it to the foreground TTY
    pub fn read(&mut self, buf: &mut [KeySymb], tty: usize) -> usize {
        self.handle_macros();
        if !self.ttys[tty].tty.foreground {
            return 0;
        }
        if let Some(key) = self.buf {
            buf[0] = key;
            self.buf = None;
            return 1;
        }
        return 0;
    }

    /// Write a string th the designed TTY
    pub fn write_str(&mut self, fd: usize, s: &str) {
        self.ttys[fd].write_str(s).unwrap();
    }

    /// Get the TTY n
    pub fn get_tty(&mut self, fd: usize) -> &mut BufferedTty {
        &mut self.ttys[fd]
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
    let mut term = Terminal::new();
    term.get_tty(1).tty.cursor.visible = true;

    term.switch_foreground_tty(1);

    unsafe {
        TERMINAL = Some(term);
    }

    let screen_monad = SCREEN_MONAD.lock();
    if screen_monad.is_graphic() {
        let (height, width, bpp) = screen_monad.query_graphic_infos().unwrap();
        let size = width * height * bpp / 8;

        let mut v: Vec<u8> = vec![42; size];
        bmp_loader::draw_image(unsafe { &_wanggle_bmp_start }, v.as_mut_ptr(), width, height, bpp).unwrap();
        unsafe {
            TERMINAL.as_mut().unwrap().get_tty(1).tty.set_background_buffer(v);
        }

        let mut v: Vec<u8> = vec![84; size];
        bmp_loader::draw_image(unsafe { &_univers_bmp_start }, v.as_mut_ptr(), width, height, bpp).unwrap();
        unsafe {
            TERMINAL.as_mut().unwrap().get_tty(0).tty.set_background_buffer(v);
        }
    }

    // unlock mutex
    drop(screen_monad);

    unsafe {
        TERMINAL.as_mut().unwrap().get_foreground_tty().unwrap().tty.refresh();
        KEYBOARD_DRIVER.as_mut().unwrap().bind(CallbackKeyboard::RequestKeySymb(stock_keysymb));
    }
    self::log::init().unwrap();
    ::log::info!("Terminal has been initialized");
}
