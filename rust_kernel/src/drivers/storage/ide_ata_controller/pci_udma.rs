//! This module contains udma read/write methods on IDE drive. See https://wiki.osdev.org/ATA/ATAPI_using_DMA

use super::{AtaCommand, AtaError, AtaResult, Capabilities, Drive, ErrorRegister, StatusRegister};
use super::{DmaCommand, DmaIo, DmaStatus, Udma};
use super::{NbrSectors, Sector};

use io::{Io, Pio};

/// -------- INITIALISATION -------
/// Prepare a PRDT in system memory.
/// Send the physical PRDT address to the Bus Master PRDT Register.
/// ----------------------------------------------- READ / WRITE ------------------------------------------
/// - 1: Set the direction of the data transfer by setting the Read/Write bit in the Bus Master Command Register.
/// - 2: Clear the Error and Interrupt bit in the Bus Master Status Register.
/// - 3: Select the drive. (seems to be almost useless on qemu)
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
    ) -> AtaResult<NbrSectors> {
        if nbr_sectors == NbrSectors(0) {
            log::error!("DMA request of 0 len");
            return Ok(NbrSectors(0));
        }
        // Set limit as UDMA buffer max_size for a single read operation
        let sectors_to_read = core::cmp::min(udma.get_memory_amount().into(), nbr_sectors);

        udma.set_read(); /* 1 */
        udma.clear_error(); /* 2 */
        udma.clear_interrupt(); /* 2 */

        /* 3, 4, 5 */
        match self.capabilities {
            Capabilities::Lba48 => {
                self.init_lba48(start_sector, sectors_to_read);
                Pio::<u8>::new(self.command_register + Self::COMMAND)
                    .write(AtaCommand::AtaCmdReadDmaExt as u8);
            }
            Capabilities::Lba28 => {
                self.init_lba28(start_sector, sectors_to_read);
                Pio::<u8>::new(self.command_register + Self::COMMAND)
                    .write(AtaCommand::AtaCmdReadDma as u8);
            }
            // I experiment a lack of documentation about this mode
            Capabilities::Chs => {
                return Err(AtaError::NotSupported);
            }
        }

        self.common_routine(udma)?;

        // Copy the UDMA content into the Buf
        let s = unsafe { core::slice::from_raw_parts_mut(buf, sectors_to_read.into()) };
        for (i, chunk) in s.chunks_mut(Udma::PRD_SIZE.into()).enumerate() {
            chunk.copy_from_slice(&udma.get_memory()[i][0..chunk.len()]);
        }

        Ok(sectors_to_read)
    }

    /// drive specific WRITE method
    fn write(
        &self,
        start_sector: Sector,
        nbr_sectors: NbrSectors,
        buf: *const u8,
        udma: &mut Udma,
    ) -> AtaResult<NbrSectors> {
        if nbr_sectors == NbrSectors(0) {
            log::error!("DMA request of 0 len");
            return Ok(NbrSectors(0));
        }
        // Set limit as UDMA buffer max_size for a single write operation
        let sectors_to_write = core::cmp::min(udma.get_memory_amount().into(), nbr_sectors);

        // Copy the Buf into the UDMA content
        let s = unsafe { core::slice::from_raw_parts(buf, sectors_to_write.into()) };
        for (i, chunk) in s.chunks(Udma::PRD_SIZE.into()).enumerate() {
            &udma.get_memory()[i][0..chunk.len()].copy_from_slice(chunk);
        }

        udma.set_write(); /* 1 */
        udma.clear_error(); /* 2 */
        udma.clear_interrupt(); /* 2 */

        /* 3, 4, 5 */
        match self.capabilities {
            Capabilities::Lba48 => {
                self.init_lba48(start_sector, sectors_to_write);
                Pio::<u8>::new(self.command_register + Self::COMMAND)
                    .write(AtaCommand::AtaCmdWriteDmaExt as u8);
            }
            Capabilities::Lba28 => {
                self.init_lba28(start_sector, sectors_to_write);
                Pio::<u8>::new(self.command_register + Self::COMMAND)
                    .write(AtaCommand::AtaCmdWriteDma as u8);
            }
            // I experiment a lack of documentation about this mode
            Capabilities::Chs => {
                return Err(AtaError::NotSupported);
            }
        }

        self.common_routine(udma)?;

        Ok(sectors_to_write)
    }
}

impl Drive {
    fn common_routine(&self, udma: &mut Udma) -> AtaResult<()> {
        // Set the current bus mastered register for the global IRQ handler
        unsafe {
            CURRENT_BUS_MASTERED_REGISTER = udma.get_bus_mastered_register();
        }

        /* 6 */
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
        let disk_status = Pio::<u8>::new(self.command_register + Self::STATUS).read();
        if StatusRegister::from_bits_truncate(disk_status).contains(StatusRegister::ERR) {
            log::error!(
                "unexpected disk error after DMA transfert: {:?}",
                ErrorRegister::from_bits_truncate(
                    Pio::<u8>::new(self.command_register + Self::ERROR).read()
                )
            );
            return Err(AtaError::IoError);
        }

        // udma.reset_command(); ???
        Ok(())
    }
}
use core::sync::atomic::{AtomicBool, Ordering};

static mut CURRENT_BUS_MASTERED_REGISTER: u16 = 0;
static TRIGGER: AtomicBool = AtomicBool::new(false);

pub unsafe extern "C" fn primary_hard_disk_interrupt_handler() {
    TRIGGER.store(true, Ordering::Relaxed);

    // It could be good to check here is the IRQ flag is set and unset IRQ bit ?

    /*
     * Eq to udma.stop_transfer() since we cannot have access to `self` and a extern handler /* 7 */
     */
    Pio::<u8>::new(CURRENT_BUS_MASTERED_REGISTER + Udma::DMA_COMMAND)
        .write(!DmaCommand::ONOFF.bits());
}

pub unsafe extern "C" fn secondary_hard_disk_interrupt_handler() {
    // TODO: For the moment. turbofish supports only one single disk operation at the same time
    primary_hard_disk_interrupt_handler()
}
