use crate::drivers::pit_8253;
use crate::drivers::{PCI, PIC_8259, PIT0};

use crate::interrupts;
use crate::math::random::{srand, srand_init};
use crate::memory;
use crate::memory::tools::DeviceMap;
use crate::multiboot::MultibootInfo;
use crate::tests::helpers::exit_qemu;

use crate::drivers::storage::BiosInt13h;
use crate::drivers::storage::{NbrSectors, Sector};

const NB_TESTS: usize = 64;
const DISK_SECTOR_CAPACITY: u16 = 0x8000;
const SECTOR_SIZE: u64 = 512;

#[no_mangle]
pub extern "C" fn kmain(multiboot_info: *const MultibootInfo, device_map_ptr: *const DeviceMap) -> u32 {
    #[cfg(feature = "serial-eprintln")]
    {
        unsafe { crate::drivers::UART_16550.init() };
        eprintln!("you are in serial eprintln mode");
    }
    let multiboot_info: MultibootInfo = unsafe { *multiboot_info };

    unsafe {
        interrupts::init();
        PIC_8259.lock().init();
        PIC_8259.lock().disable_all_irqs();

        PIT0.lock().configure(pit_8253::OperatingMode::RateGenerator);
        PIT0.lock().start_at_frequency(1000.).unwrap();

        crate::watch_dog();
        interrupts::enable();

        let device_map = crate::memory::tools::get_device_map_slice(device_map_ptr);
        memory::init_memory_system(multiboot_info.get_memory_amount_nb_pages(), device_map).unwrap();
    }

    log::info!("Scanning PCI buses ...");
    PCI.lock().scan_pci_buses();
    log::info!("PCI buses has been scanned");

    crate::watch_dog();

    srand_init(42).unwrap();

    // 0x81 is the second hard disk for a basic bios
    let d = BiosInt13h::new(0x81);

    println!("{:#X?}", d);

    use alloc::vec;
    use alloc::vec::Vec;

    for nb_test in 0..NB_TESTS {
        let start_sector = Sector(srand::<u16>(DISK_SECTOR_CAPACITY - 1) as u64);
        let mut n = srand::<u16>(1024) as u64;
        if start_sector.0 + n > DISK_SECTOR_CAPACITY as u64 {
            n = DISK_SECTOR_CAPACITY as u64 - start_sector.0;
        }
        let nbr_sectors = NbrSectors(n);

        let mut v: Vec<u32> = vec![0; n as usize * SECTOR_SIZE as usize / 4];
        eprintln!("read at {:?} for {:?}", start_sector, nbr_sectors);
        d.unwrap().read(start_sector, nbr_sectors, v.as_mut_ptr() as *mut u8).unwrap();

        for (j, i) in
            (start_sector.0 * SECTOR_SIZE..(start_sector.0 + nbr_sectors.0) * SECTOR_SIZE).step_by(4).enumerate()
        {
            if v[j] != i as u32 {
                panic!("expected: {:?}, readen {:?} at offset {:?} on test {:?}", i, v[j], j, nb_test);
            }
        }
    }
    crate::watch_dog();
    exit_qemu(0);
    0
}
