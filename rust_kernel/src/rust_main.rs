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
use crate::watch_dog;

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

        memory::init_memory_system(multiboot_info.get_memory_amount_nb_pages(), device_map_ptr).unwrap();
    }
    SCREEN_MONAD.lock().switch_graphic_mode(0x118).unwrap();

    init_terminal();
    println!("TTY system initialized");

    unsafe {
        PIC_8259.lock().enable_irq(pic_8259::Irq::KeyboardController); // enable only the keyboard.
    }
    log::info!("Keyboard has been initialized: IRQ mask: {:X?}", PIC_8259.lock().get_masks());

    let size = SCREEN_MONAD.lock().query_window_size();
    printfixed!(Pos { line: 1, column: size.column - 17 }, "{}", "Turbo Fish v0.2+".green());

    log::info!("Scanning PCI buses ...");
    PCI.lock().scan_pci_buses();
    log::info!("PCI buses has been scanned");

    crate::test_helpers::really_lazy_hello_world();

    let mut rtc = Rtc::new();
    log::info!("RTC system seems to be working perfectly");
    let date = rtc.read_date();
    println!("{}", date);

    watch_dog();
    shell();
    0
}
