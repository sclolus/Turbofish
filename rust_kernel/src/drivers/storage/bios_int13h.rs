//! this module provide dummy low level bios IO operations.
//! See https://en.wikipedia.org/wiki/INT_13H

use super::SECTOR_SIZE;
use super::{DiskError, DiskResult};
use super::{NbrSectors, Sector};

use crate::system::i8086_payload;
use crate::system::BaseRegisters;

use bit_field::BitField;
use bitflags::bitflags;
use const_assert::const_assert;

use core::slice;

/// This module call 16 bits payloads which contain interrupt 13h calls
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
    /// Version
    version: u16,
}
pub static mut BIOS_INT13H: Option<BiosInt13h> = None;

pub unsafe fn init(boot_device: u8) -> DiskResult<()> {
    BIOS_INT13H = Some(BiosInt13h::new(boot_device)?);
    Ok(())
}

// Check extension result
bitflags! {
    struct InterfaceSupport: u16 {
        const DAP = 1 << 0;              // Device Access using the packet structure
        const LOCK_AND_EJECT = 1 << 1;   // Drive Locking and Ejecting
        const EDD = 1 << 2;              // Enhanced Disk Drive Support (EDD)
    }
}

/// Packet sended and returned with payload_13h_extended_read_drive_parameters
#[derive(Debug, Copy, Clone)]
#[repr(packed)] // ! Representation packed is mandatory for this kind of structure !
struct DriveParameters {
    /// Size of Result Buffer (set this to 1Eh)
    result_size: u16,
    /// Information flags
    info_flag: u16,
    /// Physical number of cylinders = last index + 1 (because index starts with 0)
    cylinders: u32,
    /// Physical number of heads = last index + 1 (because index starts with 0)
    heads: u32,
    /// Physical number of sectors per track = last index (because index starts with 1)
    sectors: u32,
    /// Absolute number of sectors = last index + 1 (because index starts with 0)
    nb_sector: u64,
    /// Bytes per sector
    bytes_per_sector: u16,
    /// Optional pointer to Enhanced Disk Drive (EDD) configuration parameters which may be used for subsequent interrupt 13h Extension calls (if supported)
    option_ptr: u32,
}

/// Packed structure used by real and write operations
#[derive(Debug, Copy, Clone)]
#[repr(packed)] // ! Representation packed is mandatory for this kind of structure !
struct Dap {
    /// Size of DAP (set this to 10h)
    size_of_dap: u8,
    /// Unused, should be zero
    unused: u8,
    /// Number of sectors to be read, (some Phoenix BIOSes are limited to a maximum of 127 sectors)
    nb_sectors: u16,
    /// Segment:offset pointer to the memory buffer to which sectors will be transferred
    memory: u32,
    /// Absolute number of the start of the sectors to be read (1st sector of drive has number 0) using logical block addressing.
    sector: u64,
}

/// Location of the DAP
const DAP_LOCATION: usize = 0x80000; // Corresponds to real addr 0x8000:0000

// Physical description of region 0x8000:0000 (0x80000)
// 0x80000 +--- DAP ---+  ^
//         |     ^     |  | 16ko for DAP & STACK
//         |     |     |  |
//         |   STACK   |  v
// 0x84000 |BBBBBBBBBBB|  ^
//         |BBBBBBBBBBB|  |
//         |BBBBBBBBBBB|  |
//         |BBBBBBBBBBB|  |
// 0x88000 |BBBBBBBBBBB|  |
//         |BBBBBBBBBBB|  | 48ko for buffer
//         |BBBBBBBBBBB|  |
//         |BBBBBBBBBBB|  |
// 0x8C000 |BBBBBBBBBBB|  |
//         |BBBBBBBBBBB|  |
//         |BBBBBBBBBBB|  |
//         |BBBBBBBBBBB|  |
// 0x90000 +--BUF(end)-+  v

macro_rules! convert_to_real_address {
    ($expr:expr) => {{
        const_assert!($expr < 0x100_000);
        $expr << 12
    }};
}

/// This part define the buffer
const N_SECTOR: usize = 96; // Max sector capacity in one buffer chunk
const CHUNK_SIZE: usize = SECTOR_SIZE * N_SECTOR; // Corresponds to 48ko buffer
const REAL_BUFFER_LOCATION: u32 = convert_to_real_address!(BUFFER_LOCATION);
const BUFFER_LOCATION: u32 = 0x84000; // The buffer will be between 0x8000:4000 and 0x9000:0000

