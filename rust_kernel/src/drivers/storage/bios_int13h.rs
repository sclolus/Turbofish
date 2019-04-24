//! this module provide dummy low level bios IO operations

use super::SECTOR_SIZE;
use super::{DiskError, DiskResult};
use super::{NbrSectors, Sector};

use crate::system::i8086_payload;
use crate::system::BaseRegisters;

use bit_field::BitField;
use bitflags::bitflags;

use core::slice;

extern "C" {
    static payload_13h_check_extension_present: extern "C" fn();
    static payload_13h_check_extension_present_len: usize;
    static payload_13h_extended_read_drive_parameters: extern "C" fn();
    static payload_13h_extended_read_drive_parameters_len: usize;
    static payload_13h_extended_read_sectors: extern "C" fn();
    static payload_13h_extended_read_sectors_len: usize;
    static payload_13h_extended_write_sectors: extern "C" fn();
    static payload_13h_extended_write_sectors_len: usize;
}

/// Main Structure
#[derive(Debug, Copy, Clone)]
pub struct BiosInt13h {
    /// Device where bios boot into. (80h, 81h ...)
    boot_device: u8,
    /// List of supported mode by the interface
    interface_support: InterfaceSupport,
    /// Number of sector of the drive
    nb_sector: NbrSectors,
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

/// Packed structure used by rea and write operations
#[derive(Debug, Copy, Clone)]
#[repr(packed)] // Representation packed is mandatory for this kind of structure
struct Dap {
    /// size of DAP (set this to 10h)
    size_of_dap: u8,
    /// unused, should be zero
    unused: u8,
    /// number of sectors to be read, (some Phoenix BIOSes are limited to a maximum of 127 sectors)
    nb_sectors: u16,
    /// segment:offset pointer to the memory buffer to which sectors will be transferred
    memory: u32,
    /// absolute number of the start of the sectors to be read (1st sector of drive has number 0) using logical block addressing.
    sector: u64,
}

impl BiosInt13h {
    const N_SECTOR: usize = 128; // Max sector capacity in one buffer chunk
    const DAP_LOCATION: usize = 0x80000; // Correspond to real addr 0x8000:0000
    const CHUNK_SIZE: usize = SECTOR_SIZE * Self::N_SECTOR; // Correspond to 64ko
    const REAL_BUFFER_LOCATION: u32 = 0x90000000; // expressed as segment/offset, correspond to 0x90000 in 32bits
    const BUFFER_LOCATION: u32 = 0x90000;

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
        let mut p: *mut DriveParameters = Self::DAP_LOCATION as *mut _;
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

        let (nb_sector, sector_size) = unsafe { (NbrSectors((*p).nb_sector), (*p).bytes_per_sector) };

        // Sector size != 512 is very difficult to manage in our kernel
        if sector_size != SECTOR_SIZE as u16 {
            return Err(DiskError::NotSupported);
        }

        // Return the main constructor
        Ok(Self { boot_device, interface_support, nb_sector, sector_size })
    }

    /// Read nbr_sectors after start_sector location and write it into the buf
    pub fn read(&self, start_sector: Sector, nbr_sectors: NbrSectors, buf: *mut u8) -> DiskResult<()> {
        check_bounds(start_sector, nbr_sectors, self.nb_sector)?;

        let s = unsafe { slice::from_raw_parts_mut(buf, nbr_sectors.into()) };

        for (i, chunk) in s.chunks_mut(Self::CHUNK_SIZE).enumerate() {
            let sectors_to_read: NbrSectors = chunk.len().into();

            // Initalize a new DAP packet
            let mut dap: *mut Dap = Self::DAP_LOCATION as *mut _;
            unsafe {
                (*dap).size_of_dap = 0x10;
                (*dap).unused = 0;
                (*dap).nb_sectors = sectors_to_read.0 as u16;
                (*dap).memory = Self::REAL_BUFFER_LOCATION;
                (*dap).sector = start_sector.0 + (i * Self::N_SECTOR) as u64;
            }

            // Command a read from disk to DAP buffer
            let mut reg: BaseRegisters = BaseRegisters { ..Default::default() };
            reg.edx = self.boot_device as u32;
            let ret = unsafe {
                i8086_payload(
                    &mut reg as *mut BaseRegisters,
                    &payload_13h_extended_read_sectors,
                    payload_13h_extended_read_sectors_len,
                )
            };

            // Check for error
            if ret == -1 {
                return Err(DiskError::IOError);
            }

            // Copy DAP buffer into buf
            let p: *mut u8 = Self::BUFFER_LOCATION as *mut u8;
            for (i, elem) in chunk.iter_mut().enumerate() {
                *elem = unsafe { *(p.add(i)) };
            }
        }
        Ok(())
    }

    /// Write nbr_sectors after start_sector location from the buf
    pub fn write(&self, start_sector: Sector, nbr_sectors: NbrSectors, buf: *const u8) -> DiskResult<()> {
        check_bounds(start_sector, nbr_sectors, self.nb_sector)?;

        let s = unsafe { slice::from_raw_parts(buf, nbr_sectors.into()) };

        for (i, chunk) in s.chunks(Self::CHUNK_SIZE).enumerate() {
            let sectors_to_write: NbrSectors = chunk.len().into();

            // Initalize a new DAP packet
            let mut dap: *mut Dap = Self::DAP_LOCATION as *mut _;
            unsafe {
                (*dap).size_of_dap = 0x10;
                (*dap).unused = 0;
                (*dap).nb_sectors = sectors_to_write.0 as u16;
                (*dap).memory = Self::REAL_BUFFER_LOCATION;
                (*dap).sector = start_sector.0 + (i * Self::N_SECTOR) as u64;
            }

            // Copy 'sectors_to_write' datas from 'buf' to DAP buffer
            let p: *mut u8 = Self::BUFFER_LOCATION as *mut u8;
            for (i, elem) in chunk.iter().enumerate() {
                unsafe {
                    *(p.add(i)) = *elem;
                }
            }

            // Command a write to disk
            let mut reg: BaseRegisters = BaseRegisters { ..Default::default() };
            reg.edx = self.boot_device as u32;
            let ret = unsafe {
                i8086_payload(
                    &mut reg as *mut BaseRegisters,
                    &payload_13h_extended_write_sectors,
                    payload_13h_extended_write_sectors_len,
                )
            };

            // Check for error
            if ret == -1 {
                return Err(DiskError::IOError);
            }
        }
        Ok(())
    }
}

/// Emit Out Of Bound when a bound problem occured
fn check_bounds(start_sector: Sector, nbr_sectors: NbrSectors, drive_capacity: NbrSectors) -> DiskResult<()> {
    // 0 sector meens nothing for an human interface
    if nbr_sectors == NbrSectors(0) {
        Err(DiskError::NothingToDo)
    // Be careful with logical overflow
    } else if start_sector.0 as u64 > u64::max_value() as u64 - nbr_sectors.0 as u64 {
        Err(DiskError::OutOfBound)
    // raide disk capacity
    } else if start_sector.0 + nbr_sectors.0 as u64 > drive_capacity.0 {
        Err(DiskError::OutOfBound)
    } else {
        Ok(())
    }
}
