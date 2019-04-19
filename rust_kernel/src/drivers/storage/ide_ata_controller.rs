//! This module contains the turbo fish's ATA/IDE drivers

use super::SECTOR_SIZE;
use super::{IdeControllerProgIf, MassStorageControllerSubClass, PciDeviceClass, PciType0, PCI};

pub mod pio_polling;
pub use pio_polling::PioPolling;

pub mod pci_udma;
pub use pci_udma::PciUdma;
