//! This file contains all the stuff about TTY

use super::InodeId;
use super::SysResult;
use super::{get_file_op_uid, Driver, FileOperation, IpcResult};

use alloc::sync::Arc;
use fallible_collections::FallibleArc;
use libc_binding::{termios, OpenFlags, Pid};
use sync::dead_mutex::DeadMutex;

use crate::terminal::{ReadResult, TERMINAL};

/// This structure represents a FileOperation of type TtyFileOperation
#[derive(Debug, Default)]
pub struct TtyFileOperation {
    controlling_terminal: usize,
    file_op_uid: usize,
    inode_id: InodeId,
}

/// Main implementation of TtyFileOperation
impl TtyFileOperation {
    pub fn new(controlling_terminal: usize, inode_id: InodeId) -> Self {
        let file_op_uid = get_file_op_uid();
        Self {
            controlling_terminal,
            file_op_uid,
            inode_id,
        }
    }
}

/// Taken directly from the rust std Utf8Error doc.
fn from_utf8_lossy<F>(mut input: &[u8], mut push: F)
where
    F: FnMut(&str),
{
    loop {
        match ::core::str::from_utf8(input) {
            Ok(valid) => {
                push(valid);
                break;
            }
            Err(error) => {
                let (valid, after_valid) = input.split_at(error.valid_up_to());
                let to_push =
                    ::core::str::from_utf8(valid).expect("Valid str from res should be valid...");
                push(to_push);
                // push("\u{FFFD}");

                if let Some(invalid_sequence_length) = error.error_len() {
                    input = &after_valid[invalid_sequence_length..]
                } else {
                    break;
                }
            }
        }
    }
}

/// Main Trait implementation of TtyFileOperation
impl FileOperation for TtyFileOperation {
    fn register(&mut self, _flags: OpenFlags) {}
    fn unregister(&mut self, _flags: OpenFlags) {}

    fn get_inode_id(&self) -> SysResult<InodeId> {
        Ok(self.inode_id)
    }

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
        // This skips invalid utf8 sequences.
        from_utf8_lossy(buf, |to_print| {
            print_tty!(self.controlling_terminal, "{}", to_print)
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
    /// A Tty got just one FileOperation structure which share with all
    operation: Arc<DeadMutex<TtyFileOperation>>,
}

/// Main implementation of TtyDevice
impl TtyDevice {
    pub fn try_new(controlling_terminal: usize, inode_id: InodeId) -> SysResult<Self> {
        let r = Ok(Self {
            operation: Arc::try_new(DeadMutex::new(TtyFileOperation::new(
                controlling_terminal,
                inode_id,
            )))?,
        });
        log::info!("TTY {} created !", controlling_terminal);
        r
    }
}

/// Driver trait implementation of TtyDevice
impl Driver for TtyDevice {
    fn open(
        &mut self,
        _flags: OpenFlags,
    ) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>> {
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
