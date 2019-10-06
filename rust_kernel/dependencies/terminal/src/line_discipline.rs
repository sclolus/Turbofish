use super::tty::{BufferedTty, Scroll, Tty};
use ansi_escape_code::CursorMove;
use arrayvec::{ArrayVec, CapacityError};
use core::cmp::min;
use core::convert::TryFrom;
use core::fmt::Write;
use keyboard::{KeySymb, ScanCode};
use libc_binding::{termios, Pid, Signum, ECHO, ICANON, ISIG, TOSTOP};
use libc_binding::{VEOF, VERASE, VINTR, VKILL, VQUIT, VSUSP};
use messaging::{MessageTo, ProcessGroupMessage};

// extern "C" {
//     fn get_current_pgid() -> Pid;
// }
// use libc_binding::{VEOL, VMIN, VSTART, VSTOP, VTIME};

#[derive(Debug, Clone)]
pub struct LineDiscipline {
    pub tty: BufferedTty,
    termios: termios,
    read_buffer: ArrayVec<[u8; 4096]>,
    foreground_process_group: Pid,
    end_of_file_set: bool,
    /// raw mode doesn't transform scancode in utf8
    is_raw_mode: bool,
}

/// represent a result of the read function to handle the blocking
/// case
pub enum ReadResult {
    /// the syscall will be blocking
    Blocking,
    /// the syscall is non blocking even if the return is 0
    NonBlocking(usize),
}

impl LineDiscipline {
    pub fn new(tty: BufferedTty) -> Self {
        Self {
            termios: termios {
                c_iflag: 0,
                c_oflag: 0,
                c_cflag: 0,
                c_lflag: (ECHO | ICANON | ISIG | TOSTOP),
                c_cc: [
                    /*VEOF  */ KeySymb::Control_d as u32,
                    /*VEOL  */ KeySymb::Return as u32,
                    /*VERASE*/ KeySymb::Delete as u32,
                    /*VINTR */ KeySymb::Control_c as u32,
                    /*VKILL */ KeySymb::Control_u as u32,
                    /*VMIN  */ KeySymb::Linefeed as u32,
                    /*VQUIT */ KeySymb::Control_backslash as u32,
                    /*VSUSP */ KeySymb::Control_z as u32,
                    /*VTIME */ KeySymb::nul as u32,
                    /*VSTART*/ KeySymb::nul as u32,
                    /*VSTOP */ KeySymb::nul as u32,
                ],
            },
            tty,
            read_buffer: ArrayVec::new(),
            foreground_process_group: 0,
            end_of_file_set: false,
            is_raw_mode: false,
        }
    }

    /// handle directly some keysymb to control the terminal
    pub fn handle_tty_control(&mut self, keysymb: KeySymb) -> bool {
        match keysymb {
            KeySymb::Control_p => self.tty.as_mut().scroll(Scroll::Up),
            KeySymb::Control_n => self.tty.as_mut().scroll(Scroll::Down),
            KeySymb::Control_b => self.tty.as_mut().scroll(Scroll::HalfScreenUp),
            KeySymb::Control_v => self.tty.as_mut().scroll(Scroll::HalfScreenDown),
            _ => {
                return false;
            }
        };
        true
    }

