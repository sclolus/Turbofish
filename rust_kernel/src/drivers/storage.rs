use super::pci;
use super::pci::{PciType0, PCI};

pub mod dummy_ata;
pub use dummy_ata::DummyAta;

pub mod ide_controller;
pub use ide_controller::IdeController;

pub mod sata_controller;
pub use sata_controller::SataController;
