use crate::drivers::pit_8253::OperatingMode;
use crate::drivers::{pic_8259, Acpi, ACPI, PCI, PIC_8259, PIT0};

use crate::interrupts;
use crate::drivers::Rtc;
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

#[no_mangle]
pub extern "C" fn kmain(
    multiboot_info: *const MultibootInfo,
    device_map_ptr: *const DeviceMap,
) -> ! {
    #[cfg(feature = "serial-eprintln")]
    {
        unsafe { crate::terminal::UART_16550.init() };
        eprintln!("you are in serial eprintln mode");
    }
    let multiboot_info: MultibootInfo = unsafe { *multiboot_info };

    unsafe {
        let device_map = get_device_map_slice(device_map_ptr);
        memory::init_memory_system(multiboot_info.get_memory_amount_nb_pages(), device_map)
            .expect("init memory system failed");
    }
    SCREEN_MONAD.lock().switch_graphic_mode(0x118).unwrap();
    unsafe {
        interrupts::init();
        interrupts::disable();
        PIC_8259.lock().init();

        init_keyboard_driver();

        watch_dog();
        interrupts::enable();
    }
    init_terminal();

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

    // TODO: Find why it crashs in Sclolus Qemu version
    log::info!("Scanning PCI buses ...");
    PCI.lock().scan_pci_buses();
    log::info!("PCI buses has been scanned");

    // crate::test_helpers::really_lazy_hello_world(Duration::from_millis(100));

    let mut rtc = Rtc::new();
    log::info!("RTC system seems to be working perfectly");
    let date = rtc.read_date();
    log::info!("{}", date);
    rtc.enable_periodic_interrupts(15); // lowest possible frequency for RTC = 2 Hz.

    watch_dog();

    crate::drivers::storage::init(&multiboot_info);

    crate::taskmaster::start("/bin/init", &["/bin/init", "/bin/shell"], &[]);
}