    pub fn handle_scancode(&mut self, scancode: ScanCode) -> Result<(), CapacityError<u8>> {
        if self.is_raw_mode {
            // let keycode = KeyCode::from_scancode(scancode);
            // dbg!(keycode);
            self.read_buffer.try_push((scancode & 0xff) as u8)?;
            self.read_buffer
                .try_push(((scancode & 0xff00) >> 8) as u8)?;
        }
        Ok(())
    }
    /// write in the read buffer the keysymb read from the keyboard
    /// Send a message if read is ready, depending of the lmode
    pub fn handle_key_pressed(&mut self, key: KeySymb) -> Result<(), CapacityError<u8>> {
        if !self.handle_tty_control(key) {
            // Check if tty is attached to a file operator
            if self.tty.uid_file_op.is_none() {
                return Ok(());
            }
            let mut encode_buff = [0; 8];
            // handle special keys in canonical mode
            if self.termios.c_lflag & ICANON != 0 {
                // handle delete key
                if key as u32 == self.termios.c_cc[VERASE as usize] {
                    if self.read_buffer.pop().is_some() {
                        self.tty.as_mut().move_cursor(CursorMove::Backward(1));
                        self.tty.write_char(' ').unwrap();
                        self.tty.as_mut().move_cursor(CursorMove::Backward(1));
                    }
                    return Ok(());;
                }
                // dbg!(key);
                // handle kill key
                if key as u32 == self.termios.c_cc[VKILL as usize] {
                    self.tty
                        .as_mut()
                        .move_cursor(CursorMove::HorizontalAbsolute(0));
                    if let Some(index) = self.read_buffer.iter().position(|c| *c == '\n' as u8) {
                        self.read_buffer.truncate(index);
                    } else {
                        self.read_buffer.clear();
                    }

                    for _ in 0..self.tty.as_mut().cursor.nb_columns - 1 {
                        self.tty
                            .as_mut()
                            .write_char(' ')
                            .expect("failed to write \0");
                    }
                    self.tty
                        .as_mut()
                        .move_cursor(CursorMove::HorizontalAbsolute(0));

                    return Ok(());;
                }
                if key as u32 == self.termios.c_cc[VEOF as usize] {
                    self.end_of_file_set = true;
                    unsafe {
                        messaging::send_message(MessageTo::Reader {
                            uid_file_op: self.tty.uid_file_op.expect("no FileOperation registered"),
                        });;
                    }
                    return Ok(());;
                }
            }
            if self.termios.c_lflag & ISIG != 0 {
                // handle control_c
                if key as u32 == self.termios.c_cc[VINTR as usize] {
                    unsafe {
                        messaging::send_message(MessageTo::ProcessGroup {
                            pgid: self.foreground_process_group,
                            content: ProcessGroupMessage::Signal(Signum::SIGINT),
                        });
                    }
                    return Ok(());;
                }
                if key as u32 == self.termios.c_cc[VSUSP as usize] {
                    unsafe {
                        messaging::send_message(MessageTo::ProcessGroup {
                            pgid: self.foreground_process_group,
                            content: ProcessGroupMessage::Signal(Signum::SIGTSTP),
                        });
                    }
                    return Ok(());;
                }
                if key as u32 == self.termios.c_cc[VQUIT as usize] {
                    unsafe {
                        messaging::send_message(MessageTo::ProcessGroup {
                            pgid: self.foreground_process_group,
                            content: ProcessGroupMessage::Signal(Signum::SIGQUIT),
                        });
                    }
                    return Ok(());;
                }
            }
            /* PUSH THE KEY */
            let b = encode_utf8(key, &mut encode_buff);
            // dbg!((key as i32 & 0xff00) >> 8);
            // dbg!(key as i32 & 0xff);
            // dbg!(&b);
            for elem in b {
                // dbg!(&b);
                self.read_buffer.try_push(*elem)?;
            }
            if (self.termios.c_lflag & ICANON != 0 && key == KeySymb::Return)
                || !self.termios.c_lflag & ICANON != 0
            {
                unsafe {
                    messaging::send_message(MessageTo::Reader {
                        uid_file_op: self.tty.uid_file_op.expect("no FileOperation registered"),
                    });;
                }
            }
            if self.termios.c_lflag & ECHO != 0 {
                self.write(b);
            }
        }
        Ok(())
    }

    /// read maximum `max_len_data_to_read` on the read_buffer
    fn read_max(&mut self, output: &mut [u8], max_len_data_to_read: usize) -> usize {
        let len_data_to_read = min(max_len_data_to_read, output.len());
        for (dest, src) in output
            .iter_mut()
            .zip(self.read_buffer.drain(0..len_data_to_read))
        {
            *dest = src;
        }
        len_data_to_read
    }

    /// open a tty
    pub fn open(&mut self, uid_file_op: usize) {
        self.tty.uid_file_op = Some(uid_file_op);
    }

    /// read (from a process) on the tty
    /// return the number of bytes readen
    pub fn read(&mut self, output: &mut [u8]) -> ReadResult {
        use ReadResult::*;
        // print!("read buffer: ");
        // for c in &self.read_buffer {
        //     print!("{}", *c as char);
        // }
        // print!("\n");
        // Any attempts by a process in a background process group to
        // read from its controlling terminal cause its process group
        // to be sent a SIGTTIN signal unless one of the following
        // special cases applies: if the reading process is ignoring
        // the SIGTTIN signal or the reading thread is blocking the
        // SIGTTIN signal, or if the process group of the reading
        // process is orphaned, the read() shall return -1, with errno
        // set to [EIO] and no signal shall be sent. The default
        // action of the SIGTTIN signal shall be to stop the process
        // to which it is sent. See <signal.h>.
        // let current_pgid = unsafe { get_current_pgid() };
        // if self.foreground_process_group != current_pgid && (self.termios.c_lflag & TOSTOP != 0) {
        //     messaging::send_message(MessageTo::ProcessGroup {
        //         pgid: self.foreground_process_group,
        //         content: ProcessGroupMessage::Signal(Signum::SIGTTIN),
        //     });
        // }
        if self.termios.c_lflag & ICANON != 0 {
            // dbg!("canonical");
            // if VEOF was pressed, read all
            if self.end_of_file_set {
                self.end_of_file_set = false;
                NonBlocking(self.read_max(output, self.read_buffer.len()))
            } else if let Some(index) = self.read_buffer.iter().position(|c| *c == '\n' as u8) {
                //  In canonical mode, we only read until the '\n' and at most output.len bytes
                NonBlocking(self.read_max(output, index + 1))
            } else {
                Blocking
            }
        } else if self.read_buffer.len() != 0 {
            NonBlocking(self.read_max(output, self.read_buffer.len()))
        } else {
            Blocking
        }
    }

