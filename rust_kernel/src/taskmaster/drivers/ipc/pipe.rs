//! This file contains all the stuff about Pipes

use super::SysResult;

use super::FileOperation;
use super::IpcResult;

use super::get_file_op_uid;

use libc_binding::{Errno, OpenFlags};

use core::cmp;

use messaging::MessageTo;

struct Buf([u8; Self::BUF_SIZE]);

/// Deref boilerplate for Buf
impl core::ops::Deref for Buf {
    type Target = [u8; Self::BUF_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// DerefMut boilerplate for Buf
impl core::ops::DerefMut for Buf {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Default boilerplate for Buf
impl Default for Buf {
    fn default() -> Self {
        Self {
            0: [0; Self::BUF_SIZE],
        }
    }
}

/// Debug boilerplate for Buf
impl core::fmt::Debug for Buf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Hidden Buf content")
    }
}

/// Buf implementation
impl Buf {
    const BUF_SIZE: usize = 128;
}

/// This structure represents a FileOperation of type Pipe
#[derive(Debug, Default)]
pub struct Pipe {
    buf: Buf,
    input_ref: usize,
    output_ref: usize,
    current_index: usize,
    file_op_uid: usize,
}

/// Main implementation for Pipe
impl Pipe {
    pub fn new() -> Self {
        let mut pipe = Pipe::default();
        pipe.file_op_uid = get_file_op_uid();
        pipe
    }
}

/// Main Trait implementation
impl FileOperation for Pipe {
    fn register(&mut self, flags: OpenFlags) {
        if flags.contains(OpenFlags::O_RDONLY) {
            self.input_ref += 1;
        } else if flags.contains(OpenFlags::O_WRONLY) {
            self.output_ref += 1;
        } else {
            panic!("Pipe invalid access mode");
        }
    }
    fn unregister(&mut self, flags: OpenFlags) {
        if flags.contains(OpenFlags::O_RDONLY) {
            self.input_ref -= 1;
            // Announce to writer(s) that the last reader is gone
            if self.input_ref == 0 {
                unsafe {
                    messaging::send_message(MessageTo::Writer {
                        uid_file_op: self.file_op_uid,
                    });
                }
            }
        } else if flags.contains(OpenFlags::O_WRONLY) {
            self.output_ref -= 1;
            // Announce to reader(s) that the last writer is gone
            if self.output_ref == 0 {
                unsafe {
                    messaging::send_message(MessageTo::Reader {
                        uid_file_op: self.file_op_uid,
                    });
                }
            }
        } else {
            panic!("Pipe invalid access mode");
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

        let min = cmp::min(buf.len(), Buf::BUF_SIZE - self.current_index);

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
