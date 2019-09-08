//! This module contains udma read/write methods on IDE drive. See https://wiki.osdev.org/ATA/ATAPI_using_DMA

// TODO: IRQ handling seems broken (i dont know if is a Qemu bug or a mistake made by us)

use super::{AtaError, AtaResult, Capabilities, DmaIo, Drive, StatusRegister};
use super::{Command, DmaStatus, Udma};
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
// Select the drive. (since to be useless almost on qemu)
// Send the LBA and sector count to their respective ports.
// Send the DMA transfer command to the ATA controller.
// Set the Start/Stop bit on the Bus Master Command Register.
// When an interrupt arrives (after the transfer is complete), respond by resetting the Start/Stop bit.
// Read the controller and drive status to determine if the transfer completed successfully.
impl DmaIo for Drive {
    /// drive specific READ method
    fn read(
        &self,
        _start_sector: Sector,
        _nbr_sectors: NbrSectors,
        _buf: *mut u8,
        udma: &mut Udma,
    ) -> AtaResult<()> {
        let mut i = 0;
        loop {
            // udma.reset_command();
            udma.set_read();
            udma.clear_error();
            udma.clear_interrupt();

            self.fill_dma(Sector(i), NbrSectors(512), udma)?;
            i += 1;

            PIT0.lock().sleep(Duration::from_millis(1));
        }
        // Ok(())
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

impl Drive {
    fn fill_dma(
        &self,
        start_sector: Sector,
        nbr_sectors: NbrSectors,
        udma: &mut Udma,
    ) -> AtaResult<()> {
        if nbr_sectors == NbrSectors(0) {
            return Ok(());
        }
        // Use this size like a limit
        // let prd_size = NbrSectors::from(Udma::PRD_SIZE / 2);

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

        /*
         * Start the DMA transfert
         */
        udma.start_transfer();

        let status = udma.get_status();
        if status.contains(DmaStatus::FAILED) {
            eprintln!("An error as occured: {:?}", status);
            panic!("panic sa mere !");
        }

        loop {
            unsafe {
                asm!("hlt");
            }

            // When transfer is done udma IRQ bit is set
            if TRIGGER.compare_and_swap(true, false, Ordering::Relaxed) == true
                && udma.get_status().contains(DmaStatus::IRQ)
            {
                break;
            }
        }

        // Get the disk status to determine if the transfert was Okay,
        // It is also a necessary thing to avoid the blocking of IRQ
        let disk_status = Pio::<u8>::new(self.command_register + Self::STATUS).read();
        println!(
            "disk_status: {:?}",
            StatusRegister::from_bits_truncate(disk_status)
        );

        let u = udma.get_memory();
        for i in 0..512 {
            print!("{:X?} ", u[0][i]);
        }

        // Get the dma status

        /*
        let status = udma.get_status();
        if status.contains(DmaStatus::FAILED) {
            eprintln!("An error as occured: {:?}", status);
            panic!("panic sa mere !");
        }
        println!("\nSTATUS after the end transfer {:?}", status);
        */

        // Set IRQ bit to 0
        Pio::<u8>::new(0xC040 + Udma::DMA_STATUS).write(0b100);

        Ok(())
    }
}

use core::sync::atomic::{AtomicBool, Ordering};

static TRIGGER: AtomicBool = AtomicBool::new(false);

#[no_mangle]
fn primary_hard_disk_interrupt_handler() -> u32 {
    eprintln!("{}", "primary IRQ");

    TRIGGER.store(true, Ordering::Relaxed);

    // let s = Pio::<u8>::new(0xC040 + Udma::DMA_STATUS).read();
    // Pio::<u8>::new(0xc040 + Udma::DMA_COMMAND).write(s & !udma::DmaCommand::ONOFF.bits());

    // It could be logical to check here is the IRQ flag is set and unset IRQ bit ?

    /*
     * Eq to udma.stop_transfer() since we cannot have access to `self` and a extern handler
     */
    Pio::<u8>::new(0xC040 + Udma::DMA_COMMAND).write(0);
    0
}

#[no_mangle]
fn secondary_hard_disk_interrupt_handler() -> u32 {
    0
}
