use crate::drivers::pit_8253::OperatingMode;
use crate::drivers::{pic_8259, Acpi, ACPI, PCI, PIC_8259, PIT0};
use crate::interrupts;
use crate::keyboard::init_keyboard_driver;
use crate::memory;
use crate::memory::tools::DeviceMap;
use crate::multiboot::MultibootInfo;
use crate::process::scheduler;
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
        PIT0.lock().start_at_frequency(100.).unwrap();
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

    let s = "write that";
    unsafe {
        crate::syscall::_user_write(1, s.as_ptr(), s.len());
    }

    scheduler::init();
    // shell();
    loop {}
    0
}
