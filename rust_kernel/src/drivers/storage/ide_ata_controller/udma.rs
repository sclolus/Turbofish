//! This module provide a toolkit for UDMA. See https://wiki.osdev.org/ATA/ATAPI_using_DMA

use io::{Io, Pio};

use bitflags::bitflags;

use crate::drivers::{pic_8259, PIC_8259};
use crate::memory::ffi::get_physical_addr;
use crate::memory::tools::*;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

/// Global UDMA structure
#[derive(Clone)]
pub struct Udma {
    memory: Vec<Vec<u8>>,
    bus_mastered_register: u16,
    channel: Channel,
    prdt: Box<Prdt>,
}

/// UDMA Debug boilerplate
impl core::fmt::Debug for Udma {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "CHANNELS: {:?} -> {:#X?} at IO/PORT: {:#X?}",
            self.channel,
            self.memory[0].as_ptr() as *const _,
            self.bus_mastered_register,
        )
    }
}

/// Our UDMA implementation contains two channels
#[derive(Copy, Clone, Debug)]
pub enum Channel {
    Primary,
    Secondary,
}

/// Physical region descriptor (size 8)
#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct PrdEntry {
    /// addr cannot cross 64K
    addr: Phys,
    /// size of the prd entry: A byte count of 0 means 64K
    size: u16,
    /// if set indicate that it is the last entry in the prdt
    is_end: u16,
}

/// Alignement = sizeof(struct PrdEntry) * Udma::NBR_DMA_ENTRIES <- Cannot cross a 64k boundary (size 8 * 16 = 128)
/// The PRDT must be uint32_t aligned, contiguous in physical memory, and cannot cross a 64K boundary.
#[derive(Debug, Clone)]
#[repr(align(128))]
struct Prdt([PrdEntry; Udma::NBR_DMA_ENTRIES]);

impl Prdt {
    fn new() -> Self {
        Self(
            [PrdEntry {
                addr: Phys(0),
                size: 0,
                is_end: 0,
            }; Udma::NBR_DMA_ENTRIES],
        )
    }
}

// There are just 2 DMA commands
bitflags! {
    pub struct DmaCommand: u8 {
        const ONOFF = 1 << 0; // Bit 0 (value = 1) is the Start/Stop bit. Setting the bit puts the controller in DMA mode for that ATA channel.
        const RDWR = 1 << 3; // Bit 3 (value = 8) The disk controller does not automatically detect whether the next disk operation is a read or write.
    }
}

// Some status indications
bitflags! {
    pub struct DmaStatus: u8 {
        const STATUS = 1 << 0; // Bit 0 (value = 1) is set when the bus goes into DMA mode. It is cleared when the last PRD in the table has been used up.
        const FAILED = 1 << 1; // Bit 1 (value = 2) is set if any DMA memory transfer failed for any reason in this PRDT.
        const IRQ = 1 << 2; // If bit 2 (value = 4) is not set after the OS receives an IRQ, then some other device sharing the IRQ generated the IRQ -- not the disk.
        const SOO = 1 << 7; // Bit 7 (Simplex operation only) is completely obsolete...
    }
}

impl Udma {
    /// *** These below constants are expressed with offset from the bus master register ***
    /// DMA Command Byte (8 bits)
    pub const DMA_COMMAND: u16 = 0x0;

    /// DMA Status Byte (8 bits)
    pub const DMA_STATUS: u16 = 0x2;

    /// DMA PRDT Address (32 bits)
    const DMA_PRDT_ADDR: u16 = 0x4;

    /// Number of PRD chunk Per PRDT
    pub const NBR_DMA_ENTRIES: usize = 16;

    /// Size of a PRD entries (eq to 64K)
    pub const PRD_SIZE: usize = 1 << 16;

