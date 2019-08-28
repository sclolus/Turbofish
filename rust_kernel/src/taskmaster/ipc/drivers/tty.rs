//! This file contains all the stuff about TTY

use super::SysResult;

use super::IpcResult;
use super::Mode;

use super::{get_file_op_uid, Driver, FileOperation};

use alloc::sync::Arc;
use fallible_collections::FallibleArc;
use libc_binding::{termios, Pid};
use sync::dead_mutex::DeadMutex;

use crate::terminal::{ReadResult, TERMINAL};

/// This structure represents a FileOperation of type TtyFileOperation
#[derive(Debug, Default)]
pub struct TtyFileOperation {
    controlling_terminal: usize,
    file_op_uid: usize,
}

/// Main implementation of TtyFileOperation
impl TtyFileOperation {
    pub fn new(controlling_terminal: usize) -> Self {
        let file_op_uid = get_file_op_uid();
        Self {
            controlling_terminal,
            file_op_uid,
        }
    }
}

/// Main Trait implementation of TtyFileOperation
impl FileOperation for TtyFileOperation {
    fn register(&mut self, _access_mode: Mode) {}
    fn unregister(&mut self, _access_mode: Mode) {}
    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        let read_result = unsafe {
            TERMINAL
                .as_mut()
                .unwrap()
                .read(buf, self.controlling_terminal)
        };

        match read_result {
            ReadResult::NonBlocking(read_count) => Ok(IpcResult::Done(read_count as _)),
            // Apply a local terminal rule: A blocked call cannot have character
            ReadResult::Blocking => Ok(IpcResult::Wait(0, self.file_op_uid)),
        }
    }
    fn write(&mut self, buf: &[u8]) -> SysResult<IpcResult<u32>> {
        print_tty!(self.controlling_terminal, "{}", unsafe {
            core::str::from_utf8_unchecked(buf)
        });
        Ok(IpcResult::Done(buf.len() as _))
    }
    fn tcgetattr(&self, termios_p: &mut termios) -> SysResult<u32> {
        unsafe {
            TERMINAL
                .as_mut()
                .unwrap()
                .get_line_discipline(self.controlling_terminal)
                .tcgetattr(termios_p);
        }
        Ok(0)
    }
    fn tcsetattr(&mut self, optional_actions: u32, termios_p: &termios) -> SysResult<u32> {
        unsafe {
            TERMINAL
                .as_mut()
                .unwrap()
                .get_line_discipline(self.controlling_terminal)
                .tcsetattr(optional_actions, termios_p);
        }
        Ok(0)
    }
    fn tcgetpgrp(&self) -> SysResult<Pid> {
        unsafe {
            Ok(TERMINAL
                .as_mut()
                .unwrap()
                .get_line_discipline(self.controlling_terminal)
                .tcgetpgrp())
        }
    }
    fn tcsetpgrp(&mut self, pgid_id: Pid) -> SysResult<u32> {
        unsafe {
            TERMINAL
                .as_mut()
                .unwrap()
                .get_line_discipline(self.controlling_terminal)
                .tcsetpgrp(pgid_id);
        }
        Ok(0)
    }
    fn isatty(&mut self) -> SysResult<u32> {
        return Ok(1);
    }
}

/// Drop boilerplate
impl Drop for TtyFileOperation {
    fn drop(&mut self) {
        log::info!("TTY {} file operation droped !", self.controlling_terminal);
    }
}

/// Stucture of TtyDevice
#[derive(Debug)]
pub struct TtyDevice {
    /// Refer ta an 'father' inode
    inode_id: Option<usize>,
    /// A Tty got just one FileOperation structure which share with all
    operation: Arc<DeadMutex<TtyFileOperation>>,
}

/// Main implementation of TtyDevice
impl TtyDevice {
    pub fn try_new(controlling_terminal: usize) -> SysResult<Self> {
        let r = Ok(Self {
            inode_id: None,
            operation: Arc::try_new(DeadMutex::new(TtyFileOperation::new(controlling_terminal)))?,
        });
        log::info!("TTY {} created !", controlling_terminal);
        r
    }
}

/// Driver trait implementation of TtyDevice
impl Driver for TtyDevice {
    fn open(&mut self) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>> {
        let controlling_terminal = self.operation.lock().controlling_terminal;
        let file_op_uid = self.operation.lock().file_op_uid;
        unsafe {
            TERMINAL
                .as_mut()
                .unwrap()
                .open(controlling_terminal, file_op_uid);
        }
        log::info!("TTY {} opened !", controlling_terminal);
        Ok(IpcResult::Done(self.operation.clone()))
    }
    fn set_inode_id(&mut self, inode_id: usize) {
        self.inode_id = Some(inode_id);
    }
}

/// Drop boilerplate
impl Drop for TtyDevice {
    fn drop(&mut self) {
        log::info!(
            "TTY {} droped !",
            self.operation.lock().controlling_terminal
        );
    }
}
