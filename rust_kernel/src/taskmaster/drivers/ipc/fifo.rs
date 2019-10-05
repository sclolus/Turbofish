//! This file contains all the stuff about Fifo

use super::SysResult;

use super::get_file_op_uid;
use super::Buf;
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
    operation: Arc<DeadMutex<FifoFileOperationData>>,
}

/// Main implementation of FifoDriver
impl FifoDriver {
    pub fn try_new(inode_id: InodeId) -> SysResult<Self> {
        Ok(Self {
            inode_id,
            operation: Arc::try_new(DeadMutex::new(FifoFileOperationData::new(inode_id)))?,
        })
    }

    /// generate a fifo file operation which share its FifoFileOperationData with the driver
    pub fn generate_file_operation(&self) -> SysResult<Arc<DeadMutex<dyn FileOperation>>> {
        Ok(Arc::try_new(DeadMutex::new(FifoFileOperation::new(
            self.operation.clone(),
        )))?)
    }
}

/// Driver trait implementation of FifoDriver
impl Driver for FifoDriver {
    fn open(
        &mut self,
        flags: OpenFlags,
    ) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>> {
        if flags.contains(OpenFlags::O_RDONLY) {
            if self.operation.lock().output_ref == 0 {
                Ok(IpcResult::Wait(
                    self.generate_file_operation()?,
                    self.operation.lock().file_op_uid,
                ))
            } else {
                Ok(IpcResult::Done(self.generate_file_operation()?))
            }
        } else if flags.contains(OpenFlags::O_WRONLY) {
            if self.operation.lock().input_ref == 0 {
                Ok(IpcResult::Wait(
                    self.generate_file_operation()?,
                    self.operation.lock().file_op_uid,
                ))
            } else {
                Ok(IpcResult::Done(self.generate_file_operation()?))
            }
        } else {
            Err(Errno::EINVAL)
        }
    }
}

#[derive(Debug, Default)]
pub struct FifoFileOperation {
    pub data: Arc<DeadMutex<FifoFileOperationData>>,
}

impl FifoFileOperation {
    pub fn new(data: Arc<DeadMutex<FifoFileOperationData>>) -> Self {
        Self { data }
    }
}

/// This structure represents a FileOperation of type Fifo
#[derive(Debug, Default)]
pub struct FifoFileOperationData {
    inode_id: InodeId,
    buf: Buf,
    input_ref: usize,
    output_ref: usize,
    current_index: usize,
    file_op_uid: usize,
    ref_count: usize,
}

/// Main implementation for Fifo file operation
impl FifoFileOperationData {
    pub fn new(inode_id: InodeId) -> Self {
        let mut fifo = FifoFileOperationData::default();
        fifo.file_op_uid = get_file_op_uid();
        fifo.inode_id = inode_id;
        fifo
    }
}

impl FileOperation for FifoFileOperation {
    fn register(&mut self, flags: OpenFlags) {
        self.data.lock().register(flags)
    }
    fn unregister(&mut self, flags: OpenFlags) {
        self.data.lock().unregister(flags)
    }
    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        self.data.lock().read(buf)
    }
    fn write(&mut self, buf: &[u8]) -> SysResult<IpcResult<u32>> {
        self.data.lock().write(buf)
    }
}

/// Main Trait implementation
impl FileOperation for FifoFileOperationData {
    fn register(&mut self, flags: OpenFlags) {
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

    fn get_inode_id(&self) -> SysResult<InodeId> {
        Ok(self.inode_id)
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
        VFS.lock().close_file_operation(self.data.lock().inode_id);
    }
}
