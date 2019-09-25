use crate::drivers::pit_8253::OperatingMode;
use crate::drivers::rtc::Rtc;
use crate::drivers::{pic_8259, Acpi, ACPI, PCI, PIC_8259, PIT0};
use crate::interrupts;
use crate::keyboard::init_keyboard_driver;
use crate::memory;
use crate::memory::tools::device_map::get_device_map_slice;
use crate::memory::tools::DeviceMap;
use crate::multiboot::MultibootInfo;
use crate::terminal::ansi_escape_code::color::Colored;
use crate::terminal::init_terminal;
use crate::terminal::monitor::Drawer;
use crate::terminal::monitor::SCREEN_MONAD;
use crate::watch_dog;
// use core::time::Duration;
use terminal::uart_16550::UART_16550;

#[no_mangle]
pub extern "C" fn kmain(
    multiboot_info: *const MultibootInfo,
    device_map_ptr: *const DeviceMap,
) -> ! {
    unsafe {
        UART_16550.init();
    }
    eprintln!("Launching of native_libc_test:");
    let multiboot_info: MultibootInfo = unsafe { *multiboot_info };

    unsafe {
        interrupts::init();
        PIC_8259.lock().init();
        PIC_8259.lock().disable_all_irqs();
        init_keyboard_driver();

        watch_dog();
        interrupts::enable();

        let device_map = get_device_map_slice(device_map_ptr);
        memory::init_memory_system(multiboot_info.get_memory_amount_nb_pages(), device_map)
            .expect("init memory system failed");
    }
    SCREEN_MONAD.lock().switch_graphic_mode(0x118).unwrap();
    init_terminal();
    println!("TTY system initialized");

    PIT0.lock().configure(OperatingMode::RateGenerator);
    PIT0.lock().start_at_frequency(1000.).unwrap();
    log::info!("PIT FREQUENCY: {:?} hz", PIT0.lock().get_frequency());

    match Acpi::init() {
        Ok(()) => match ACPI.lock().expect("acpi init failed").enable() {
            Ok(()) => log::info!("ACPI driver initialized"),
            Err(e) => log::error!("Cannot initialize ACPI: {:?}", e),
        },
        Err(e) => log::error!("Cannot initialize ACPI: {:?}", e),
    };

    unsafe {
        PIC_8259
            .lock()
            .enable_irq(pic_8259::Irq::KeyboardController); // enable only the keyboard.
    }
    log::info!(
        "Keyboard has been initialized: IRQ mask: {:X?}",
        PIC_8259.lock().get_masks()
    );

    let size = SCREEN_MONAD.lock().query_window_size();
    printfixed!(
        Pos {
            line: 1,
            column: size.column - 17
        },
        "{}",
        "Turbo Fish v0.3".green()
    );

    log::info!("Scanning PCI buses ...");
    PCI.lock().scan_pci_buses();
    log::info!("PCI buses has been scanned");

    // crate::test_helpers::really_lazy_hello_world(Duration::from_millis(100));

    let mut rtc = Rtc::new();
    log::info!("RTC system seems to be working perfectly");
    let date = rtc.read_date();
    println!("{}", date);

    log::error!("this is an example of error");

    watch_dog();

    crate::drivers::storage::init(&multiboot_info);

    eprintln!("Launching Taskmaster:");
    crate::taskmaster::start(
        "/bin/init",
        &["/bin/init", "/bin/MasterDeepThought", "/bin/DeepThought"],
        &[],
    );
}
