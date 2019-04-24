//! This module contains the turbo fish's storage drivers
#[deny(missing_docs)]
use super::pci::{
    IdeControllerProgIf, MassStorageControllerSubClass, PciCommand, PciDeviceClass, PciType0, SerialAtaProgIf, PCI,
};

const SECTOR_SIZE: usize = 512;

pub mod ide_ata_controller;
pub use ide_ata_controller::IdeAtaController;

pub mod sata_controller;
pub use sata_controller::SataController;

pub mod bios_int13h;
pub use bios_int13h::BiosInt13h;

pub mod tools;
pub use tools::{NbrSectors, Sector};

pub type DiskResult<T> = core::result::Result<T, DiskError>;

#[derive(Debug, Copy, Clone)]
pub enum DiskError {
    OutOfBound,
    InternalError,
    NotSupported,
    NothingToDo,
    IOError,
}
