// use alloc::boxed::Box;
use super::Driver;
use super::FileOperation;
use super::InodeId;
use super::IpcResult;
use super::SysResult;
use crate::drivers::storage::{
    BlockIo, DiskResult, NbrSectors, Sector, BIOS_INT13H, IDE_ATA_CONTROLLER, SECTOR_MASK,
    SECTOR_SHIFT, SECTOR_SIZE,
};
use alloc::sync::Arc;
use core::cmp::min;
use core::fmt::{self, Debug};
use ext2::IoResult;
use libc_binding::off_t;
use libc_binding::{Errno, OpenFlags};

#[derive(Debug)]
pub struct DiskDriver<D: BlockIo + Clone + Debug + 'static> {
    disk: D,
    start_of_partition: u64,
    partition_size: u64,
    /// this is an option for the bootstrap, when we create a
    /// diskdriver wich is not mount on the devfs
    inode_id: InodeId,
}

impl<D: BlockIo + Clone + Debug> DiskDriver<D> {
    pub fn new(disk: D, start_of_partition: u64, partition_size: u64) -> Self {
        Self {
            disk,
            start_of_partition,
            partition_size,
            inode_id: Default::default(),
        }
    }
}

impl<D: BlockIo + Clone + Debug> Driver for DiskDriver<D> {
    fn open(
        &mut self,
        _flags: OpenFlags,
    ) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>> {
        Ok(IpcResult::Done(Arc::new(DeadMutex::new(
            DiskFileOperation::new(
                self.disk.clone(),
                self.start_of_partition,
                self.partition_size,
                self.inode_id,
            ),
        ))))
    }

