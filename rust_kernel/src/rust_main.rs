use crate::debug;
use crate::drivers::pci::PCI;
use crate::drivers::pit_8253::{OperatingMode, PIT0};
use crate::drivers::{pic_8259, PIC_8259};
use crate::interrupts;
use crate::keyboard::init_keyboard_driver;
use crate::memory;
use crate::memory::allocator::physical_page_allocator::DeviceMap;
use crate::multiboot::MultibootInfo;
use crate::shell::shell;
use crate::terminal::ansi_escape_code::color::Colored;
use crate::terminal::init_terminal;
use crate::terminal::monitor::Drawer;
use crate::terminal::monitor::SCREEN_MONAD;
use crate::timer::Rtc;

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

        crate::watch_dog();
        interrupts::enable();

        memory::init_memory_system(multiboot_info.get_memory_amount_nb_pages(), device_map_ptr).unwrap();
    }
    println!("device map ptr {:#?}", device_map_ptr);
    // set_text_color!(Color::Red);
    println!("multiboot_infos {:#?}", multiboot_info);
    // set_text_color!(Color::Green);
    dbg!(multiboot_info.mem_lower);
    dbg!(multiboot_info.mem_upper);

    SCREEN_MONAD.lock().switch_graphic_mode(0x118).unwrap();
    init_terminal();
    println!("{} INIT {}", "AFTER", "TERMINAL");

    unsafe {
        PIC_8259.lock().enable_irq(pic_8259::Irq::KeyboardController); // enable only the keyboard.
    }
    let size = SCREEN_MONAD.lock().query_window_size();
    printfixed!(Pos { line: 1, column: size.column - 17 }, "{}", "Turbo Fish v0.2+".green());
    debug::bench_start();
    let t = debug::bench_end();
    println!("{:?} ms ellapsed", t);

    println!("from {}", function!());

    println!("irqs state: {}", interrupts::get_interrupts_state());

    println!("irq mask: {:b}", PIC_8259.lock().get_masks());

    let eflags = crate::registers::Eflags::get_eflags();
    println!("{:x?}", eflags);

    PIT0.lock().start_at_frequency(1000.).unwrap();

    PCI.lock().scan_pci_buses();
    PCI.lock().list_pci_devices();

    crate::test_helpers::really_lazy_hello_world();

    let mut rtc = Rtc::new();
    let date = rtc.read_date();
    println!("{}", date);

    use alloc::vec;
    use alloc::vec::Vec;

    println!("begin alloc test...");
    debug::bench_start();
    let mut sum: u32 = 0;
    for i in 0..2 {
        let v: Vec<u8> = vec![(i & 0xff) as u8; 4096 * 16];
        sum += v[0] as u32;
        drop(v);
    }
    let t = debug::bench_end();
    println!("{:?} ms ellapsed !", t);

    println!("{:?}", device_map_ptr);

    crate::watch_dog();
    log::trace!("a trace");
    log::warn!("a warning");
    log::error!("a error");
    println!("{} {}", "bonjour 1", "bonjour2");
    crate::test_helpers::fish();
    crate::test_helpers::fish2();
    shell();
    sum
}
