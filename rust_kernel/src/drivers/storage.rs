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
pub use bios_int13h::{BiosInt13h, BIOS_INT13H};

pub mod tools;
pub use tools::{NbrSectors, Sector};

pub mod ext2;

pub mod vfs;

use crate::multiboot::MultibootInfo;
use alloc::vec;
use alloc::vec::Vec;
use ide_ata_controller::{Hierarchy, Rank};
use mbr::Mbr;

pub type DiskResult<T> = core::result::Result<T, DiskError>;

#[derive(Debug, Copy, Clone)]
pub enum DiskError {
    OutOfBound,
    InternalError,
    NotSupported,
    NothingToDo,
    IOError,
}

pub fn init(multiboot_info: &MultibootInfo) {
    vfs::init();
    loop {}
    match SataController::init() {
        Some(sata_controller) => {
            println!("{:#X?}", sata_controller);
            sata_controller.dump_hba();
        }
        None => {}
    }

    let mut disk = IdeAtaController::new();

    println!("{:#X?}", disk);
    if let Some(d) = disk.as_mut() {
        match d.select_drive(Rank::Primary(Hierarchy::Master)) {
            Ok(drive) => {
                println!("Selecting drive: {:#X?}", drive);

                let size_read = NbrSectors(1);
                let mut v1: Vec<u8> = vec![0; size_read.into()];
                d.read(Sector(0x0), size_read, v1.as_mut_ptr()).expect("read ide failed");

                let size_read = NbrSectors(1);
                let mut v1: Vec<u8> = vec![0; size_read.into()];
                d.read(Sector(0x0), size_read, v1.as_mut_ptr()).expect("read ide failed");

                let size_read = NbrSectors(1);
                let mut v1: Vec<u8> = vec![0; size_read.into()];
                d.read(Sector(0x0), size_read, v1.as_mut_ptr()).expect("read ide failed");
            }
            Err(_) => {}
        }
    }

    unsafe {
        bios_int13h::init((multiboot_info.boot_device >> 24) as u8).expect("bios_int_13 init failed");
    }

    let size_read = NbrSectors(1);
    let mut v1: Vec<u8> = vec![0; size_read.into()];
    unsafe {
        BIOS_INT13H.as_mut().unwrap().read(Sector(0x0), size_read, v1.as_mut_ptr()).expect("bios read failed");
    }

    let mut a = [0; 512];
    for (i, elem) in a.iter_mut().enumerate() {
        *elem = v1[i];
    }
    let mbr = unsafe { Mbr::new(&a) };
    ext2::init(&mbr).expect("init ext2 failed");


}
