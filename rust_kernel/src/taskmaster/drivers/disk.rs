// use alloc::boxed::Box;
use super::Driver;
use super::FileOperation;
use super::IpcResult;
use super::SysResult;
use crate::drivers::storage::{
    BlockIo, DiskResult, NbrSectors, Sector, BIOS_INT13H, IDE_ATA_CONTROLLER, SECTOR_SIZE,
};
use alloc::sync::Arc;
use core::cmp::min;
use core::fmt::{self, Debug};
use ext2::IoResult;
use libc_binding::off_t;
use libc_binding::Errno;

#[derive(Debug)]
pub struct DiskDriver<D: BlockIo + Clone + Debug + 'static> {
    disk: D,
    start_of_partition: u64,
    partition_size: u64,
}

impl<D: BlockIo + Clone + Debug> DiskDriver<D> {
    pub fn new(disk: D, start_of_partition: u64, partition_size: u64) -> Self {
        Self {
            disk,
            start_of_partition,
            partition_size,
        }
    }
}

impl<D: BlockIo + Clone + Debug> Driver for DiskDriver<D> {
    fn open(&mut self) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>> {
        Ok(IpcResult::Done(Arc::new(DeadMutex::new(
            DiskFileOperation::new(
                self.disk.clone(),
                self.start_of_partition,
                self.partition_size,
            ),
        ))))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BiosInt13hInstance;

impl BlockIo for BiosInt13hInstance {
    fn read(&self, start_sector: Sector, nbr_sectors: NbrSectors, buf: *mut u8) -> DiskResult<()> {
        unsafe {
            BIOS_INT13H
                .as_ref()
                .unwrap()
                .read(start_sector, nbr_sectors, buf)
        }
    }

    fn write(
        &self,
        start_sector: Sector,
        nbr_sectors: NbrSectors,
        buf: *const u8,
    ) -> DiskResult<()> {
        unsafe {
            BIOS_INT13H
                .as_ref()
                .unwrap()
                .write(start_sector, nbr_sectors, buf)
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct IdeAtaInstance;

impl BlockIo for IdeAtaInstance {
    fn read(&self, start_sector: Sector, nbr_sectors: NbrSectors, buf: *mut u8) -> DiskResult<()> {
        unsafe {
            IDE_ATA_CONTROLLER
                .as_ref()
                .unwrap()
                .read(start_sector, nbr_sectors, buf)
        }
    }

    fn write(
        &self,
        start_sector: Sector,
        nbr_sectors: NbrSectors,
        buf: *const u8,
    ) -> DiskResult<()> {
        unsafe {
            IDE_ATA_CONTROLLER
                .as_ref()
                .unwrap()
                .write(start_sector, nbr_sectors, buf)
        }
    }
}

/// transform a disk which read sector by sector into a disk which
/// implement file operation
pub struct DiskFileOperation<D: BlockIo> {
    disk: D,
    offset: u64,
    start_of_partition: u64,
    partition_size: u64,
    buf: [u8; SECTOR_SIZE],
}

impl<D: BlockIo> Debug for DiskFileOperation<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?}, {:?}",
            self.start_of_partition, self.partition_size
        )
    }
}

impl<D: BlockIo> DiskFileOperation<D> {
    pub fn new(disk: D, start_of_partition: u64, partition_size: u64) -> Self {
        Self {
            disk: disk,
            offset: 0,
            start_of_partition,
            partition_size,
            buf: [0; SECTOR_SIZE],
        }
    }
}

impl<D: BlockIo + Send> FileOperation for DiskFileOperation<D> {
    fn write(&mut self, mut buf: &[u8]) -> SysResult<IpcResult<u32>> {
        let len = buf.len();
        loop {
            let size_write = min(
                (SECTOR_SIZE as u64 - self.offset % SECTOR_SIZE as u64) as usize,
                buf.len(),
            );
            if size_write == 0 {
                break;
            }

            let sector = Sector::from(self.offset + self.start_of_partition);
            self.disk
                .read(sector, NbrSectors(1), self.buf.as_mut_ptr())
                .map_err(|_| Errno::EIO)?;
            let target_read = (self.offset % SECTOR_SIZE as u64) as usize;
            self.buf[target_read..target_read + size_write].copy_from_slice(&buf[0..size_write]);
            self.disk
                .write(sector, NbrSectors(1), self.buf.as_ptr())
                .map_err(|_| Errno::EIO)?;
            buf = &buf[size_write..];
            self.offset += size_write as u64;
        }
        Ok(IpcResult::Done(len as u32))
    }

