//! This file contains all the stuff about Fifo

use super::SysResult;

use super::get_file_op_uid;
use super::pipe::Buf;
use super::VFS;
use super::{Driver, FileOperation, IpcResult};

use super::vfs::InodeId;

use alloc::sync::Arc;
use sync::DeadMutex;

use fallible_collections::arc::FallibleArc;
use libc_binding::{Errno, OpenFlags};

use core::cmp;

use messaging::MessageTo;

/// Stucture of FifoDriver
#[derive(Debug)]
pub struct FifoDriver {
    inode_id: InodeId,
    operation: Arc<DeadMutex<FifoFileOperation>>,
}

/// Main implementation of FifoDriver
impl FifoDriver {
    pub fn try_new(inode_id: InodeId) -> SysResult<Self> {
        log::info!("Fifo Driver created !");
        Ok(Self {
            inode_id,
            operation: Arc::try_new(DeadMutex::new(FifoFileOperation::new(inode_id)))?,
        })
    }
}

/// Driver trait implementation of FifoDriver
impl Driver for FifoDriver {
    fn open(
        &mut self,
        flags: OpenFlags,
    ) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>> {
        log::info!("Opening fifo");

        // dbg!(flags);
        // dbg!(&self.operation);
        if flags.contains(OpenFlags::O_RDONLY) {
            if self.operation.lock().output_ref == 0 {
                Ok(IpcResult::Wait(
                    self.operation.clone(),
                    self.operation.lock().file_op_uid,
                ))
            } else {
                Ok(IpcResult::Done(self.operation.clone()))
            }
        } else if flags.contains(OpenFlags::O_WRONLY) {
            if self.operation.lock().input_ref == 0 {
                Ok(IpcResult::Wait(
                    self.operation.clone(),
                    self.operation.lock().file_op_uid,
                ))
            } else {
                Ok(IpcResult::Done(self.operation.clone()))
            }
        } else {
            Err(Errno::EINVAL)
        }
    }
}

/// This structure represents a FileOperation of type Fifo
#[derive(Debug, Default)]
pub struct FifoFileOperation {
    inode_id: InodeId,
    buf: Buf,
    input_ref: usize,
    output_ref: usize,
    current_index: usize,
    file_op_uid: usize,
}

/// Main implementation for Fifo file operation
impl FifoFileOperation {
    pub fn new(inode_id: InodeId) -> Self {
        log::info!("New fifo file operation registered");
        let mut fifo = FifoFileOperation::default();
        fifo.file_op_uid = get_file_op_uid();
        fifo.inode_id = inode_id;
        fifo
    }
}

/// Main Trait implementation
impl FileOperation for FifoFileOperation {
    fn register(&mut self, flags: OpenFlags) {
        // dbg!("register");
        // dbg!(flags);
        if flags.contains(OpenFlags::O_RDONLY) {
            if self.input_ref == 0 && self.output_ref > 0 {
                // if there wasn't any reader, but some writers, wake them all
                unsafe {
                    messaging::send_message(MessageTo::Opener {
                        uid_file_op: self.file_op_uid,
                    });
                }
            }
            self.input_ref += 1;
        } else if flags.contains(OpenFlags::O_WRONLY) {
            if self.output_ref == 0 && self.input_ref > 0 {
                // if there wasn't any writer, but some readers, wake them all
                unsafe {
                    messaging::send_message(MessageTo::Opener {
                        uid_file_op: self.file_op_uid,
                    });
                }
            }
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
impl Drop for FifoFileOperation {
    fn drop(&mut self) {
        eprintln!("Fifo droped !");
        VFS.lock().close_file_operation(self.inode_id);
    }
}
