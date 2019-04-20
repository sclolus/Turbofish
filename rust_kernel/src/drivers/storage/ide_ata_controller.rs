//! This module contains the turbo fish's ATA/IDE drivers

#[deny(missing_docs)]
pub mod pci_udma;
pub mod pio_polling;

use super::SECTOR_SIZE;
use super::{IdeControllerProgIf, MassStorageControllerSubClass, PciDeviceClass, PciType0, PCI};
use super::{NbrSectors, Sector};

use alloc::vec::Vec;
use io::{Io, Pio};

use bitflags::bitflags;

use crate::memory::allocator::KERNEL_VIRTUAL_PAGE_ALLOCATOR;
use crate::memory::tools::*;
use alloc::vec;

/// Global structure
#[derive(Clone)]
pub struct IdeAtaController {
    primary_master: Option<Drive>,
    secondary_master: Option<Drive>,
    primary_slave: Option<Drive>,
    secondary_slave: Option<Drive>,
    selected_drive: Option<Rank>,
    memory_dma_primary: Vec<Vec<u8>>,
    memory_dma_secondary: Vec<Vec<u8>>,
    pci: PciType0,
    pci_location: u32,
}

/// Debug boilerplate (Vector cannot be displayed)
impl core::fmt::Debug for IdeAtaController {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "{:#X?}\n{:#X?}\n{:#X?}\n{:#X?}\n{:#?}\n{:#X?}\npci addr: {:#X?}",
            self.primary_master,
            self.secondary_master,
            self.primary_slave,
            self.secondary_slave,
            self.selected_drive,
            self.pci,
            self.pci_location
        )
    }
}

/// Global disk characteristics
#[derive(Debug, Clone)]
struct Drive {
    command_register: u16,
    control_register: u16,
    capabilities: Capabilities,
    sector_capacity: NbrSectors,
    udma_support: u16,
    rank: Rank,
}

pub trait DmaIo {
    fn read(&self, start_sector: Sector, nbr_sectors: NbrSectors) -> AtaResult<()>;
    fn write(&self, start_sector: Sector, nbr_sectors: NbrSectors) -> AtaResult<()>;
}

/// When in PIO mode, buff address is passed by pointer and methods read or write on it
pub trait PioIo {
    fn read(&self, start_sector: Sector, nbr_sectors: NbrSectors, buf: *mut u8) -> AtaResult<()>;
    fn write(&self, start_sector: Sector, nbr_sectors: NbrSectors, buf: *const u8) -> AtaResult<()>;
}

/// Standard port location, if they are different, probe IDE controller in PCI driver
const PRIMARY_BASE_REGISTER: u16 = 0x01F0;
const SECONDARY_BASE_REGISTER: u16 = 0x0170;
const PRIMARY_CONTROL_REGISTER: u16 = 0x03f6;
const SECONDARY_CONTROL_REGISTER: u16 = 0x376;

/// physical region descriptor
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