    fn read(&mut self, mut buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        let len = buf.len();
        loop {
            let size_read = min(
                (SECTOR_SIZE as u64 - self.offset % SECTOR_SIZE as u64) as usize,
                buf.len(),
            );

            let sector = Sector::from(self.offset + self.start_of_partition);
            self.disk
                .read(sector, NbrSectors(1), self.buf.as_mut_ptr())
                .map_err(|_| Errno::EIO)?;
            let target_read = (self.offset % SECTOR_SIZE as u64) as usize;
            buf[0..size_read].copy_from_slice(&self.buf[target_read..target_read + size_read]);
            if size_read == buf.len() {
                break;
            }
            buf = &mut buf[size_read..];
            self.offset += size_read as u64;
        }
        Ok(IpcResult::Done(len as u32))
    }
    fn lseek(&mut self, offset: off_t, whence: Whence) -> SysResult<off_t> {
        if offset == core::i64::MIN {
            // volontary trash i64 min value to avoid -offset ==
            // offset
            return Err(Errno::EINVAL);
        }
        let new_offset = match whence {
            Whence::SeekCur => {
                if offset < 0 {
                    self.offset
                        .checked_sub((-offset) as u64)
                        .ok_or(Errno::EINVAL)?
                } else {
                    self.offset
                        .checked_add(offset as u64)
                        .ok_or(Errno::EINVAL)?
                }
            }
            Whence::SeekSet => {
                if offset < 0 {
                    return Err(Errno::EINVAL);
                }
                offset as u64
            }
            Whence::SeekEnd => {
                if offset > 0 {
                    return Err(Errno::EINVAL);
                }
                self.partition_size
                    .checked_sub((-offset) as u64)
                    .ok_or(Errno::EINVAL)?
            }
        };
        if new_offset > self.partition_size {
            return Err(Errno::EINVAL);
        }
        self.offset = new_offset;
        Ok(self.offset as off_t)
    }
}

use ext2::DiskIo;

#[derive(Debug, Clone)]
/// wrap a file operation into a disk wrapper useful for mount
pub struct DiskWrapper(pub Arc<DeadMutex<dyn FileOperation>>);

use core::convert::TryInto;
use libc_binding::Whence;
use sync::DeadMutex;

impl DiskIo for DiskWrapper {
    /// flush
    fn flush(&mut self) -> IoResult<()> {
        Ok(())
    }
    /// write at offset
    fn write_buffer(&mut self, offset: u64, buf: &[u8]) -> IoResult<u64> {
        self.0.lock().lseek(
            offset.try_into().map_err(|_| Errno::EINVAL)?,
            Whence::SeekSet,
        )?;
        match self.0.lock().write(buf)? {
            IpcResult::Done(r) => Ok(r as u64),
            _ => Err(Errno::EINVAL),
        }
    }
    /// read at offset
    fn read_buffer(&mut self, offset: u64, buf: &mut [u8]) -> IoResult<u64> {
        self.0.lock().lseek(
            offset.try_into().map_err(|_| Errno::EINVAL)?,
            Whence::SeekSet,
        )?;
        match self.0.lock().read(buf)? {
            IpcResult::Done(r) => Ok(r as u64),
            _ => Err(Errno::EINVAL),
        }
    }
}
