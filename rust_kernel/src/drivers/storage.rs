//! This module contains the turbo fish's storage drivers

use super::pci::{IdeControllerProgIf, MassStorageControllerSubClass, PciDeviceClass, PciType0, SerialAtaProgIf, PCI};

const SECTOR_SIZE: usize = 512;

pub mod ide_ata_controller;
pub use ide_ata_controller::{IdeAtaController, PciUdma};

pub mod tools;
pub use tools::*;
// TODO -> MUST WILL BE:
// pub use ide_ata_controller::IdeAtaController;

pub mod sata_controller;
pub use sata_controller::SataController;