impl IdeAtaController {
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
        crate::watch_dog();
    }

    fn init_dma(&mut self) {
        self.memory_dma_primary = vec![vec![0; 1 << 16]; Self::NBR_DMA_ENTRIES];
        self.memory_dma_secondary = vec![vec![0; 1 << 16]; Self::NBR_DMA_ENTRIES];

        let prdt1 =
            unsafe { core::slice::from_raw_parts_mut(Self::PRDT_PRIMARY_PTR as *mut PrdEntry, Self::NBR_DMA_ENTRIES) };
        let prdt2 = unsafe {
            core::slice::from_raw_parts_mut(Self::PRDT_SECONDARY_PTR as *mut PrdEntry, Self::NBR_DMA_ENTRIES)
        };

        Self::init_prdt(prdt1, &mut self.memory_dma_primary);
        Self::init_prdt(prdt2, &mut self.memory_dma_secondary);

        self.pci.bar4;
    }

    /// Invocation of a new PioMode-IDE controller
    pub fn new() -> Option<Self> {
        let (pci, pci_location) = PCI.lock().query_device::<PciType0>(PciDeviceClass::MassStorageController(
            MassStorageControllerSubClass::IdeController(
                IdeControllerProgIf::IsaCompatibilityModeOnlyControllerBusMastered,
            ),
        ))?;
        let primary_base_register =
            if pci.bar0 == 0 || pci.bar0 == 1 { PRIMARY_BASE_REGISTER } else { pci.bar0 as u16 };
        let primary_control_register =
            if pci.bar1 == 0 || pci.bar1 == 1 { PRIMARY_CONTROL_REGISTER } else { pci.bar1 as u16 };
        let secondary_base_register =
            if pci.bar2 == 0 || pci.bar2 == 1 { SECONDARY_BASE_REGISTER } else { pci.bar2 as u16 };
        let secondary_control_register =
            if pci.bar3 == 0 || pci.bar3 == 1 { SECONDARY_CONTROL_REGISTER } else { pci.bar3 as u16 };
        println!("PCI BAR 4: {:X?}", pci.bar4);
        let mut s = Self {
            primary_master: Drive::identify(
                Rank::Primary(Hierarchy::Master),
                primary_base_register,
                primary_control_register,
            ),
            secondary_master: Drive::identify(
                Rank::Primary(Hierarchy::Master),
                secondary_base_register,
                secondary_control_register,
            ),
            primary_slave: Drive::identify(
                Rank::Primary(Hierarchy::Slave),
                primary_base_register,
                primary_control_register,
            ),
            secondary_slave: Drive::identify(
                Rank::Primary(Hierarchy::Slave),
                secondary_base_register,
                secondary_control_register,
            ),
            selected_drive: None,
            pci_location,
            pci,
            memory_dma_primary: Vec::new(),
            memory_dma_secondary: Vec::new(),
        };
        s.init_dma();

        Some(s)
    }

    /// Select the drive we would like to read or write
    pub fn select_drive(&mut self, rank: Rank) -> AtaResult<()> {
        self.selected_drive = match rank {
            Rank::Primary(Hierarchy::Master) if self.primary_master.is_some() => Some(rank),
            Rank::Primary(Hierarchy::Slave) if self.primary_slave.is_some() => Some(rank),
            Rank::Secondary(Hierarchy::Master) if self.secondary_master.is_some() => Some(rank),
            Rank::Secondary(Hierarchy::Slave) if self.secondary_slave.is_some() => Some(rank),
            _ => None,
        };
        self.get_selected_drive().ok_or(AtaError::DeviceNotFound)?.select_drive();
        Ok(())
    }

    /// Read nbr_sectors after start_sector location and write it into the buf
    pub fn read(&self, start_sector: Sector, nbr_sectors: NbrSectors, buf: *mut u8) -> AtaResult<()> {
        self.get_selected_drive()
            .ok_or(AtaError::DeviceNotFound)
            .and_then(|d| PioIo::read(d, start_sector, nbr_sectors, buf))
    }

    /// Write nbr_sectors after start_sector location from the buf
    pub fn write(&self, start_sector: Sector, nbr_sectors: NbrSectors, buf: *const u8) -> AtaResult<()> {
        self.get_selected_drive()
            .ok_or(AtaError::DeviceNotFound)
            .and_then(|d| PioIo::write(d, start_sector, nbr_sectors, buf))
    }

    /// Get the drive pointed by Rank, or else return None
    fn get_selected_drive(&self) -> Option<&Drive> {
        match self.selected_drive? {
            Rank::Primary(Hierarchy::Master) => self.primary_master.as_ref(),
            Rank::Primary(Hierarchy::Slave) => self.primary_slave.as_ref(),
            Rank::Secondary(Hierarchy::Master) => self.secondary_master.as_ref(),
            Rank::Secondary(Hierarchy::Slave) => self.secondary_slave.as_ref(),
        }
    }
}

/// Emit Out Of Bound when a bound problem occured
fn check_bounds(start_sector: Sector, nbr_sectors: NbrSectors, drive_capacity: NbrSectors) -> AtaResult<()> {
    // 0 sector meens nothing for an human interface
    if nbr_sectors == NbrSectors(0) {
        Err(AtaError::NothingToDo)
    // Be careful with logical overflow
    } else if start_sector.0 as u64 > u64::max_value() as u64 - nbr_sectors.0 as u64 {
        Err(AtaError::OutOfBound)
    // raide disk capacity
    } else if start_sector.0 + nbr_sectors.0 as u64 > drive_capacity.0 {
        Err(AtaError::OutOfBound)
    } else {
        Ok(())
    }
}

/// Rank
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Rank {
    Primary(Hierarchy),
    Secondary(Hierarchy),
}

/// Is it a Slave or a Master ?
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Hierarchy {
    Master,
    Slave,
}

/// Disk access capabilities
#[derive(Debug, Copy, Clone)]
enum Capabilities {
    Lba48,
    Lba28,
    Chs,
}

// Necessary to set some advanced features
bitflags! {
    struct DeviceControlRegister: u8 {
        const NIEN = 1 << 1; // Set this to stop the current device from sending interrupts.
        const SRST = 1 << 2; // Set, then clear (after 5us), this to do a "Software Reset" on all ATA drives on a bus, if one is misbehaving.
        const HOB = 1 << 7; // Set this to read back the High Order Byte of the last LBA48 value sent to an IO port.
    }
}

