//! This files contains the code related to the ATA PIO MODE
/// See https://wiki.osdev.org/ATA_PIO_Mode

#[deny(missing_docs)]
use super::Drive;
use super::SECTOR_SIZE;
use super::{check_bounds, AtaError, AtaResult, Capabilities, DeviceControlRegister, Hierarchy, Rank};
use super::{Command, ErrorRegister, PioIo, StatusRegister};

use crate::drivers::storage::tools::*;

use io::{Io, Pio};

use bit_field::BitField;

use core::slice;

impl PioIo for Drive {
    /// drive specific READ method
    fn read(&self, start_sector: Sector, nbr_sectors: NbrSectors, buf: *mut u8) -> AtaResult<()> {
        check_bounds(start_sector, nbr_sectors, self.sector_capacity)?;

        let s = unsafe { slice::from_raw_parts_mut(buf, nbr_sectors.into()) };

        match self.capabilities {
            Capabilities::Lba48 => {
                // Do disk operation for each 'chunk_size' bytes
                const CHUNK_SIZE: usize = SECTOR_SIZE * 256 * 256;

                for (i, chunk) in s.chunks_mut(CHUNK_SIZE).enumerate() {
                    let sectors_to_read = chunk.len().into();

                    self.init_lba48(start_sector + (i * CHUNK_SIZE).into(), sectors_to_read);

                    // Send the "READ SECTORS EXT" command (0x24) to port 0x1F7: outb(0x1F7, 0x24)
                    self.wait_available();
                    Pio::<u8>::new(self.command_register + Self::COMMAND).write(Command::AtaCmdReadPioExt as u8);

                    // Read n sectors and put them into buf
                    self.read_sectors(sectors_to_read, chunk.as_mut_ptr())?;
                }
                Ok(())
            }
            Capabilities::Lba28 => {
                // Do disk operation for each 'chunk_size' bytes
                const CHUNK_SIZE: usize = SECTOR_SIZE * 256;

                for (i, chunk) in s.chunks_mut(CHUNK_SIZE).enumerate() {
                    let sectors_to_read = chunk.len().into();

                    self.init_lba28(start_sector + (i * CHUNK_SIZE).into(), sectors_to_read);

                    // Send the "READ SECTORS" command (0x20) to port 0x1F7: outb(0x1F7, 0x20)
                    self.wait_available();
                    Pio::<u8>::new(self.command_register + Self::COMMAND).write(Command::AtaCmdReadPio as u8);

                    // Read n sectors and put them into buf
                    self.read_sectors(sectors_to_read, chunk.as_mut_ptr())?;
                }
                Ok(())
            }
            // I experiment a lack of documentation about this mode
            Capabilities::Chs => Err(AtaError::NotSupported),
        }
    }

    /// Drive specific WRITE method
    fn write(&self, start_sector: Sector, nbr_sectors: NbrSectors, buf: *const u8) -> AtaResult<()> {
        check_bounds(start_sector, nbr_sectors, self.sector_capacity)?;

        let s = unsafe { slice::from_raw_parts(buf, nbr_sectors.into()) };

        match self.capabilities {
            Capabilities::Lba48 => {
                // Do disk operation for each 'chunk_size' bytes (32mo max for lba48)
                const CHUNK_SIZE: usize = SECTOR_SIZE * 256 * 256;

                for (i, chunk) in s.chunks(CHUNK_SIZE).enumerate() {
                    let sectors_to_write = chunk.len().into();

                    self.init_lba48(start_sector + (i * CHUNK_SIZE).into(), sectors_to_write);

                    // Send the "WRITE SECTORS EXT" command (0x34) to port 0x1F7: outb(0x1F7, 0x34)
                    self.wait_available();
                    Pio::<u8>::new(self.command_register + Self::COMMAND).write(Command::AtaCmdWritePioExt as u8);

                    // Write n sectors from buf to disk
                    self.write_sectors(sectors_to_write, chunk.as_ptr())?;

                    // Fflush write cache
                    self.fflush_write_cache();
                }
                Ok(())
            }
            Capabilities::Lba28 => {
                // Do disk operation for each 'chunk_size' bytes (32k max for lba28)
                const CHUNK_SIZE: usize = SECTOR_SIZE * 256;

                for (i, chunk) in s.chunks(CHUNK_SIZE).enumerate() {
                    let sectors_to_write = chunk.len().into();

                    self.init_lba28(start_sector + (i * CHUNK_SIZE).into(), sectors_to_write);

                    // Send the "WRITE SECTORS" command (0x30) to port 0x1F7: outb(0x1F7, 0x30)
                    self.wait_available();
                    Pio::<u8>::new(self.command_register + Self::COMMAND).write(Command::AtaCmdWritePio as u8);

                    // Write n sectors from buf to disk
                    self.write_sectors(sectors_to_write, chunk.as_ptr())?;

                    // Fflush write cache
                    self.fflush_write_cache();
                }
                Ok(())
            }
            // I experiment a lack of documentation about this mode
            Capabilities::Chs => Err(AtaError::NotSupported),
        }
    }
}

