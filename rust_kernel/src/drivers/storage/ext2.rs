use super::{NbrSectors, Sector, BIOS_INT13H, SECTOR_SIZE};
use alloc::boxed::Box;
use core::fmt::{self, Debug};
use mbr::Mbr;

use ext2::DiskIo;
use ext2::Ext2Filesystem;
use ext2::IoResult;
use ext2::{align_next, align_prev, Errno};

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
    fn write_buffer(&mut self, offset: u64, buf: &[u8]) -> IoResult<u64> {
        // if offset.
        let len = buf.len();
        // let first_chunk_off = (SECTOR_SIZE as u64 - offset % SECTOR_SIZE as u64) as usize;
        // let prev = align_prev(offset, SECTOR_SIZE as u64);
        // let next = align_next(offset, SECTOR_SIZE as u64);

        let sector = Sector::from(offset + self.start_of_partition);
        unsafe {
            BIOS_INT13H.as_mut().unwrap().read(sector, NbrSectors(1), self.buf.as_mut_ptr()).map_err(|_| Errno::Eio)?;
        }
        for (x, b) in self.buf[(offset % SECTOR_SIZE as u64) as usize..].iter_mut().zip(buf.iter()) {
            *x = *b;
        }
        unsafe {
            BIOS_INT13H.as_mut().unwrap().write(sector, NbrSectors(1), self.buf.as_ptr()).map_err(|_| Errno::Eio)?;
        }
        Ok(len as u64)
    }

    fn read_buffer(&mut self, offset: u64, buf: &mut [u8]) -> IoResult<u64> {
        // if offset.
        let len = buf.len();
        // let first_chunk_off = (SECTOR_SIZE as u64 - offset % SECTOR_SIZE as u64) as usize;
        let _prev = align_prev(offset, SECTOR_SIZE as u64);
        let _next = align_next(offset, SECTOR_SIZE as u64);

        let sector = Sector::from(offset + self.start_of_partition);
        unsafe {
            BIOS_INT13H.as_mut().unwrap().read(sector, NbrSectors(1), self.buf.as_mut_ptr()).map_err(|_| Errno::Eio)?;
        }
        for (x, b) in self.buf[(offset % SECTOR_SIZE as u64) as usize..].iter().zip(buf.iter_mut()) {
            *b = *x;
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
