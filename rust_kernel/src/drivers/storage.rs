//! This module contains the turbo fish's storage drivers
#[deny(missing_docs)]
use super::pci::{
    IdeControllerProgIf, MassStorageControllerSubClass, PciCommand, PciDeviceClass, PciType0,
    SerialAtaProgIf, PCI,
};

pub const SECTOR_SIZE: usize = 512;

pub mod ide_ata_controller;
pub use ide_ata_controller::IdeAtaController;

pub mod sata_controller;
pub use sata_controller::SataController;

pub mod bios_int13h;
pub use bios_int13h::{BiosInt13h, BIOS_INT13H};

pub mod tools;
pub use tools::{NbrSectors, Sector};

pub mod ext2;

use crate::multiboot::MultibootInfo;
use ide_ata_controller::{Hierarchy, Rank};

pub type DiskResult<T> = core::result::Result<T, DiskError>;

#[derive(Debug, Copy, Clone)]
pub enum DiskError {
    OutOfBound,
    InternalError,
    NotSupported,
    NothingToDo,
    IOError,
}

#[derive(Debug, Copy, Clone)]
pub enum DiskDriverType {
    Sata,
    Ide,
    Bios,
}

/// A trait for block devices which can read a write by blocks of
/// Sector
pub trait BlockIo {
    fn read(&self, start_sector: Sector, nbr_sectors: NbrSectors, buf: *mut u8) -> DiskResult<()>;
    fn write(
        &self,
        start_sector: Sector,
        nbr_sectors: NbrSectors,
        buf: *const u8,
    ) -> DiskResult<()>;
}

pub fn init(multiboot_info: &MultibootInfo) {
    // Intialize SATA controller
    // loop {}
    match SataController::init() {
        Some(sata_controller) => {
            log::info!("Sata Controller detected: {:#X?}", sata_controller);
            sata_controller.dump_hba();
        }
        None => {}
    }

    // Initialize IDE controller
    let mut disk = IdeAtaController::new();

    if let Some(d) = disk.as_mut() {
        match d.select_drive(Rank::Primary(Hierarchy::Master)) {
            Ok(drive) => {
                log::info!("Ide Controller detected: Selecting drive -> {:#X?}", drive);
            }
            Err(_) => {}
        }
    }

    // Initialize BIOS controller
    unsafe {
        bios_int13h::init((multiboot_info.boot_device >> 24) as u8)
            .expect("bios_int_13 init failed");
    }
}