    fn set_inode_id(&mut self, inode_id: InodeId) {
        self.inode_id = inode_id;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BiosInt13hInstance;

impl BlockIo for BiosInt13hInstance {
    fn read(
        &mut self,
        start_sector: Sector,
        nbr_sectors: NbrSectors,
        buf: *mut u8,
    ) -> DiskResult<NbrSectors> {
        unsafe {
            BIOS_INT13H
                .as_mut()
                .unwrap()
                .read(start_sector, nbr_sectors, buf)
        }
    }

    /// return the size of the disk
    fn disk_size(&self) -> u64 {
        unsafe { BIOS_INT13H.as_ref().unwrap().disk_size() }
    }

    fn write(
        &mut self,
        start_sector: Sector,
        nbr_sectors: NbrSectors,
        buf: *const u8,
    ) -> DiskResult<NbrSectors> {
        unsafe {
            BIOS_INT13H
                .as_mut()
                .unwrap()
                .write(start_sector, nbr_sectors, buf)
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct IdeAtaInstance;

impl BlockIo for IdeAtaInstance {
    fn read(
        &mut self,
        start_sector: Sector,
        nbr_sectors: NbrSectors,
        buf: *mut u8,
    ) -> DiskResult<NbrSectors> {
        unsafe {
            IDE_ATA_CONTROLLER
                .as_mut()
                .unwrap()
                .read(start_sector, nbr_sectors, buf)
        }
    }

    fn write(
        &mut self,
        start_sector: Sector,
        nbr_sectors: NbrSectors,
        buf: *const u8,
    ) -> DiskResult<NbrSectors> {
        unsafe {
            IDE_ATA_CONTROLLER
                .as_mut()
                .unwrap()
                .write(start_sector, nbr_sectors, buf)
        }
    }

    /// return the size of the disk
    fn disk_size(&self) -> u64 {
        unimplemented!()
    }
}

/// transform a disk which read sector by sector into a disk which
/// implement file operation
pub struct DiskFileOperation<D: BlockIo> {
    disk: D,
    offset: u64,
    start_of_partition: u64,
    partition_size: u64,
    tmp_buf: [u8; SECTOR_SIZE],
    inode_id: InodeId,
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
    pub fn new(disk: D, start_of_partition: u64, partition_size: u64, inode_id: InodeId) -> Self {
        Self {
            disk: disk,
            offset: 0,
            start_of_partition,
            partition_size,
            // Hopefully. We got a temporary buf of SECTOR_SIZE for unaligned read/write !
            tmp_buf: [0; SECTOR_SIZE],
            inode_id,
        }
    }
}

impl<D: BlockIo + Send> FileOperation for DiskFileOperation<D> {
    fn get_inode_id(&self) -> SysResult<InodeId> {
        Ok(self.inode_id)
    }

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
                .read(sector, NbrSectors(1), self.tmp_buf.as_mut_ptr())
                .map_err(|_| Errno::EIO)?;
            let target_read = (self.offset % SECTOR_SIZE as u64) as usize;
            self.tmp_buf[target_read..target_read + size_write]
                .copy_from_slice(&buf[0..size_write]);
            self.disk
                .write(sector, NbrSectors(1), self.tmp_buf.as_ptr())
                .map_err(|_| Errno::EIO)?;
            buf = &buf[size_write..];
            self.offset += size_write as u64;
        }
        Ok(IpcResult::Done(len as u32))
    }

    /// Read data fron DiskIo: Convert bytes len to Sectors
    fn read(&mut self, mut buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        let len = buf.len();
        //let start_offset = self.offset; // Just used for Assert

        // Unaligned read start: in case of start_offset not multiple of SECTOR_SIZE
        let start_unaligned_bytes = (self.offset & SECTOR_MASK as u64) as usize;
        if start_unaligned_bytes != 0 {
            let sector = Sector::from(self.offset + self.start_of_partition as u64);
            self.disk
                .read(sector, NbrSectors(1), self.tmp_buf.as_mut_ptr())
                .map_err(|_| Errno::EIO)?;
            let size_to_copy = min(SECTOR_SIZE - start_unaligned_bytes, buf.len());
            buf[..size_to_copy].copy_from_slice(
                &self.tmp_buf[start_unaligned_bytes..start_unaligned_bytes + size_to_copy],
            );
            self.offset += size_to_copy as u64;
            if size_to_copy == buf.len() {
                //assert!(self.offset == start_offset + len as u64);
                return Ok(IpcResult::Done(len as u32));
            } else {
                buf = &mut buf[size_to_copy..];
            }
        }

        //assert!(self.offset & SECTOR_MASK as u64 == 0);
        // Aligned read
        let mut aligned_sectors: NbrSectors = NbrSectors(buf.len() >> SECTOR_SHIFT);
        while aligned_sectors != NbrSectors(0) {
            let sector = Sector::from(self.offset + self.start_of_partition as u64);
            let aligned_sectors_readen = self
                .disk
                .read(sector, aligned_sectors, buf.as_mut_ptr())
                .map_err(|_| Errno::EIO)?;
            //assert!(aligned_sectors_readen != NbrSectors(0));
            aligned_sectors = aligned_sectors - aligned_sectors_readen;
            let bytes_readen: usize = aligned_sectors_readen.0 << SECTOR_SHIFT;

            self.offset += bytes_readen as u64;
            buf = &mut buf[bytes_readen..];
        }
        //assert!(self.offset & SECTOR_MASK as u64 == 0);
        // Unaligned read end: in case of end_offset not multiple of SECTOR_SIZE
        let end_unaligned_bytes = ((self.offset + buf.len() as u64) & SECTOR_MASK as u64) as usize;
        if end_unaligned_bytes != 0 {
            let sector = Sector::from(self.offset + self.start_of_partition as u64);
            self.disk
                .read(sector, NbrSectors(1), self.tmp_buf.as_mut_ptr())
                .map_err(|_| Errno::EIO)?;
            let size_to_copy = min(end_unaligned_bytes, buf.len());
            buf[..size_to_copy].copy_from_slice(&self.tmp_buf[..size_to_copy]);
            self.offset += size_to_copy as u64;
        }

        //assert!(self.offset == start_offset + len as u64);
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
    // fn get_inode_id(&self) -> InodeId {
    //     self.inode_id
    // }
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
