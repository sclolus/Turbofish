use crate::drivers::pit_8253::OperatingMode;
use crate::drivers::{pic_8259, Acpi, ACPI, PCI, PIC_8259, PIT0};
use crate::interrupts;
use crate::keyboard::init_keyboard_driver;
use crate::memory;
use crate::memory::tools::device_map::get_device_map_slice;
use crate::memory::tools::DeviceMap;
use crate::multiboot::MultibootInfo;
use crate::process::scheduler;
use crate::syscall;
use crate::terminal::ansi_escape_code::color::Colored;
use crate::terminal::init_terminal;
use crate::terminal::monitor::Drawer;
use crate::terminal::monitor::SCREEN_MONAD;
use crate::timer::Rtc;
use crate::watch_dog;
use core::time::Duration;

#[no_mangle]
pub extern "C" fn kmain(multiboot_info: *const MultibootInfo, device_map_ptr: *const DeviceMap) -> ! {
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

    log::error!("this is an example of error");

    watch_dog();

    // Ceate a Dummy process Page directory
    use crate::memory::allocator::VirtualPageAllocator;
    let mut v = unsafe { VirtualPageAllocator::new_for_process() };
    unsafe {
        v.context_switch();
    }

    use crate::memory::tools::{AllocFlags, NbrPages};

    // Allocate one page for code segment of the Dummy process
    let addr = v.alloc(NbrPages::_1MB, AllocFlags::USER_MEMORY).unwrap().to_addr().0 as *mut u8;
    println!("processus address allocated: {:x?}", addr);

    // Copy dummy code for the process
    unsafe {
        ft_memcpy(addr, &_dummy_process_code, _dummy_process_len);
    }

    // Check is data has been correctly copied
    let slice = unsafe { core::slice::from_raw_parts(addr, _dummy_process_len) };
    for i in slice.iter() {
        println!("{:#X?}", *i);
    }

    // Initialize the TSS segment: TODO: What about DS/ES/FS/GS segments ?
    use crate::process::tss::Tss;
    let _t = unsafe { Tss::init(&kernel_stack as *const u8 as u32, 0x10) };
    Tss::display();

    // Switch to ring 3
    unsafe {
        _ring3_switch(0x28 + 3, addr.add(4096) as u32, 0x20 + 3, addr as u32);
    }
    //loop {}

    crate::shell::shell();
    loop {}
}

extern "C" {
    fn ft_memcpy(dst: *mut u8, src: *const u8, len: usize);

    static _dummy_process_code: u8;
    static _dummy_process_len: usize;

    static kernel_stack: u8;

    fn _ring3_switch(ss: u16, esp: u32, cs: u16, eip: u32);
}


// syscall::init();
// scheduler::init();

// use crate::process::tss::Tss;
// let t = unsafe { Tss::init(0x42, 0x84) };
// Tss::display();
// unsafe {
//     (*t).reset(0x10, 0x20);
// }
// Tss::display();

// use crate::memory::allocator::VirtualPageAllocator;
// let v = unsafe { VirtualPageAllocator::new_for_process() };
// unsafe {
//    v.context_switch();
// }

// use crate::memory::tools::*;
// println!("before alloc");

// let addr = v.alloc(NbrPages::_1MB, AllocFlags::USER_MEMORY).unwrap().to_addr().0 as *mut u8;

// let slice = unsafe { core::slice::from_raw_parts_mut(addr, NbrPages::_1MB.into()) };
// for i in slice.iter_mut() {
//     *i = 42;
// }
// println!("processus address allocated: {:x?}", addr);
// let addr = v.alloc(NbrPages::_1MB, AllocFlags::USER_MEMORY).unwrap().to_addr().0 as *mut u8;
// println!("processus address allocated: {:x?}", addr);

// let s = "write that";
// unsafe {
//     crate::syscall::_user_write(1, s.as_ptr(), s.len());
// }
