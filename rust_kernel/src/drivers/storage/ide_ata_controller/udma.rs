//! This module provide a toolkit for UDMA

use bitflags::bitflags;

use crate::memory::allocator::KERNEL_VIRTUAL_PAGE_ALLOCATOR;
use crate::memory::tools::*;

use alloc::vec;
use alloc::vec::Vec;

/// Global UDMA structure
#[derive(Clone)]
pub struct Udma {
    primary_channel: Vec<Vec<u8>>,
    secondary_channel: Vec<Vec<u8>>,
}

/// UDMA Debug boilerplate
impl core::fmt::Debug for Udma {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "CHANNELS: primary -> {:#X?} secondary -> {:#X?}",
            self.primary_channel[0].as_ptr() as *const _,
            self.secondary_channel[0].as_ptr() as *const _
        )
    }
}

/// Our UDMA implementation contains two channels
#[derive(Copy, Clone, Debug)]
pub enum UdmaChannel {
    Primary,
    Secondary,
}

/// Physical region descriptor
#[derive(Copy, Clone, Debug)]
#[repr(C)]
struct PrdEntry {
    /// addr cannot cross 64K
    addr: Phys,
    size: u16,
    /// if set indicate that it is the last entry in the prdt
    is_end: u16,
}

// There are just 2 DMA commands
bitflags! {
    struct DmaCommand: u8 {
        const ONOFF = 1 << 0; // Bit 0 (value = 1) is the Start/Stop bit. Setting the bit puts the controller in DMA mode for that ATA channel.
        const RDWR = 1 << 3; // Bit 3 (value = 8) The disk controller does not automatically detect whether the next disk operation is a read or write.
    }
}

// Some status indications
bitflags! {
    struct DmaStatus: u8 {
        const STATUS = 1 << 0; // Bit 0 (value = 1) is set when the bus goes into DMA mode. It is cleared when the last PRD in the table has been used up.
        const FAILED = 1 << 1; // Bit 1 (value = 2) is set if any DMA memory transfer failed for any reason in this PRDT.
        const IRQ = 1 << 2; // If bit 2 (value = 4) is not set after the OS receives an IRQ, then some other device sharing the IRQ generated the IRQ -- not the disk.
        const SOO = 1 << 7; // Bit 7 (Simplex operation only) is completely obsolete...
    }
}

impl Udma {
    /// *** These below constants are expressed with offset from the bus master register ***
    /// DMA Command Byte (8 bits)
    const _DMA_PRIMARY_COMMAND: usize = 0x0;
    const _DMA_SECONDARY_COMMAND: usize = 0x8;

    /// DMA Status Byte (8 bits)
    const _DMA_PRIMARY_STATUS: usize = 0x2;
    const _DMA_SECONDARY_STATUS: usize = 0xA;

    /// DMA PRDT Address (32 bits)
    const _DMA_PRIMARY_PRDT_ADDR: usize = 0x4;
    const _DMA_SECONDARY_PRDT_ADDR: usize = 0xC;

    /// Physical address of primary and secondary PRDT
    const PRDT_PRIMARY_PTR: usize = 0x4000;
    const PRDT_SECONDARY_PTR: usize = 0x8000;

    /// Number of PRD chunk Per PRDT
    const NBR_DMA_ENTRIES: usize = 16;

    /// Size of a PRD entries
    const PRD_SIZE: usize = 1 << 16;

    /// Init all UDMA channels
    pub fn init() -> Self {
        let mut primary_channel = vec![vec![0; Self::PRD_SIZE]; Self::NBR_DMA_ENTRIES];
        let mut secondary_channel = vec![vec![0; Self::PRD_SIZE]; Self::NBR_DMA_ENTRIES];

        let (prdt1, prdt2) = unsafe {
            (
                core::slice::from_raw_parts_mut(Self::PRDT_PRIMARY_PTR as *mut PrdEntry, Self::NBR_DMA_ENTRIES),
                core::slice::from_raw_parts_mut(Self::PRDT_SECONDARY_PTR as *mut PrdEntry, Self::NBR_DMA_ENTRIES),
            )
        };

        init_prdt(prdt1, &mut primary_channel);
        init_prdt(prdt2, &mut secondary_channel);

        Self { primary_channel, secondary_channel }
    }

    /// Get a specific UDMA channel
    pub fn get_channel(&mut self, channel: UdmaChannel) -> *mut Vec<Vec<u8>> {
        match channel {
            UdmaChannel::Primary => &mut self.primary_channel,
            UdmaChannel::Secondary => &mut self.secondary_channel,
        }
    }
}

/// Set a unique PRDT
fn init_prdt(prdt: &mut [PrdEntry], memory_zone: &mut Vec<Vec<u8>>) {
    for (mem, prd) in memory_zone.iter().zip(prdt.iter_mut()) {
        unsafe {
            *prd = PrdEntry {
                addr: KERNEL_VIRTUAL_PAGE_ALLOCATOR
                    .as_mut()
                    .unwrap()
                    .get_physical_addr(Virt(mem.as_ptr() as usize))
                    .unwrap(),
                size: 0,
                is_end: 0,
            }
        }
    }
}