/// 0x01F0-0x01F7 The primary ATA hard-disk controller. 0x03F6-0x03F7 The control register, pop on IRQ14,
/// 0x0170-0x0177 The secondary ATA hard-disk controller. 0x0376-0x0377 The control register, pop on IRQ15
impl Drive {
    /// *** These below constants are expressed with offset from base register ***
    /// Data Register: Read/Write PIO data bytes. (read/write) (16-bit / 16-bit)
    const DATA: u16 = 0x0;

    /// Error Register: Used to retrieve any error generated by the last ATA command executed. (read) (8-bit / 16-bit)
    /// Features Register: Used to control command specific interface features. (write) (8-bit / 16-bit)
    const ERROR: u16 = 0x1;
    const FEATURES: u16 = 0x1;

    /// Sector Count Register:  Number of sectors to read/write (0 is a special value). (read/write) (8-bit / 16-bit)
    const SECTOR_COUNT: u16 = 0x2;

    /// Sector Number Register or LBA low. (read/write) (8-bit / 16-bit)
    const L1_SECTOR: u16 = 0x3;

    /// Cylinder Low Register or LBA mid. (read/write) (8-bit / 16-bit)
    const L2_CYLINDER: u16 = 0x4;

    /// Cylinder High Register or LBA high. (read/write) (8-bit / 16-bit)
    const L3_CYLINDER: u16 = 0x5;

    /// Drive / Head Register: Used to select a drive and/or head. Supports extra address/flag bits. (read/write) (8-bit / 8-bit)
    const SELECTOR: u16 = 0x6;

    /// Status Register: Used to read the current status. (read) (8-bit / 8-bit)
    /// Command Register:  Used to send ATA commands to the device. (write) (8-bit / 8-bit)
    const STATUS: u16 = 0x7;
    const COMMAND: u16 = 0x7;

    /// *** These below constants are expressed with offset from control register ***
    /// A duplicate of the Status Register which does not affect interrupts. (read) (8-bit / 8-bit)
    /// Used to reset the bus or enable/disable interrupts. (write) (8-bit / 8-bit)
    const ALTERNATE_STATUS: u16 = 0x0;
    const DEVICE_CONTROL: u16 = 0x0;

    /// Provides drive select and head select information. (read) (8-bit / 8-bit)
    const _DRIVE_ADDRESS: u16 = 0x1;

    /// Check if the selected IDE device is present, return characteristics if it is
    pub fn identify(rank: Rank, command_register: u16, control_register: u16) -> Option<Drive> {
        let target = match rank {
            Rank::Primary(Hierarchy::Master) => 0xA0,
            Rank::Primary(Hierarchy::Slave) => 0xB0,
            Rank::Secondary(Hierarchy::Master) => 0xA0,
            Rank::Secondary(Hierarchy::Slave) => 0xB0,
        };

        // select a target drive by sending 0xA0 for the master drive, or 0xB0 for the slave
        Pio::<u8>::new(command_register + Self::SELECTOR).write(target);

        // set the Sectorcount, LBAlo, LBAmid, and LBAhi IO ports to 0
        Pio::<u8>::new(command_register + Self::SECTOR_COUNT).write(0);
        Pio::<u8>::new(command_register + Self::L1_SECTOR).write(0);
        Pio::<u8>::new(command_register + Self::L2_CYLINDER).write(0);
        Pio::<u8>::new(command_register + Self::L3_CYLINDER).write(0);

        // send the IDENTIFY command (0xEC) to the Command IO port (0x1F7)
        Pio::<u8>::new(command_register + Self::COMMAND).write(Command::AtaCmdIdentify as u8);

        // read the Status port (0x1F7). If the value read is 0, the drive does not exist
        if Pio::<u8>::new(command_register + Self::STATUS).read() == 0 {
            return None;
        }

        // For any other value: poll the Status port (0x1F7) until bit 7 (BSY, value = 0x80) clears
        while (StatusRegister::from_bits_truncate(Pio::<u8>::new(command_register + Self::STATUS).read()))
            .contains(StatusRegister::BSY)
        {}

        // Continue polling one of the Status ports until bit 3 (DRQ, value = 8) sets, or until bit 0 (ERR, value = 1) sets.
        while !(StatusRegister::from_bits_truncate(Pio::<u8>::new(command_register + Self::STATUS).read()))
            .intersects(StatusRegister::ERR | StatusRegister::DRQ)
        {}

        // If ERR is set, it is a failure
        if (StatusRegister::from_bits_truncate(Pio::<u8>::new(command_register + Self::STATUS).read()))
            .contains(StatusRegister::ERR)
        {
            eprintln!(
                "unexpected error while polling status of {:?} err: {:?}",
                rank,
                ErrorRegister::from_bits_truncate(Pio::<u8>::new(command_register + Self::ERROR).read())
            );
            return None;
        }

        // if ERR is clear, the data is ready to read from the Data port (0x1F0). Read 256 16-bit values, and store them.
        let mut v = Vec::new();

        for _i in 0..256 {
            v.push(Pio::<u16>::new(command_register + Self::DATA).read());
        }

        // Bit 10 is set if the drive supports LBA48 mode.
        // 100 through 103 taken as a uint64_t contain the total number of 48 bit addressable sectors on the drive. (Probably also proof that LBA48 is supported.)
        if v[83] & (1 << 10) != 0 {
            Some(Drive {
                capabilities: Capabilities::Lba48,
                sector_capacity: NbrSectors(
                    v[100] as u64 + ((v[101] as u64) << 16) + ((v[102] as u64) << 32) + ((v[103] as u64) << 48),
                ),
                // The bits in the low byte tell you the supported UDMA modes, the upper byte tells you which UDMA mode is active.
                udma_support: v[88],
                command_register,
                control_register,
                rank,
            })
        // 60 & 61 taken as a uint32_t contain the total number of 28 bit LBA addressable sectors on the drive. (If non-zero, the drive supports LBA28.)
        } else if v[60] != 0 || v[61] != 0 {
            Some(Drive {
                capabilities: Capabilities::Lba28,
                sector_capacity: NbrSectors(v[60] as u64 + ((v[61] as u64) << 16)),
                udma_support: v[88],
                command_register,
                control_register,
                rank,
            })
        } else {
            Some(Drive {
                capabilities: Capabilities::Chs,
                sector_capacity: NbrSectors(0),
                udma_support: v[88],
                command_register,
                control_register,
                rank,
            })
        }
    }
}

