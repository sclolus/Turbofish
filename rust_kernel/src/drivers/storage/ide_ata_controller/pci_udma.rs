//! This module contains udma read/write methods on IDE drive. See https://wiki.osdev.org/ATA/ATAPI_using_DMA

use super::{AtaError, AtaResult, Capabilities, DmaIo, Drive, StatusRegister};
use super::{Command, DmaStatus, Udma};
use super::{NbrSectors, Sector};

use io::{Io, Pio};

use crate::drivers::PIT0;
use core::time::Duration;

/// -------- INITIALISATION -------
/// Prepare a PRDT in system memory.
/// Send the physical PRDT address to the Bus Master PRDT Register.
/// ----------------------------------------------- READ / WRITE ------------------------------------------
/// - 1: Set the direction of the data transfer by setting the Read/Write bit in the Bus Master Command Register.
/// - 2: Clear the Error and Interrupt bit in the Bus Master Status Register.
/// - 3: Select the drive. (since to be useless almost on qemu)
/// - 4: Send the LBA and sector count to their respective ports.
/// - 5: Send the DMA transfer command to the ATA controller.
/// - 6: Set the Start/Stop bit on the Bus Master Command Register.
/// - 7: When an interrupt arrives (after the transfer is complete), respond by resetting the Start/Stop bit.
/// - 8: Read the controller and drive status to determine if the transfer completed successfully.
impl DmaIo for Drive {
    /// drive specific READ method
    fn read(
        &self,
        start_sector: Sector,
        nbr_sectors: NbrSectors,
        buf: *mut u8,
        udma: &mut Udma,
    ) -> AtaResult<()> {
        if nbr_sectors == NbrSectors(0) {
            log::error!("DMA request of 0 len");
            return Ok(());
        }
        // Use this size like a limit
        // let prd_size = NbrSectors::from(Udma::PRD_SIZE / 2);

        udma.set_read(); /* 1 */
        udma.clear_error(); /* 2 */
        udma.clear_interrupt(); /* 2 */

        /* 3, 4 */
        match self.capabilities {
            Capabilities::Lba48 => {
                self.init_lba48(start_sector, nbr_sectors);
                Pio::<u8>::new(self.command_register + Self::COMMAND)
                    .write(Command::AtaCmdReadDmaExt as u8);
            }
            Capabilities::Lba28 => {
                self.init_lba28(start_sector, nbr_sectors);
                Pio::<u8>::new(self.command_register + Self::COMMAND)
                    .write(Command::AtaCmdReadDma as u8);
            }
            // I experiment a lack of documentation about this mode
            Capabilities::Chs => {
                return Err(AtaError::NotSupported);
            }
        }

        /* 5 */
        udma.start_transfer();

        /*
         * Check if an error has occured
         */
        let status = udma.get_status();
        if status.contains(DmaStatus::FAILED) {
            log::error!("An error as occured: {:?}", status);
            return Err(AtaError::IoError);
        }

        loop {
            unsafe {
                asm!("hlt");
            }

            if TRIGGER.compare_and_swap(true, false, Ordering::Relaxed) == true {
                let status = udma.get_status(); /* 8 */

                if status.contains(DmaStatus::FAILED) {
                    log::error!("An error as occured: {:?}", status);
                    return Err(AtaError::IoError);
                } else if status.contains(DmaStatus::IRQ) {
                    // If transfer is done udma IRQ bit is set
                    break;
                }
            }
        }

        // Get the disk status to determine if the transfert was Okay,
        // It is also a necessary thing to avoid the blocking of IRQ /* 8 */
        let _disk_status = Pio::<u8>::new(self.command_register + Self::STATUS).read();
        // TODO: Check status

        unsafe {
            core::ptr::copy(udma.get_memory()[0].as_ptr(), buf, nbr_sectors.into());
        }

        // Set IRQ bit to 0
        Pio::<u8>::new(0xC040 + Udma::DMA_STATUS).write(0b100);

        // udma.reset_command();
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
        unimplemented!();
        // Ok(())
    }
}

use core::sync::atomic::{AtomicBool, Ordering};

static TRIGGER: AtomicBool = AtomicBool::new(false);

#[no_mangle]
fn primary_hard_disk_interrupt_handler() -> u32 {
    //eprintln!("{}", "primary IRQ");

    TRIGGER.store(true, Ordering::Relaxed);

    // let s = Pio::<u8>::new(0xC040 + Udma::DMA_STATUS).read();
    // Pio::<u8>::new(0xc040 + Udma::DMA_COMMAND).write(s & !udma::DmaCommand::ONOFF.bits());

    // It could be logical to check here is the IRQ flag is set and unset IRQ bit ?

    /*
     * Eq to udma.stop_transfer() since we cannot have access to `self` and a extern handler /* 6 */
     */
    Pio::<u8>::new(0xC040 + Udma::DMA_COMMAND).write(0);
    0
}

#[no_mangle]
fn secondary_hard_disk_interrupt_handler() -> u32 {
    0
}