    /// write on the tty
    pub fn write(&mut self, s: &[u8]) -> usize {
        //Attempts by a process in a background process group to write
        // to its controlling terminal shall cause the process group
        // to be sent a SIGTTOU signal unless one of the following
        // special cases applies: if TOSTOP is not set, or if TOSTOP
        // is set and the process is ignoring the SIGTTOU signal or
        // the writing thread is blocking the SIGTTOU signal, the
        // process is allowed to write to the terminal and the SIGTTOU
        // signal is not sent. If TOSTOP is set, the process group of
        // the writing process is orphaned, the writing process is not
        // ignoring the SIGTTOU signal, and the writing thread is not
        // blocking the SIGTTOU signal, the write() shall return -1,
        // with errno set to [EIO] and no signal shall be sent.
        let s = core::str::from_utf8(s).expect("bad utf8");
        // let current_pgid = unsafe { get_current_pgid() };
        // if self.foreground_process_group != current_pgid && (self.termios.c_lflag & TOSTOP != 0) {
        //     messaging::send_message(MessageTo::ProcessGroup {
        //         pgid: self.foreground_process_group,
        //         content: ProcessGroupMessage::Signal(Signum::SIGTTOU),
        //     });
        // }

        self.tty.write_str(s).expect("write failed");
        s.len()
    }
    pub fn set_raw_mode(&mut self, val: bool) {
        self.is_raw_mode = val;
    }

    pub fn is_raw_mode(&self) -> bool {
        self.is_raw_mode
    }

    pub fn get_tty(&self) -> &Tty {
        &self.tty.tty
    }
    pub fn get_tty_mut(&mut self) -> &mut Tty {
        &mut self.tty.tty
    }
    pub fn tcsetattr(&mut self, _optional_actions: u32, termios_p: &termios) {
        // dbg!(self.termios.c_lflag);
        self.termios = *termios_p;
        // dbg!(self.termios.c_lflag);
    }
    pub fn tcgetattr(&mut self, termios_p: &mut termios) {
        *termios_p = self.termios;
    }
    pub fn tcsetpgrp(&mut self, pgid_id: Pid) {
        // dbg!(self.termios.c_lflag);
        self.foreground_process_group = pgid_id;
        // dbg!(&self.foreground_process_group);
        // dbg!(self.termios.c_lflag);
    }
    pub fn tcgetpgrp(&mut self) -> Pid {
        self.foreground_process_group
    }
}

pub fn encode_utf8(keysymb: KeySymb, dst: &mut [u8]) -> &[u8] {
    // particular case of convertion
    match keysymb {
        KeySymb::Right => {
            dst[0] = 27;
            dst[1] = 79;
            dst[2] = 67;
            return &dst[0..3];
        }
        KeySymb::Left => {
            dst[0] = 27;
            dst[1] = 79;
            dst[2] = 68;
            return &dst[0..3];
        }
        KeySymb::Up => {
            dst[0] = 27;
            dst[1] = 79;
            dst[2] = 65;
            return &dst[0..3];
        }
        KeySymb::Down => {
            dst[0] = 27;
            dst[1] = 79;
            dst[2] = 66;
            return &dst[0..3];
        }
        KeySymb::Delete => {
            dst[0] = 127;
            return &dst[0..1];
        }
        KeySymb::Return => {
            dst[0] = '\n' as u8;
            return &dst[0..1];
        }
        _ => {
            let c = char::try_from(keysymb as u32).unwrap();
            // GAFFE: Je fix vite fait les problemes de carateres fantomes et bullshit !
            // Mais en considerant les caracteres de plus de 2 bytes comme de simples espaces,
            // j'inhibe toutes possibilite de faire fonctionner l'UTF8.
            // Si on voulait qu'il soit correctement gere, Le read_buffer devrait etre une
            // 'string' de capacite finie et des caracteres par defaut devrait s'afficher a la place de tout UTF8
            // dont on aurait pas la police.
            // Des modifications dans les buffers de terminal devraient etre operees aussi.
            if c.len_utf8() == 1 {
                c.encode_utf8(dst).as_bytes()
            } else {
                dst[0] = ' ' as u8;
                return &dst[0..1];
            }
        }
    }
}
