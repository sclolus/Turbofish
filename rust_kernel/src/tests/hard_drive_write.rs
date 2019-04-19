use crate::drivers::UART_16550;
use crate::interrupts;
use crate::math::random::{srand, srand_init};
use crate::memory;
use crate::memory::tools::DeviceMap;
use crate::multiboot::MultibootInfo;
use crate::tests::helpers::exit_qemu;

use crate::drivers::storage::ata_pio::{Hierarchy, NbrSectors, Rank, Sector};
use crate::drivers::storage::AtaPio;

const NB_TESTS: usize = 32;
const DISK_SECTOR_CAPACITY: u16 = 0x8000;
const SECTOR_SIZE: u64 = 512;

#[no_mangle]
pub extern "C" fn kmain(multiboot_info: *const MultibootInfo, device_map_ptr: *const DeviceMap) -> u32 {
    unsafe {
        UART_16550.init();
    }
    let multiboot_info: MultibootInfo = unsafe { *multiboot_info };
    unsafe {
        interrupts::init();
    }
    crate::watch_dog();
    unsafe {
        let device_map = crate::memory::tools::get_device_map_slice(device_map_ptr);
        memory::init_memory_system(multiboot_info.get_memory_amount_nb_pages(), device_map).unwrap();
    }
    crate::watch_dog();

    srand_init(42).unwrap();

    let mut disk = AtaPio::new();

    eprintln!("{:#X?}", disk);
    eprintln!("Selecting drive: {:#X?}", disk.select_drive(Rank::Primary(Hierarchy::Slave)));

    use alloc::vec;
    use alloc::vec::Vec;

    for _i in 0..NB_TESTS {
        let start_sector = Sector(srand::<u16>(DISK_SECTOR_CAPACITY - 1) as u64);
        let mut n = srand::<u16>(1024) as u64;
        if start_sector.0 + n > DISK_SECTOR_CAPACITY as u64 {
            n = DISK_SECTOR_CAPACITY as u64 - start_sector.0;
        }
        let nbr_sectors = NbrSectors(n);

        let r = srand::<u8>(255);

        let src: Vec<u8> = vec![r; n as usize * SECTOR_SIZE as usize];
        disk.write(start_sector, nbr_sectors, src.as_ptr()).unwrap();

        let mut dst: Vec<u8> = vec![0; n as usize * SECTOR_SIZE as usize];
        disk.read(start_sector, nbr_sectors, dst.as_mut_ptr()).unwrap();

        for i in 0..src.len() {
            assert_eq!(src[i], dst[i]);
        }
    }
    crate::watch_dog();
    exit_qemu(0);
    0
}
