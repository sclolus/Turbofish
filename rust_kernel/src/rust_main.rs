use crate::drivers::pit_8253::OperatingMode;
use crate::drivers::{pic_8259, Acpi, ACPI, PCI, PIC_8259, PIT0};

use crate::drivers::storage::ide_ata_controller::{Hierarchy, IdeAtaController, Rank};
use crate::drivers::storage::{BiosInt13h, NbrSectors, SataController, Sector};

use crate::interrupts;
use crate::keyboard::init_keyboard_driver;
use crate::memory;
use crate::memory::tools::DeviceMap;
use crate::multiboot::MultibootInfo;
use crate::shell::shell;
use crate::syscall;
use crate::terminal::ansi_escape_code::color::Colored;
use crate::terminal::init_terminal;
use crate::terminal::monitor::Drawer;
use crate::terminal::monitor::SCREEN_MONAD;
use crate::timer::Rtc;
use crate::watch_dog;
use core::time::Duration;

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
        init_keyboard_driver();

        PIT0.lock().configure(OperatingMode::RateGenerator);
        PIT0.lock().start_at_frequency(1000.).unwrap();

        watch_dog();
        interrupts::enable();

        let device_map = get_device_map_slice(device_map_ptr);
        memory::init_memory_system(multiboot_info.get_memory_amount_nb_pages(), device_map).unwrap();
    }
    SCREEN_MONAD.lock().switch_graphic_mode(0x118).unwrap();
    init_terminal();
    println!("TTY system initialized");

    match Acpi::init() {
        Ok(()) => match ACPI.lock().unwrap().enable() {
            Ok(()) => log::info!("ACPI driver initialized"),
            Err(e) => log::error!("Cannot initialize ACPI: {:?}", e),
        },
        Err(e) => log::error!("Cannot initialize ACPI: {:?}", e),
    };

    unsafe {
        PIC_8259.lock().enable_irq(pic_8259::Irq::KeyboardController); // enable only the keyboard.
    }
    log::info!("Keyboard has been initialized: IRQ mask: {:X?}", PIC_8259.lock().get_masks());

    let size = SCREEN_MONAD.lock().query_window_size();
    printfixed!(Pos { line: 1, column: size.column - 17 }, "{}", "Turbo Fish v0.3".green());

    log::info!("Scanning PCI buses ...");
    PCI.lock().scan_pci_buses();
    log::info!("PCI buses has been scanned");

    crate::test_helpers::really_lazy_hello_world(Duration::from_millis(100));

    let mut rtc = Rtc::new();
    log::info!("RTC system seems to be working perfectly");
    let date = rtc.read_date();
    println!("{}", date);
    use crate::memory::allocator::VirtualPageAllocator;
    let mut v = unsafe { VirtualPageAllocator::new_for_process() };
    unsafe {
        v.context_switch();
    }
    use crate::memory::tools::*;
    println!("before alloc");

    let addr = v.alloc(NbrPages::_1MB, AllocFlags::USER_MEMORY).unwrap().to_addr().0 as *mut u8;

    let slice = unsafe { core::slice::from_raw_parts_mut(addr, NbrPages::_1MB.into()) };
    for i in slice.iter_mut() {
        *i = 42;
    }
    println!("processus address allocated: {:x?}", addr);
    let addr = v.alloc(NbrPages::_1MB, AllocFlags::USER_MEMORY).unwrap().to_addr().0 as *mut u8;
    println!("processus address allocated: {:x?}", addr);

    log::error!("this is an example of error");

    watch_dog();

    syscall::init();

    match SataController::init() {
        Some(sata_controller) => {
            println!("{:#X?}", sata_controller);
            sata_controller.dump_hba();
        }
        None => {}
    }

    let s = "write that";
    unsafe {
        crate::syscall::_write(1, s.as_ptr(), s.len());
    }

    let mut disk = IdeAtaController::new();

    println!("{:#X?}", disk);
    if let Some(d) = disk.as_mut() {
        match d.select_drive(Rank::Primary(Hierarchy::Master)) {
            Ok(drive) => {
                println!("Selecting drive: {:#X?}", drive);

                use alloc::vec;
                use alloc::vec::Vec;

                let size_read = NbrSectors(1);
                let mut v1: Vec<u8> = vec![0; size_read.into()];
                d.read(Sector(0x0), size_read, v1.as_mut_ptr()).unwrap();

                let size_read = NbrSectors(1);
                let mut v1: Vec<u8> = vec![0; size_read.into()];
                d.read(Sector(0x0), size_read, v1.as_mut_ptr()).unwrap();

                let size_read = NbrSectors(1);
                let mut v1: Vec<u8> = vec![0; size_read.into()];
                d.read(Sector(0x0), size_read, v1.as_mut_ptr()).unwrap();
            }
            Err(_) => {}
        }
    }

    let b = BiosInt13h::new((multiboot_info.boot_device >> 24) as u8);
    dbg_hex!(b.unwrap());

    use alloc::vec;
    use alloc::vec::Vec;
    use mbr::Mbr;

    let size_read = NbrSectors(1);
    let mut v1: Vec<u8> = vec![0; size_read.into()];
    b.unwrap().read(Sector(0x0), size_read, v1.as_mut_ptr()).unwrap();

    let mut a = [0; 512];
    for (i, elem) in a.iter_mut().enumerate() {
        *elem = v1[i];
    }
    unsafe {
        dbg!(Mbr::new(&a));
    }

    shell();
    0
}
