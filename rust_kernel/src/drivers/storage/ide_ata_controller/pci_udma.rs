//! This module contains udma read/write methods on IDE drive. See https://wiki.osdev.org/ATA/ATAPI_using_DMA

use super::{AtaError, AtaResult, Capabilities, DmaIo, Drive};
use super::{Command, Udma};
use super::{NbrSectors, Sector};

use io::{Io, Pio};

use crate::drivers::PIT0;
use core::time::Duration;

// -------- INITIALISATION -------
// Prepare a PRDT in system memory.
// Send the physical PRDT address to the Bus Master PRDT Register.
// ----------------------------------------------- READ / WRITE ------------------------------------------
// Set the direction of the data transfer by setting the Read/Write bit in the Bus Master Command Register.
// Clear the Error and Interrupt bit in the Bus Master Status Register.
// Select the drive.
// Send the LBA and sector count to their respective ports.
// Send the DMA transfer command to the ATA controller.
// Set the Start/Stop bit on the Bus Master Command Register.
// When an interrupt arrives (after the transfer is complete), respond by resetting the Start/Stop bit.
// Read the controller and drive status to determine if the transfer completed successfully.
impl DmaIo for Drive {
    /// drive specific READ method
    fn read(&self, start_sector: Sector, nbr_sectors: NbrSectors, _buf: *mut u8, udma: &mut Udma) -> AtaResult<()> {
        println!("dma read");

        udma.set_read();
        udma.clear_error();
        udma.clear_interrupt();

        self.select_drive();
        self.wait_available();

        match self.capabilities {
            Capabilities::Lba48 => {
                self.init_lba48(start_sector, nbr_sectors);
                Pio::<u8>::new(self.command_register + Self::COMMAND).write(Command::AtaCmdReadDmaExt as u8);
            }
            Capabilities::Lba28 => {
                self.init_lba28(start_sector, nbr_sectors);
                Pio::<u8>::new(self.command_register + Self::COMMAND).write(Command::AtaCmdReadDma as u8);
            }
            // I experiment a lack of documentation about this mode
            Capabilities::Chs => {
                return Err(AtaError::NotSupported);
            }
        }

        udma.start_transfer();

        PIT0.lock().sleep(Duration::from_millis(1000));
        let u = udma.get_memory();

        for i in 0..512 {
            print!("{:X?} ", u[0][i]);
        }
        eprintln!("current status of DMA controller '{:X?}'", udma.get_status());

        eprintln!("stoping transfer...");
        udma.stop_transfer();

        PIT0.lock().sleep(Duration::from_millis(1000));

        eprintln!("current status of DMA controller '{:X?}'", udma.get_status());

        let u = udma.get_memory();

        for i in 0..512 {
            print!("{:X?} ", u[0][i]);
        }
        Ok(())
    }

    /// drive specific WRITE method
    fn write(
        &self,
        _start_sector: Sector,
        _nbr_sectors: NbrSectors,
        _buf: *const u8,
        _udma: &mut Udma,
    ) -> AtaResult<()> {
        Ok(())
    }
}

#[no_mangle]
fn primary_hard_disk_interrupt_handler() -> u32 {
    println!("primary IRQ");
    0
}

#[no_mangle]
fn secondary_hard_disk_interrupt_handler() -> u32 {
    println!("secondary IRQ");
    0
}
