//! This module contains the turbo fish's ATA/IDE drivers

#[deny(missing_docs)]

use super::SECTOR_SIZE;
use super::{IdeControllerProgIf, MassStorageControllerSubClass, PciDeviceClass, PciType0, PCI};

use crate::drivers::storage::tools::*;

pub mod pio_polling;

pub mod pci_udma;
pub use pci_udma::PciUdma;

use bitflags::bitflags;

/// Global structure
#[derive(Debug, Copy, Clone, Default)]
pub struct IdeAtaController {
    primary_master: Option<Drive>,
    secondary_master: Option<Drive>,
    primary_slave: Option<Drive>,
    secondary_slave: Option<Drive>,

    selected_drive: Option<Rank>,
}

/// Standard port location, if they are different, probe IDE controller in PCI driver
const PRIMARY_BASE_REGISTER: u16 = 0x01F0;
const SECONDARY_BASE_REGISTER: u16 = 0x0170;
const PRIMARY_CONTROL_REGISTER: u16 = 0x03f6;
const SECONDARY_CONTROL_REGISTER: u16 = 0x376;

impl IdeAtaController {
    /// Invocation of a new PioMode-IDE controller
    pub fn new() -> Self {
        Self {
            primary_master: Drive::identify(
                Rank::Primary(Hierarchy::Master),
                PRIMARY_BASE_REGISTER,
                PRIMARY_CONTROL_REGISTER,
            ),
            secondary_master: Drive::identify(
                Rank::Primary(Hierarchy::Master),
                SECONDARY_BASE_REGISTER,
                SECONDARY_CONTROL_REGISTER,
            ),
            primary_slave: Drive::identify(
                Rank::Primary(Hierarchy::Slave),
                PRIMARY_BASE_REGISTER,
                PRIMARY_CONTROL_REGISTER,
            ),
            secondary_slave: Drive::identify(
                Rank::Primary(Hierarchy::Slave),
                SECONDARY_BASE_REGISTER,
                SECONDARY_CONTROL_REGISTER,
            ),
            selected_drive: None,
        }
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
        self.get_selected_drive().ok_or(AtaError::DeviceNotFound).and_then(|d| d.read(start_sector, nbr_sectors, buf))
    }

    /// Write nbr_sectors after start_sector location from the buf
    pub fn write(&self, start_sector: Sector, nbr_sectors: NbrSectors, buf: *const u8) -> AtaResult<()> {
        self.get_selected_drive().ok_or(AtaError::DeviceNotFound).and_then(|d| d.write(start_sector, nbr_sectors, buf))
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
/// Global disk characteristics
#[derive(Debug, Copy, Clone)]
struct Drive {
    command_register: u16,
    control_register: u16,
    capabilities: Capabilities,
    sector_capacity: NbrSectors,
    udma_support: u16,
    rank: Rank,
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