impl BiosInt13h {
    /// Public invocation of a new BiosInt13h instance
    pub fn new(boot_device: u8) -> DiskResult<Self> {
        // Check if BIOS extension of int 13h is present
        let mut reg: BaseRegisters = BaseRegisters {
            ..Default::default()
        };
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
        let version = reg.eax as u16;

        // Check if interface support DAP (Device Access using the packet structure)
        let interface_support = InterfaceSupport {
            bits: reg.ecx.get_bits(0..16) as u16,
        };
        if !interface_support.contains(InterfaceSupport::DAP) {
            return Err(DiskError::NotSupported);
        }

        // Get device characteristics
        let mut reg: BaseRegisters = BaseRegisters {
            ..Default::default()
        };
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

        let (nb_sector, sector_size) =
            unsafe { (NbrSectors((*p).nb_sector), (*p).bytes_per_sector) };

        // Sector size != 512 is very difficult to manage in our kernel, skip out !
        if sector_size != SECTOR_SIZE as u16 {
            return Err(DiskError::NotSupported);
        }

        // Returns the main constructor
        Ok(Self {
            boot_device,
            interface_support,
            nb_sector,
            sector_size,
            version,
        })
    }

    /// Read nbr_sectors after start_sector location and write it into the buf
    pub fn read(
        &self,
        start_sector: Sector,
        nbr_sectors: NbrSectors,
        buf: *mut u8,
    ) -> DiskResult<()> {
        check_bounds(start_sector, nbr_sectors, self.nb_sector)?;

        let s = unsafe { slice::from_raw_parts_mut(buf, nbr_sectors.into()) };

        for (i, chunk) in s.chunks_mut(CHUNK_SIZE).enumerate() {
            let sectors_to_read: NbrSectors = chunk.len().into();

            // Initalize a new DAP packet
            unsafe {
                create_dap(
                    Sector(start_sector.0 + (i * N_SECTOR) as u64),
                    sectors_to_read,
                    DAP_LOCATION,
                );
            }

            // Command a read from disk to DAP buffer
            let mut reg: BaseRegisters = BaseRegisters {
                ..Default::default()
            };
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
            let p: *mut u8 = BUFFER_LOCATION as *mut u8;
            for (i, elem) in chunk.iter_mut().enumerate() {
                *elem = unsafe { *(p.add(i)) };
            }
        }
        Ok(())
    }

    /// Write nbr_sectors after start_sector location from the buf
    pub fn write(
        &self,
        start_sector: Sector,
        nbr_sectors: NbrSectors,
        buf: *const u8,
    ) -> DiskResult<()> {
        check_bounds(start_sector, nbr_sectors, self.nb_sector)?;

        let s = unsafe { slice::from_raw_parts(buf, nbr_sectors.into()) };

        for (i, chunk) in s.chunks(CHUNK_SIZE).enumerate() {
            let sectors_to_write: NbrSectors = chunk.len().into();

            // Initalize a new DAP packet
            unsafe {
                create_dap(
                    Sector(start_sector.0 + (i * N_SECTOR) as u64),
                    sectors_to_write,
                    DAP_LOCATION,
                );
            }

            // Copy 'sectors_to_write' data from 'buf' to DAP buffer
            let p: *mut u8 = BUFFER_LOCATION as *mut u8;
            for (i, elem) in chunk.iter().enumerate() {
                unsafe {
                    *(p.add(i)) = *elem;
                }
            }

            // Command a write to disk
            let mut reg: BaseRegisters = BaseRegisters {
                ..Default::default()
            };
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
fn check_bounds(
    start_sector: Sector,
    nbr_sectors: NbrSectors,
    drive_capacity: NbrSectors,
) -> DiskResult<()> {
    // 0 sector meens nothing for an human interface
    if nbr_sectors == NbrSectors(0) {
        Err(DiskError::NothingToDo)
    // Be careful with logical overflow
    } else if start_sector.0 as u64 > u64::max_value() as u64 - nbr_sectors.0 as u64 {
        Err(DiskError::OutOfBound)
    // Raise disk capacity
    } else if start_sector.0 + nbr_sectors.0 as u64 > drive_capacity.0 {
        Err(DiskError::OutOfBound)
    } else {
        Ok(())
    }
}

/// Create a new DAP structure for write and read transactions
unsafe fn create_dap(start_sector: Sector, nb_sectors: NbrSectors, addr: usize) {
    let mut dap: *mut Dap = addr as *mut _;

    (*dap).size_of_dap = 0x10;
    (*dap).unused = 0;
    (*dap).nb_sectors = nb_sectors.0 as u16;
    (*dap).memory = REAL_BUFFER_LOCATION;
    (*dap).sector = start_sector.0;
}
