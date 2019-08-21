//! This file contains all the stuff about Pipes

use super::SysResult;

use super::FileOperation;
use super::IpcResult;
use super::Mode;

use libc_binding::Errno;

use core::cmp;

const BUF_SIZE: usize = 16;

/// This structure represents a FileOperation of type Pipe
#[derive(Debug, Default)]
pub struct Pipe {
    input_ref: usize,
    output_ref: usize,
    buf: [u8; BUF_SIZE],
    current_index: usize,
}

/// Main implementation for Pipe
impl Pipe {
    pub fn new() -> Self {
        Self {
            input_ref: Default::default(),
            output_ref: Default::default(),
            buf: [0; BUF_SIZE],
            current_index: 0,
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
            Mode::ReadOnly => self.input_ref -= 1,
            Mode::WriteOnly => self.output_ref -= 1,
            _ => panic!("Pipe invalid access mode"),
        };
    }
    fn read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        if self.current_index == 0 {
            return Ok(IpcResult::Wait(0));
        }

        // memcpy(buf, self.buf, MIN(buf.len(), self.current_index)
        let min = cmp::min(buf.len(), self.current_index);
        unsafe {
            buf.as_mut_ptr()
                .copy_from(self.buf.as_ptr(), self.current_index);
        }
        // memcpy(self.buf, self.buf + MIN(buf.len(), self.current_index), self.current_index - MIN(buf.len(), self.current_index))
        unsafe {
            self.buf
                .as_mut_ptr()
                .copy_from(self.buf.as_mut_ptr().add(min), self.current_index - min);
        }
        self.current_index -= min;
        Ok(IpcResult::Done(min as _))
    }
    fn write(&mut self, buf: &[u8]) -> SysResult<IpcResult<u32>> {
        if self.input_ref == 0 {
            return Err(Errno::EPIPE);
        }

        let min = cmp::min(buf.len(), BUF_SIZE - self.current_index);
        unsafe {
            self.buf
                .as_mut_ptr()
                .add(self.current_index)
                .copy_from(buf.as_ptr(), min);
        }
        self.current_index += min;
        if min == buf.len() {
            Ok(IpcResult::Done(min as _))
        } else {
            Ok(IpcResult::Wait(min as _))
        }
    }
}

/// Some boilerplate to check if all is okay
impl Drop for Pipe {
    fn drop(&mut self) {
        println!("Pipe droped !");
    }
}
