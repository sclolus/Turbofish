//! Kernel tty manager
#![cfg_attr(all(not(test), not(feature = "std-print")), no_std)]

extern crate alloc;

#[macro_use]
mod macros;

pub mod ansi_escape_code;

pub mod early_terminal;
pub use early_terminal::EARLY_TERMINAL;

pub mod cursor;
pub use cursor::{Cursor, Pos};

pub mod monitor;

mod tty;
pub use tty::{BufferedTty, LineDiscipline, ReadResult, Scroll, Tty, WriteMode};

mod log;

#[macro_use]
pub mod uart_16550;
pub use uart_16550::UART_16550;

use self::monitor::SCREEN_MONAD;
use self::monitor::{bmp_loader, bmp_loader::BmpImage};

use crate::monitor::{AdvancedGraphic, Drawer};
use alloc::collections::VecDeque;
use alloc::vec;
use alloc::vec::Vec;
use keyboard::keysymb::KeySymb;

/// Main structure of the terminal center
#[derive(Debug, Clone)]
pub struct Terminal {
    buf: VecDeque<KeySymb>,
    ttys: Vec<LineDiscipline>,
}

/// No initialized at the beginning
pub static mut TERMINAL: Option<Terminal> = None;

const MAX_SCREEN_BUFFER: usize = 10;

impl Terminal {
    /// Construct all the TTY
    pub fn new() -> Self {
        let size = SCREEN_MONAD.lock().query_window_size();
        Self {
            buf: VecDeque::new(),
            // do not create a vec directly because BufferedTty::new() as side efect of chosing capacity of buffer
            ttys: (0..2)
                .map(|_| {
                    LineDiscipline::new(BufferedTty::new(Tty::new(
                        false,
                        size.line,
                        size.column,
                        MAX_SCREEN_BUFFER,
                        None,
                    )))
                })
                .collect(),
        }
    }

    fn switch_foreground_tty(&mut self, new_foreground_tty: usize) {
        self.ttys
            .iter_mut()
            .find(|l| l.get_tty().foreground)
            .map(|l| l.get_tty_mut().foreground = false);
        let new_tty = self.ttys[new_foreground_tty].get_tty_mut();
        new_tty.foreground = true;
        new_tty.refresh();
    }

    /// Get the foregounded TTY
    pub fn get_foreground_tty(&mut self) -> Option<&mut BufferedTty> {
        Some(&mut self.ttys.iter_mut().find(|l| l.get_tty().foreground)?.tty)
    }

    /// Read a Key from the buffer
    pub fn read(&mut self, buf: &mut [u8], tty_index: usize) -> ReadResult {
        self.ttys[tty_index].read(buf)
    }

    // /// Write a string th the designed TTY
    // pub fn write_str(&mut self, fd: usize, s: &str) {
    //     self.ttys[fd].write_str(s).unwrap();
    // }

    pub fn write_input(&mut self, buff: &[KeySymb], tty_index: usize) {
        // eprintln!("write_input {:?}", buff);
        if !self.handle_tty_control(buff[0]) {
            self.ttys[tty_index]
                .write_input(buff)
                //TODO: remove this expect later
                .expect("write input failed");
        }
    }

    /// Get the TTY n
    pub fn get_tty(&mut self, fd: usize) -> &mut BufferedTty {
        &mut self.ttys[fd].tty
    }

    pub fn get_line_discipline(&mut self, fd: usize) -> &mut LineDiscipline {
        &mut self.ttys[fd]
    }
    /// Provide a tiny interface to sontrol some features on the tty
    pub fn handle_tty_control(&mut self, keysymb: KeySymb) -> bool {
        match keysymb {
            KeySymb::F1 => self.switch_foreground_tty(1),
            KeySymb::F2 => self.switch_foreground_tty(0),
            _ => {
                return false;
            }
        };
        true
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

    let screen_monad = SCREEN_MONAD.lock();
    if screen_monad.is_graphic() {
        let (height, width, bpp) = screen_monad.query_graphic_infos().unwrap();
        let size = width * height * bpp / 8;

        let mut v: Vec<u8> = vec![42; size];
        // bmp_loader::draw_image(unsafe { &_wanggle_bmp_start }, v.as_mut_ptr(), width, height, bpp).unwrap();
        bmp_loader::draw_image(
            unsafe { &_univers_bmp_start },
            v.as_mut_ptr(),
            width,
            height,
            bpp,
        )
        .unwrap();
        term.get_tty(1).tty.set_background_buffer(v);

        let mut v: Vec<u8> = vec![84; size];
        bmp_loader::draw_image(
            unsafe { &_univers_bmp_start },
            v.as_mut_ptr(),
            width,
            height,
            bpp,
        )
        .unwrap();
        term.get_tty(0).tty.set_background_buffer(v);
    }

    // unlock mutex
    drop(screen_monad);

    term.get_foreground_tty().unwrap().tty.refresh();
    unsafe {
        TERMINAL = Some(term);
    }
    self::log::init().unwrap();
    ::log::info!("Terminal has been initialized");
}
