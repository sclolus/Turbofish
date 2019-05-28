use super::{NbrSectors, Sector, BIOS_INT13H, SECTOR_SIZE};
use alloc::boxed::Box;
use core::cmp::min;
use core::fmt::{self, Debug};
use errno::Errno;
use mbr::Mbr;

use ext2::DiskIo;
use ext2::Ext2Filesystem;
use ext2::IoResult;

pub struct DiskIoBios {
    start_of_partition: u64,
    partition_size: u64,
    buf: [u8; SECTOR_SIZE],
}

impl Debug for DiskIoBios {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}, {:?}", self.start_of_partition, self.partition_size)
    }
}

impl DiskIoBios {
    pub fn new(start_of_partition: u64, partition_size: u64) -> Self {
        Self { start_of_partition, partition_size, buf: [0; SECTOR_SIZE] }
    }
}

impl DiskIo for DiskIoBios {
    fn flush(&mut self) -> IoResult<()> {
        Ok(())
    }

    //pub fn read(&self, start_sector: Sector, nbr_sectors: NbrSectors, buf: *mut u8) -> DiskResult<()> {
    //pub fn write(&self, start_sector: Sector, nbr_sectors: NbrSectors, buf: *const u8) -> DiskResult<()> {
    fn write_buffer(&mut self, mut offset: u64, mut buf: &[u8]) -> IoResult<u64> {
        let len = buf.len();
        loop {
            let size_read = min((SECTOR_SIZE as u64 - offset % SECTOR_SIZE as u64) as usize, buf.len());

            let sector = Sector::from(offset + self.start_of_partition);
            unsafe {
                BIOS_INT13H
                    .as_mut()
                    .unwrap()
                    .read(sector, NbrSectors(1), self.buf.as_mut_ptr())
                    .map_err(|_| Errno::Eio)?;
            }
            let target_read = (offset % SECTOR_SIZE as u64) as usize;
            self.buf[target_read..target_read + size_read].copy_from_slice(&buf[0..size_read]);
            if size_read == buf.len() {
                break;
            }
            unsafe {
                BIOS_INT13H
                    .as_mut()
                    .unwrap()
                    .write(sector, NbrSectors(1), self.buf.as_ptr())
                    .map_err(|_| Errno::Eio)?;
            }
            buf = &buf[size_read..];
            offset += size_read as u64;
        }
        Ok(len as u64)
    }

    fn read_buffer(&mut self, mut offset: u64, mut buf: &mut [u8]) -> IoResult<u64> {
        let len = buf.len();
        loop {
            let size_read = min((SECTOR_SIZE as u64 - offset % SECTOR_SIZE as u64) as usize, buf.len());

            let sector = Sector::from(offset + self.start_of_partition);
            unsafe {
                BIOS_INT13H
                    .as_mut()
                    .unwrap()
                    .read(sector, NbrSectors(1), self.buf.as_mut_ptr())
                    .map_err(|_| Errno::Eio)?;
            }
            let target_read = (offset % SECTOR_SIZE as u64) as usize;
            buf[0..size_read].copy_from_slice(&self.buf[target_read..target_read + size_read]);
            if size_read == buf.len() {
                break;
            }
            buf = &mut buf[size_read..];
            offset += size_read as u64;
        }
        Ok(len as u64)
    }
}

pub static mut EXT2: Option<Ext2Filesystem> = None;

pub fn init(mbr: &Mbr) -> IoResult<()> {
    let disk_io = DiskIoBios::new(dbg!(mbr.parts[0].start as u64 * 512), mbr.parts[0].size as u64 * 512);
    unsafe {
        EXT2 = Some(Ext2Filesystem::new(Box::new(disk_io))?);
    }
    Ok(())
}
