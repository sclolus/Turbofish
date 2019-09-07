//! This module contains udma read/write methods on IDE drive. See https://wiki.osdev.org/ATA/ATAPI_using_DMA

// TODO: IRQ handling seems broken (i dont know if is a Qemu bug or a mistake made by us)

use super::udma;
use super::{AtaError, AtaResult, Capabilities, DmaIo, Drive};
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
        start_sector: Sector,
        nbr_sectors: NbrSectors,
        _buf: *mut u8,
        udma: &mut Udma,
    ) -> AtaResult<()> {
        println!("dma read");

        udma.reset_command();
        udma.set_read();
        udma.clear_error();
        udma.clear_interrupt();

        // udma.start_transfer();
        self.fill_dma(start_sector, nbr_sectors, udma)?;
        // udma.stop_transfer();

        eprintln!(
            "current status of DMA controller '{:X?}'",
            udma.get_status()
        );

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
//        Ok(())
    }
}

impl Drive {
    fn fill_dma(
        &self,
        start_sector: Sector,
        nbr_sectors: NbrSectors,
        udma: &mut Udma,
    ) -> AtaResult<()> {
        println!("fill dma");
        if nbr_sectors == NbrSectors(0) {
            return Ok(());
        }
        //let prd_size = NbrSectors::from(Udma::PRD_SIZE / 2);
        let prd_size = NbrSectors(1);

        PIT0.lock().sleep(Duration::from_millis(50));

        match self.capabilities {
            Capabilities::Lba48 => {
                self.init_lba48(start_sector, prd_size);
                Pio::<u8>::new(self.command_register + Self::COMMAND)
                    .write(Command::AtaCmdReadDmaExt as u8);
            }
            Capabilities::Lba28 => {
                self.init_lba28(start_sector, prd_size);
                Pio::<u8>::new(self.command_register + Self::COMMAND)
                    .write(Command::AtaCmdReadDma as u8);
            }
            // I experiment a lack of documentation about this mode
            Capabilities::Chs => {
                return Err(AtaError::NotSupported);
            }
        }
        udma.start_transfer();

        // println!("after start transfer{:?}", udma.get_status());
        // PIT0.lock().sleep(Duration::from_millis(1000));

        // let status = udma.get_status();
        // if status.contains(DmaStatus::FAILED) {
        //     eprintln!("An error as occured: {:?}", status);
        //     panic!("panic sa mere !");
        // }

        // loop {
        //     unsafe {
        //         asm!("hlt");
        //     }

        //     if TRIGGER.compare_and_swap(true, false, Ordering::Relaxed) == true
        //         && udma.get_status().contains(DmaStatus::IRQ)
        //     {
        //         break;
        //     }
        // }

        PIT0.lock().sleep(Duration::from_millis(500));

        let u = udma.get_memory();
        for i in 0..512 {
            print!("{:X?} ", u[0][i]);
        }

        udma.clear_interrupt();
        let status = udma.get_status();
        if status.contains(DmaStatus::FAILED) {
            eprintln!("An error as occured: {:?}", status);
            panic!("panic sa mere !");
        }
        //self.wait_available();
        //udma.stop_transfer();
        println!("\nSTATUS after the end transfer {:?}", udma.get_status());
        dbg!(udma);
        Ok(())
    }
}

use core::sync::atomic::{AtomicBool, Ordering};

static TRIGGER: AtomicBool = AtomicBool::new(false);

#[no_mangle]
fn primary_hard_disk_interrupt_handler() -> u32 {
    eprintln!("{}", "primary IRQ");
    TRIGGER.store(true, Ordering::Relaxed);

    // It seems to be wrong: Logically, we dont need to do things inside IRQ handling since we have Atomic recall
    // read disk status
    eprintln!("disk status {:?}", Pio::<u8>::new(0x0170 + 7).read());

    // read dma status
    let s = Pio::<u8>::new(0xC040 + Udma::DMA_STATUS).read();
    eprintln!("dma status {:?}", s);

    // clear interrupt
    Pio::<u8>::new(0xc040 + Udma::DMA_STATUS).write(s & !((1 << 2) as u8));

    // stop DMA transfert
    Pio::<u8>::new(0xc040 + Udma::DMA_COMMAND).write(s & !udma::DmaCommand::ONOFF.bits());
    Pio::<u8>::new(0xC040 + Udma::DMA_COMMAND).write(0);
    0
}

#[no_mangle]
fn secondary_hard_disk_interrupt_handler() -> u32 {
    0
}