impl Drive {
    /// Read n_sectors, store them into buf
    fn read_sectors(&self, nbr_sectors: NbrSectors, buf: *const u8) -> AtaResult<()> {
        for sector in 0..nbr_sectors.0 as usize {
            // Wait for end of Busy state and DRQ ready
            self.busy_wait()?;

            let p = buf as *mut u16;
            for i in 0..SECTOR_SIZE >> 1 {
                unsafe {
                    *p.add(i + sector * (SECTOR_SIZE >> 1)) = Pio::<u16>::new(self.command_register + Self::DATA).read()
                }
            }
        }
        Ok(())
    }

    /// Write n sectors from buf
    fn write_sectors(&self, nbr_sectors: NbrSectors, buf: *const u8) -> AtaResult<()> {
        for sector in 0..nbr_sectors.0 as usize {
            // Wait for end of Busy state and DRQ ready
            self.busy_wait()?;

            let p = buf as *const u16;
            for i in 0..SECTOR_SIZE >> 1 {
                unsafe {
                    Pio::<u16>::new(self.command_register + Self::DATA).write(*p.add(i + sector * (SECTOR_SIZE >> 1)))
                }
            }
        }
        Ok(())
    }

    /// Wait for end of Busy state and DRQ ready
    fn busy_wait(&self) -> AtaResult<()> {
        loop {
            let r = StatusRegister::from_bits_truncate(Pio::<u8>::new(self.command_register + Self::STATUS).read());
            if r.contains(StatusRegister::ERR) {
                eprintln!(
                    "unexpected error while busy of {:?} err: {:?}",
                    self.rank,
                    ErrorRegister::from_bits_truncate(Pio::<u8>::new(self.command_register + Self::ERROR).read())
                );
                return Err(AtaError::IoError);
            }
            if !r.contains(StatusRegister::BSY) && r.contains(StatusRegister::DRQ) {
                break;
            }
        }
        Ok(())
    }

    /// On some drives it is necessary to "manually" flush the hardware write cache after every write command.
    /// This is done by sending the 0xE7 command to the Command Register (then waiting for BSY to clear).
    /// If a driver does not do this, then subsequent write commands can fail invisibly,
    /// or "temporary bad sectors" can be created on your disk.
    fn fflush_write_cache(&self) {
        match self.capabilities {
            Capabilities::Lba28 => {
                Pio::<u8>::new(self.command_register + Self::COMMAND).write(Command::AtaCmdCacheFlush as u8)
            }
            Capabilities::Lba48 => {
                Pio::<u8>::new(self.command_register + Self::COMMAND).write(Command::AtaCmdCacheFlushExt as u8)
            }
            _ => {}
        };

        let p = Pio::<u8>::new(self.command_register + Self::STATUS);
        while StatusRegister::from_bits_truncate(p.read()).contains(StatusRegister::BSY) {}
    }

    /// The method suggested in the ATA specs for sending ATA commands tells you to check the BSY and DRQ bits before trying to send a command
    fn wait_available(&self) {
        // Continue polling one of the Status ports until bit 3 (DRQ, value = 8) sets, or until bit 0 (BSY, value = 7) sets.
        while StatusRegister::from_bits_truncate(Pio::<u8>::new(self.control_register + Self::ALTERNATE_STATUS).read())
            .intersects(StatusRegister::BSY | StatusRegister::DRQ)
        {}
    }

