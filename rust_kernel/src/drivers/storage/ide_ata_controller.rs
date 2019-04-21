//! This module contains the turbo fish's ATA/IDE drivers, See https://wiki.osdev.org/PCI_IDE_Controller

// TODO: Disable interrupts if only PIO mode is possible (enable it id UDMA)
pub mod pci_udma;
pub mod pio_polling;

mod udma;
use udma::{Channel, Udma};

use super::SECTOR_SIZE;
use super::{IdeControllerProgIf, MassStorageControllerSubClass, PciCommand, PciDeviceClass, PciType0, PCI};
use super::{NbrSectors, Sector};

use alloc::vec::Vec;
use io::{Io, Pio};

use bit_field::BitField;
use bitflags::bitflags;

use crate::drivers::PIT0;
use core::time::Duration;

/// Global structure
#[derive(Debug, Clone)]
pub struct IdeAtaController {
    primary_master: Option<Drive>,
    secondary_master: Option<Drive>,
    primary_slave: Option<Drive>,
    secondary_slave: Option<Drive>,
    selected_drive: Option<Rank>,
    pci: PciType0,
    pci_location: u32,
    udma_primary: Option<Udma>,
    udma_secondary: Option<Udma>,
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

/// Since data are copied into DMA for read, DmaIo copy DMA buff into buf address passed
pub trait DmaIo {
    fn read(&self, start_sector: Sector, nbr_sectors: NbrSectors, buf: *mut u8, udma: &mut Udma) -> AtaResult<()>;
    fn write(&self, start_sector: Sector, nbr_sectors: NbrSectors, buf: *const u8, udma: &mut Udma) -> AtaResult<()>;
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

impl IdeAtaController {
    /// Invocation of a new PioMode-IDE controller
    pub fn new() -> Option<Self> {
        // Search a specific IDE controller 'IsaCompatibilityModeOnlyControllerBusMastered'
        let (pci, pci_location) = PCI.lock().query_device::<PciType0>(PciDeviceClass::MassStorageController(
            MassStorageControllerSubClass::IdeController(
                IdeControllerProgIf::IsaCompatibilityModeOnlyControllerBusMastered,
            ),
        ))?;

        // Become the BUS MASTER, it is very important on QEMU since it does not do it for us (give little tempos)
        PIT0.lock().sleep(Duration::from_millis(40));

        pci.set_command(PciCommand::BUS_MASTER, true, pci_location);

        PIT0.lock().sleep(Duration::from_millis(40));

        eprintln!("current pci status: {:#?}", pci.get_status(pci_location));

        // Get primary and secondary IO ports (0 or 1 means ide default port values_
        let primary_base_register =
            if pci.bar0 == 0 || pci.bar0 == 1 { PRIMARY_BASE_REGISTER } else { pci.bar0 as u16 };
        let primary_control_register =
            if pci.bar1 == 0 || pci.bar1 == 1 { PRIMARY_CONTROL_REGISTER } else { pci.bar1 as u16 };
        let secondary_base_register =
            if pci.bar2 == 0 || pci.bar2 == 1 { SECONDARY_BASE_REGISTER } else { pci.bar2 as u16 };
        let secondary_control_register =
            if pci.bar3 == 0 || pci.bar3 == 1 { SECONDARY_CONTROL_REGISTER } else { pci.bar3 as u16 };

        // DMA port is contained inside BAR 4 of the PCI device
        let dma_port = pci.bar4 as u16;

        // Construct all objects
        Some(Self {
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
            udma_primary: if dma_port != 0 { Some(Udma::init(dma_port, Channel::Primary)) } else { None },
            udma_secondary: if dma_port != 0 { Some(Udma::init(dma_port + 8, Channel::Secondary)) } else { None },
        })
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
    pub fn read(&mut self, start_sector: Sector, nbr_sectors: NbrSectors, buf: *mut u8) -> AtaResult<()> {
        let (drive, udma) = match self.selected_drive.ok_or(AtaError::DeviceNotFound)? {
            Rank::Primary(h) => (
                match h {
                    Hierarchy::Master => self.primary_master.as_ref(),
                    Hierarchy::Slave => self.primary_slave.as_ref(),
                },
                self.udma_primary.as_mut(),
            ),
            Rank::Secondary(h) => (
                match h {
                    Hierarchy::Master => self.secondary_master.as_ref(),
                    Hierarchy::Slave => self.secondary_slave.as_ref(),
                },
                self.udma_secondary.as_mut(),
            ),
        };
        drive.ok_or(AtaError::DeviceNotFound).and_then(|d| match udma {
            Some(u) => DmaIo::read(d, start_sector, nbr_sectors, buf, u),
            None => PioIo::read(d, start_sector, nbr_sectors, buf),
        })
    }

    /// Write nbr_sectors after start_sector location from the buf
    pub fn write(&mut self, start_sector: Sector, nbr_sectors: NbrSectors, buf: *const u8) -> AtaResult<()> {
        let (drive, udma) = match self.selected_drive.ok_or(AtaError::DeviceNotFound)? {
            Rank::Primary(h) => (
                match h {
                    Hierarchy::Master => self.primary_master.as_ref(),
                    Hierarchy::Slave => self.primary_slave.as_ref(),
                },
                self.udma_primary.as_mut(),
            ),
            Rank::Secondary(h) => (
                match h {
                    Hierarchy::Master => self.secondary_master.as_ref(),
                    Hierarchy::Slave => self.secondary_slave.as_ref(),
                },
                self.udma_secondary.as_mut(),
            ),
        };
        drive.ok_or(AtaError::DeviceNotFound).and_then(|d| match udma {
            Some(u) => DmaIo::write(d, start_sector, nbr_sectors, buf, u),
            None => PioIo::write(d, start_sector, nbr_sectors, buf),
        })
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

#[derive(Debug, Copy, Clone, PartialEq)]
/// Rank
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
    const _DEVICE_CONTROL: u16 = 0x0;

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

    /// Select the drive for future read and write operations
    pub fn select_drive(&self) {
        self.wait_available();
        match self.get_hierarchy() {
            // select a target drive by sending 0xA0 for the master drive, or 0xB0 for the slave
            // I dont think it is necessary or really true
            Hierarchy::Master => Pio::<u8>::new(self.command_register + Self::SELECTOR).write(0xA0),
            Hierarchy::Slave => Pio::<u8>::new(self.command_register + Self::SELECTOR).write(0xB0),
        };
        // Disable interruot bit for the selected drive
        // Pio::<u8>::new(self.control_register + Self::DEVICE_CONTROL).write(DeviceControlRegister::NIEN.bits());
    }

    /// The method suggested in the ATA specs for sending ATA commands tells you to check the BSY and DRQ bits before trying to send a command
    fn wait_available(&self) {
        // Continue polling one of the Status ports until bit 3 (DRQ, value = 8) sets, or until bit 0 (BSY, value = 7) sets.
        while StatusRegister::from_bits_truncate(Pio::<u8>::new(self.control_register + Self::ALTERNATE_STATUS).read())
            .intersects(StatusRegister::BSY | StatusRegister::DRQ)
        {}
    }

    /// Extract the sub tag hierarchy from rank
    fn get_hierarchy(&self) -> Hierarchy {
        match self.rank {
            Rank::Primary(h) | Rank::Secondary(h) => h,
        }
    }

    /// Init read or write sequence for lba48 mode
    fn init_lba48(&self, start_sector: Sector, nbr_sectors: NbrSectors) {
        let lba_low = start_sector.0.get_bits(0..32) as u32;
        let lba_high = start_sector.0.get_bits(32..48) as u32;

        // Send 0x40 for the "master" or 0x50 for the "slave" to port 0x1F6: outb(0x1F6, 0x40 | (slavebit << 4))
        self.wait_available();
        match self.get_hierarchy() {
            Hierarchy::Master => Pio::<u8>::new(self.command_register + Self::SELECTOR).write(0x40),
            Hierarchy::Slave => Pio::<u8>::new(self.command_register + Self::SELECTOR).write(0x50),
        }

        // Outb (0x1F2, sectorcount high byte)
        Pio::<u8>::new(self.command_register + Self::SECTOR_COUNT).write(nbr_sectors.0.get_bits(8..16) as u8);

        // LBA 4
        Pio::<u8>::new(self.command_register + Self::L1_SECTOR).write(lba_low.get_bits(24..32) as u8);
        // LBA 5
        Pio::<u8>::new(self.command_register + Self::L2_CYLINDER).write(lba_high.get_bits(0..8) as u8);
        // LBA 6
        Pio::<u8>::new(self.command_register + Self::L3_CYLINDER).write(lba_high.get_bits(8..16) as u8);

        // outb (0x1F2, sectorcount low byte)
        Pio::<u8>::new(self.command_register + Self::SECTOR_COUNT).write(nbr_sectors.0.get_bits(0..8) as u8);

        // LBA 1
        Pio::<u8>::new(self.command_register + Self::L1_SECTOR).write(lba_low.get_bits(0..8) as u8);
        // LBA 2
        Pio::<u8>::new(self.command_register + Self::L2_CYLINDER).write(lba_low.get_bits(8..16) as u8);
        // LBA 3
        Pio::<u8>::new(self.command_register + Self::L3_CYLINDER).write(lba_low.get_bits(16..24) as u8);
    }

    /// Init read or write sequence for lba28 mode
    fn init_lba28(&self, start_sector: Sector, nbr_sectors: NbrSectors) {
        let lba_low = start_sector.0.get_bits(0..32) as u32;

        // Send 0xE0 for the "master" or 0xF0 for the "slave" to port 0x1F6
        // and add the highest 4 bits of the LBA to port 0x1F6: outb(0x1F6, 0xE0 | (slavebit << 4) | ((LBA >> 24) & 0x0F))
        self.wait_available();
        match self.get_hierarchy() {
            Hierarchy::Master => {
                Pio::<u8>::new(self.command_register + Self::SELECTOR).write(0xE0 | ((lba_low >> 24) & 0xF) as u8)
            }
            Hierarchy::Slave => {
                Pio::<u8>::new(self.command_register + Self::SELECTOR).write(0xF0 | ((lba_low >> 24) & 0xF) as u8)
            }
        }

        // Send a NULL byte to port 0x1F1, if you like (it is ignored and wastes lots of CPU time): outb(0x1F1, 0x00)
        Pio::<u8>::new(self.command_register + Self::FEATURES).write(0);

        // outb (0x1F2, sectorcount low byte)
        Pio::<u8>::new(self.command_register + Self::SECTOR_COUNT).write(nbr_sectors.0.get_bits(0..8) as u8);

        // LBA 1
        Pio::<u8>::new(self.command_register + Self::L1_SECTOR).write(lba_low.get_bits(0..8) as u8);
        // LBA 2
        Pio::<u8>::new(self.command_register + Self::L2_CYLINDER).write(lba_low.get_bits(8..16) as u8);
        // LBA 3
        Pio::<u8>::new(self.command_register + Self::L3_CYLINDER).write(lba_low.get_bits(16..24) as u8);
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
