//! this module provide dummy low level bios IO operations

// use super::Sector;
use super::SECTOR_SIZE;
use super::{DiskError, DiskResult};

use crate::system::i8086_payload;
use crate::system::BaseRegisters;

use bit_field::BitField;
use bitflags::bitflags;

extern "C" {
    static payload_13h_check_extension_present: extern "C" fn();
    static payload_13h_check_extension_present_len: usize;
    static payload_13h_extended_read_drive_parameters: extern "C" fn();
    static payload_13h_extended_read_drive_parameters_len: usize;
// static payload_13h_extended_read_sectors: extern "C" fn();
// static payload_13h_extended_read_sectors_len: usize;
// static payload_13h_extended_write_sectors: extern "C" fn();
// static payload_13h_extended_write_sectors_len: usize;
}

/// Main Structure
#[derive(Debug, Copy, Clone)]
pub struct BiosInt13h {
    /// Device where bios boot into. (80h, 81h ...)
    boot_device: u8,
    /// List of supported mode by the interface
    interface_support: InterfaceSupport,
    /// Number of sector of the drive
    nb_sector: u64,
    /// Size of one sector
    sector_size: u16,
}

// Check extension result boilerplate
bitflags! {
    struct InterfaceSupport: u16 {
        const DAP = 1 << 0;              // Device Access using the packet structure
        const LOCK_AND_EJECT = 1 << 1;   // Drive Locking and Ejecting
        const EDD = 1 << 2;              // Enhanced Disk Drive Support (EDD)
    }
}

/// Packet sended and returned with payload_13h_extended_read_drive_parameters
#[derive(Debug, Copy, Clone)]
#[repr(packed)] // Representation packed is mandatory for this kind of structure
struct DriveParameters {
    /// size of Result Buffer (set this to 1Eh)
    result_size: u16,
    /// information flags
    info_flag: u16,
    /// physical number of cylinders = last index + 1 (because index starts with 0)
    cylinders: u32,
    /// physical number of heads = last index + 1 (because index starts with 0)
    heads: u32,
    /// physical number of sectors per track = last index (because index starts with 1)
    sectors: u32,
    /// absolute number of sectors = last index + 1 (because index starts with 0)
    nb_sector: u64,
    /// bytes per sector
    bytes_per_sector: u16,
    /// optional pointer to Enhanced Disk Drive (EDD) configuration parameters which may be used for subsequent interrupt 13h Extension calls (if supported)
    option_ptr: u32,
}

const DAP_LOCATION: usize = 0x80000; // correspond to real addr 0x8000:0000

impl BiosInt13h {
    /// Public invocation of a new BiosInt13h instance
    pub fn new(boot_device: u8) -> DiskResult<Self> {
        // Check if BIOS extension of int 13h is present
        let mut reg: BaseRegisters = BaseRegisters { ..Default::default() };
        reg.edx = boot_device as u32;

        let ret = unsafe {
            i8086_payload(
                &mut reg as *mut BaseRegisters,
                &payload_13h_check_extension_present,
                payload_13h_check_extension_present_len,
            )
        };

        if ret == -1 {
            return Err(DiskError::NotSupported);
        }

        // Check if interface support DAP (Device Access using the packet structure)
        let interface_support = InterfaceSupport { bits: reg.ecx.get_bits(0..16) as u16 };
        if !interface_support.contains(InterfaceSupport::DAP) {
            return Err(DiskError::NotSupported);
        }

        // Get device characteristics
        let mut reg: BaseRegisters = BaseRegisters { ..Default::default() };
        let mut p: *mut DriveParameters = DAP_LOCATION as *mut _;
        unsafe {
            (*p).result_size = 0x1E;
        }
        reg.edx = boot_device as u32;
        let ret = unsafe {
            i8086_payload(
                &mut reg as *mut BaseRegisters,
                &payload_13h_extended_read_drive_parameters,
                payload_13h_extended_read_drive_parameters_len,
            )
        };
        if ret == -1 {
            return Err(DiskError::NotSupported);
        }

        let (nb_sector, sector_size) = unsafe { ((*p).nb_sector, (*p).bytes_per_sector) };

        // Sector size != 512 is very difficult to manage in our kernel
        if sector_size != SECTOR_SIZE as u16 {
            return Err(DiskError::NotSupported);
        }

        // Return the main constructor
        Ok(Self { boot_device, interface_support, nb_sector, sector_size })
    }
}