/// AtaResult is just made to handle module errors
pub type AtaResult<T> = core::result::Result<T, AtaError>;

/// Common errors for this module
#[derive(Debug, Copy, Clone)]
pub enum AtaError {
    /// Not a valid position
    DeviceNotFound,
    /// Common error Variant
    NotSupported,
    /// Out of bound like always
    OutOfBound,
    /// There is nothing to do
    NothingToDo,
    /// IO error
    IoError,
}

// Some errors may occured
bitflags! {
    struct ErrorRegister: u8 {
        const ADDRESS_MARK_NOT_FOUND = 1 << 0;
        const TRACK_ZERO_NOT_FOUND = 1 << 1;
        const ABORTED_COMMAND = 1 << 2;
        const MEDIA_CHANGE_REQUEST = 1 << 3;
        const ID_MOT_FOUND = 1 << 4;
        const MEDIA_CHANGED = 1 << 5;
        const UNCORRECTABLE_DATA_ERROR = 1 << 6;
        const BAD_BLOCK_DETECTED = 1 << 7;
    }
}

// We need always check status register
bitflags! {
    struct StatusRegister: u8 {
        const ERR = 1 << 0; // Indicates an error occurred. Send a new command to clear it (or nuke it with a Software Reset).
        const IDX = 1 << 1; // Index. Always set to zero.
        const CORR = 1 << 2; // Corrected data. Always set to zero.
        const DRQ = 1 << 3; // Set when the drive has PIO data to transfer, or is ready to accept PIO data.
        const SRV = 1 << 4; // Overlapped Mode Service Request.
        const DF = 1 << 5; // Drive Fault Error (does not set ERR).
        const RDY = 1 << 6; // Bit is clear when drive is spun down, or after an error. Set otherwise.
        const BSY = 1 << 7; //Indicates the drive is preparing to send/receive data (wait for it to clear). In case of 'hang' (it never clears), do a software reset
    }
}

#[allow(dead_code)]
#[repr(u8)]
enum Command {
    AtaCmdReadPio = 0x20,
    AtaCmdReadPioExt = 0x24,
    AtaCmdReadDma = 0xC8,
    AtaCmdReadDmaExt = 0x25,
    AtaCmdWritePio = 0x30,
    AtaCmdWritePioExt = 0x34,
    AtaCmdWriteDma = 0xCA,
    AtaCmdWriteDmaExt = 0x35,
    AtaCmdCacheFlush = 0xE7,
    AtaCmdCacheFlushExt = 0xEA,
    AtaCmdPacket = 0xA0,
    AtaCmdIdentifyPacket = 0xA1,
    AtaCmdIdentify = 0xEC,
}