    /// Init all UDMA channels
    pub fn init(mut bus_mastered_register: u16, channel: Channel) -> Self {
        // The data buffers cannot cross a 64K boundary
        let mut memory = vec![vec![0; Self::PRD_SIZE]; Self::NBR_DMA_ENTRIES];
        let mut prdt = Box::new(Prdt::new());

        // Qemu quick fix
        bus_mastered_register &= 0xfffe;

        // Init a new PRDT
        init_prdt(prdt.as_mut(), &mut memory);

        let physical_prdt_address = get_physical_addr(Virt(prdt.as_ref() as *const _ as usize))
            .expect("Buddy Allocator is bullshit")
            .0;

        // Check if physical_prdt_address is 'self' aligned and so cannot cross a 64k boundary
        assert!(physical_prdt_address % core::mem::size_of::<Prdt>() == 0);

        // Set the IO/PORT on Bus master register with physical DMA PRDT Address
        Pio::<u32>::new(bus_mastered_register + Self::DMA_PRDT_ADDR)
            .write(physical_prdt_address as u32);

        // Enable IRQ mask for a specific channel
        unsafe {
            match channel {
                Channel::Primary => PIC_8259.lock().enable_irq(pic_8259::Irq::PrimaryATAChannel),
                Channel::Secondary => PIC_8259
                    .lock()
                    .enable_irq(pic_8259::Irq::SecondaryATAChannel),
            }
        }

        Self {
            memory,
            channel,
            prdt,
            bus_mastered_register,
        }
    }

    /// Get the I/O port of the bus_mastered_register
    pub fn get_bus_mastered_register(&self) -> u16 {
        self.bus_mastered_register
    }

    /// Get the total size of the PRD(s)
    pub fn get_memory_amount(&self) -> usize {
        self.memory.len() * self.memory[0].len()
    }

    /// Get the complete memory DMA zone
    pub fn get_memory(&mut self) -> &mut Vec<Vec<u8>> {
        &mut self.memory
    }

    /// Reset bus master register's command register
    pub fn reset_command(&mut self) {
        Pio::<u8>::new(self.bus_mastered_register + Self::DMA_COMMAND).write(0);
    }

    /// Start the UDMA transfert
    pub fn start_transfer(&self) {
        let s = Pio::<u8>::new(self.bus_mastered_register + Self::DMA_COMMAND).read();
        Pio::<u8>::new(self.bus_mastered_register + Self::DMA_COMMAND)
            .write(s | DmaCommand::ONOFF.bits());
    }

    /// Stop the UDMA transfert
    pub fn stop_transfer(&self) {
        let s = Pio::<u8>::new(self.bus_mastered_register + Self::DMA_COMMAND).read();
        Pio::<u8>::new(self.bus_mastered_register + Self::DMA_COMMAND)
            .write(s & !DmaCommand::ONOFF.bits());
    }

    /// Set reading mode: DISK to DMA buffer
    pub fn set_read(&self) {
        let s = Pio::<u8>::new(self.bus_mastered_register + Self::DMA_COMMAND).read();
        Pio::<u8>::new(self.bus_mastered_register + Self::DMA_COMMAND)
            .write(s | DmaCommand::RDWR.bits());
    }

    /// Set writing mode: DMA buffer to DISK
    pub fn set_write(&self) {
        let s = Pio::<u8>::new(self.bus_mastered_register + Self::DMA_COMMAND).read();
        Pio::<u8>::new(self.bus_mastered_register + Self::DMA_COMMAND)
            .write(s & !DmaCommand::RDWR.bits());
    }

    /// Clear interrupt bit
    pub fn clear_interrupt(&self) {
        Pio::<u8>::new(self.bus_mastered_register + Self::DMA_STATUS).write(DmaStatus::IRQ.bits());
    }

    /// Clear error bit
    pub fn clear_error(&self) {
        let s = Pio::<u8>::new(self.bus_mastered_register + Self::DMA_STATUS).read();
        Pio::<u8>::new(self.bus_mastered_register + Self::DMA_STATUS)
            .write(s & !DmaStatus::FAILED.bits());
    }

    /// Get the UDMA status
    pub fn get_status(&self) -> DmaStatus {
        DmaStatus {
            bits: Pio::<u8>::new(self.bus_mastered_register + Self::DMA_STATUS).read(),
        }
    }

    /// Clear the IRQ bit in status
    pub fn clear_irq_bit(&self) {
        Pio::<u8>::new(self.bus_mastered_register + Self::DMA_STATUS).write(0b100);
    }
}

/// Set a unique PRDT
fn init_prdt(prdt: &mut Prdt, memory_zone: &mut Vec<Vec<u8>>) {
    for (mem, prd) in memory_zone.iter().zip(prdt.0.iter_mut()) {
        let addr =
            get_physical_addr(Virt(mem.as_ptr() as usize)).expect("Buddy Allocator is bullshit");
        // Check if data buffers are 64K aligned
        assert!(addr.0 & 0xffff == 0);
        *prd = PrdEntry {
            addr,
            size: 0,
            is_end: 0,
        }
    }
}
