//! This file contains all the stuff about Pipes

use super::SysResult;

use super::FileOperation;
use super::IpcResult;
use super::Mode;

use super::get_file_op_uid;

use libc_binding::Errno;

use core::cmp;

const BUF_SIZE: usize = 16;

use messaging::MessageTo;

/// This structure represents a FileOperation of type Pipe
#[derive(Debug, Default)]
pub struct Pipe {
    input_ref: usize,
    output_ref: usize,
    buf: [u8; BUF_SIZE],
    current_index: usize,
    file_op_uid: usize,
}

/// Main implementation for Pipe
impl Pipe {
    pub fn new() -> Self {
        Self {
            input_ref: Default::default(),
            output_ref: Default::default(),
            buf: [0; BUF_SIZE],
            current_index: Default::default(),
            file_op_uid: get_file_op_uid(),
        }
    }
}

/// Main Trait implementation
impl FileOperation for Pipe {
    fn register(&mut self, access_mode: Mode) {
        match access_mode {
            Mode::ReadOnly => self.input_ref += 1,
            Mode::WriteOnly => self.output_ref += 1,
            _ => panic!("Pipe invalid access mode"),
        };
    }
    fn unregister(&mut self, access_mode: Mode) {
        match access_mode {
            Mode::ReadOnly => {
                self.input_ref -= 1;
                // Announce to writer(s) that the last reader is gone
                if self.input_ref == 0 {
                    unsafe {
                        messaging::send_message(MessageTo::Writer {
                            uid_file_op: self.file_op_uid,
                        });
                    }
                }
            }
            Mode::WriteOnly => {
                self.output_ref -= 1;
                // Announce to reader(s) that the last writer is gone
                if self.output_ref == 0 {
                    unsafe {
                        messaging::send_message(MessageTo::Reader {
                            uid_file_op: self.file_op_uid,
                        });
                    }
                }
            }
            _ => panic!("Pipe invalid access mode"),
        };
    }
    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        if self.current_index == 0 {
            if self.output_ref == 0 {
                // Writers are gone, returns immediatly
                return Ok(IpcResult::Done(0));
            } else {
                // Waiting for a writer
                return Ok(IpcResult::Wait(0, self.file_op_uid));
            }
        }

        let min = cmp::min(buf.len(), self.current_index);

        // memcpy(buf, self.buf, MIN(buf.len(), self.current_index)
        buf[..min].copy_from_slice(&self.buf[..min]);

        // memcpy(self.buf, self.buf + MIN(buf.len(), self.current_index), self.current_index - MIN(buf.len(), self.current_index))
        self.buf.copy_within(min..self.current_index, 0);
        self.current_index -= min;

        unsafe {
            messaging::send_message(MessageTo::Writer {
                uid_file_op: self.file_op_uid,
            });
        }
        Ok(IpcResult::Done(min as _))
    }
    fn write(&mut self, buf: &[u8]) -> SysResult<IpcResult<u32>> {
        // Readers are gone, returns immediatly
        if self.input_ref == 0 {
            return Err(Errno::EPIPE);
        }

        let min = cmp::min(buf.len(), BUF_SIZE - self.current_index);

        self.buf[self.current_index..self.current_index + min].copy_from_slice(&buf[..min]);
        self.current_index += min;

        // If the writer has writed something into the pipe...
        if min > 0 {
            unsafe {
                messaging::send_message(MessageTo::Reader {
                    uid_file_op: self.file_op_uid,
                });
            }
        }
        if min == buf.len() {
            Ok(IpcResult::Done(min as _))
        } else {
            Ok(IpcResult::Wait(min as _, self.file_op_uid))
        }
    }
}

/// Some boilerplate to check if all is okay
impl Drop for Pipe {
    fn drop(&mut self) {}
}
