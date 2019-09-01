use crate::tools::IoResult;
use core::fmt::Debug;
use core::mem::size_of;
use libc_binding::Errno;
extern crate alloc;
use alloc::boxed::Box;

// const START_OF_PARTITION: u64 = 0;

/// trait to read / write on a disk
pub trait DiskIo: Debug + Send {
    /// flush
    fn flush(&mut self) -> IoResult<()>;
    /// write at offset
    fn write_buffer(&mut self, offset: u64, buf: &[u8]) -> IoResult<u64>;
    /// read at offset
    fn read_buffer(&mut self, offset: u64, buf: &mut [u8]) -> IoResult<u64>;

    // /// Try to clone xD
    // pub fn try_clone(&self) -> std::io::Result<Self> {
    //     Ok(Self(self.0.try_clone()?))
    // }
}

#[derive(Debug)]
pub struct Disk(pub Box<dyn DiskIo>);

impl Disk {
    pub fn write_buffer(&mut self, offset: u64, buf: &[u8]) -> IoResult<u64> {
        self.0.write_buffer(offset, buf)
    }

    pub fn read_buffer(&mut self, offset: u64, buf: &mut [u8]) -> IoResult<u64> {
        self.0.read_buffer(offset, buf)
    }

    pub fn write_all(&mut self, mut offset: u64, mut buf: &[u8]) -> IoResult<()> {
        while !buf.is_empty() {
            match self.0.write_buffer(offset, buf) {
                Ok(0) => return Err(Errno::EIO),
                Ok(n) => {
                    offset += n;
                    buf = &buf[n as usize..]
                }
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    /// Write a particulary struct inside file object
    pub fn write_struct<T: Copy>(&mut self, offset: u64, t: &T) -> IoResult<()> {
        let s = unsafe { core::slice::from_raw_parts(t as *const _ as *const u8, size_of::<T>()) };
        self.write_all(offset, s)
    }

    /// Read a particulary struct in file object
    pub fn read_struct<T: Copy>(&mut self, offset: u64) -> IoResult<T> {
        let t: T;
        unsafe {
            t = core::mem::uninitialized();
            let count = self.0.read_buffer(
                offset,
                core::slice::from_raw_parts_mut(&t as *const T as *mut u8, size_of::<T>()),
            )?;
            if count as usize != size_of::<T>() {
                return Err(Errno::EIO);
            }
        }
        Ok(t)
    }
}