    /// Init read or write sequence for lba48 mode
    fn init_lba48(&self, start_sector: Sector, nbr_sectors: NbrSectors) {
        let lba_low = start_sector.0.get_bits(0..32) as u32;
        let lba_high = start_sector.0.get_bits(32..48) as u32;

        // Send 0x40 for the "master" or 0x50 for the "slave" to port 0x1F6: outb(0x1F6, 0x40 | (slavebit << 4))
        self.wait_available();
        match self.get_hierarchy() {
            Hierarchy::Master => Pio::<u8>::new(self.command_register + Self::SELECTOR).write(0x40),
            Hierarchy::Slave => Pio::<u8>::new(self.command_register + Self::SELECTOR).write(0x50),
        }

        // Outb (0x1F2, sectorcount high byte)
        Pio::<u8>::new(self.command_register + Self::SECTOR_COUNT).write(nbr_sectors.0.get_bits(8..16) as u8);

        // LBA 4
        Pio::<u8>::new(self.command_register + Self::L1_SECTOR).write(lba_low.get_bits(24..32) as u8);
        // LBA 5
        Pio::<u8>::new(self.command_register + Self::L2_CYLINDER).write(lba_high.get_bits(0..8) as u8);
        // LBA 6
        Pio::<u8>::new(self.command_register + Self::L3_CYLINDER).write(lba_high.get_bits(8..16) as u8);

        // outb (0x1F2, sectorcount low byte)
        Pio::<u8>::new(self.command_register + Self::SECTOR_COUNT).write(nbr_sectors.0.get_bits(0..8) as u8);

        // LBA 1
        Pio::<u8>::new(self.command_register + Self::L1_SECTOR).write(lba_low.get_bits(0..8) as u8);
        // LBA 2
        Pio::<u8>::new(self.command_register + Self::L2_CYLINDER).write(lba_low.get_bits(8..16) as u8);
        // LBA 3
        Pio::<u8>::new(self.command_register + Self::L3_CYLINDER).write(lba_low.get_bits(16..24) as u8);
    }

    /// Init read or write sequence for lba28 mode
    fn init_lba28(&self, start_sector: Sector, nbr_sectors: NbrSectors) {
        let lba_low = start_sector.0.get_bits(0..32) as u32;

        // Send 0xE0 for the "master" or 0xF0 for the "slave" to port 0x1F6
        // and add the highest 4 bits of the LBA to port 0x1F6: outb(0x1F6, 0xE0 | (slavebit << 4) | ((LBA >> 24) & 0x0F))
        self.wait_available();
        match self.get_hierarchy() {
            Hierarchy::Master => {
                Pio::<u8>::new(self.command_register + Self::SELECTOR).write(0xE0 | ((lba_low >> 24) & 0xF) as u8)
            }
            Hierarchy::Slave => {
                Pio::<u8>::new(self.command_register + Self::SELECTOR).write(0xF0 | ((lba_low >> 24) & 0xF) as u8)
            }
        }

        // Send a NULL byte to port 0x1F1, if you like (it is ignored and wastes lots of CPU time): outb(0x1F1, 0x00)
        Pio::<u8>::new(self.command_register + Self::FEATURES).write(0);

        // outb (0x1F2, sectorcount low byte)
        Pio::<u8>::new(self.command_register + Self::SECTOR_COUNT).write(nbr_sectors.0.get_bits(0..8) as u8);

        // LBA 1
        Pio::<u8>::new(self.command_register + Self::L1_SECTOR).write(lba_low.get_bits(0..8) as u8);
        // LBA 2
        Pio::<u8>::new(self.command_register + Self::L2_CYLINDER).write(lba_low.get_bits(8..16) as u8);
        // LBA 3
        Pio::<u8>::new(self.command_register + Self::L3_CYLINDER).write(lba_low.get_bits(16..24) as u8);
    }

    /// Extract the sub tag hierarchy from rank
    fn get_hierarchy(&self) -> Hierarchy {
        match self.rank {
            Rank::Primary(h) | Rank::Secondary(h) => h,
        }
    }

    /// Select the drive for future read and write operations
    pub fn select_drive(&self) {
        self.wait_available();
        match self.get_hierarchy() {
            // select a target drive by sending 0xA0 for the master drive, or 0xB0 for the slave
            // I dont think it is necessary or really true
            Hierarchy::Master => Pio::<u8>::new(self.command_register + Self::SELECTOR).write(0xA0),
            Hierarchy::Slave => Pio::<u8>::new(self.command_register + Self::SELECTOR).write(0xB0),
        };
        // Disable interruot bit for the selected drive
        Pio::<u8>::new(self.control_register + Self::DEVICE_CONTROL).write(DeviceControlRegister::NIEN.bits());
    }
}

#[no_mangle]
fn primary_hard_disk_interrupt_handler() -> u32 {
    0
}

#[no_mangle]
fn secondary_hard_disk_interrupt_handler() -> u32 {
    0
}
