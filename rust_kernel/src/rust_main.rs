use crate::memory::tools::DeviceMap;
use crate::multiboot::MultibootInfo;

#[cfg(not(feature = "test"))]
#[no_mangle]
pub extern "C" fn kmain(
    multiboot_info: *const MultibootInfo,
    device_map_ptr: *const DeviceMap,
) -> ! {
    init_kernel(multiboot_info, device_map_ptr);
    #[cfg(feature = "with-login")]
    crate::taskmaster::start(
        "/bin/init",
        &["/bin/init", "/bin/session_manager", "/bin/login"],
        &[],
    );
    #[cfg(not(feature = "with-login"))]
    crate::taskmaster::start(
        "/bin/init",
        &["/bin/init", "/bin/session_manager", "-"],
        &["HOME=/root", "SHELL=/bin/sh"],
    );
}

use ansi_escape_code::color::Colored;
use screen::{Drawer, SCREEN_MONAD};

use crate::drivers::pit_8253::OperatingMode;
use crate::drivers::{Acpi, ACPI, PCI, PIC_8259, PIT0};
use crate::memory::init_memory_system;
use crate::memory::tools::device_map::get_device_map_slice;
use crate::system::init_idt;
use crate::terminal::init_terminal;
use crate::watch_dog;

/// Kernel Initialization
pub fn init_kernel(multiboot_info: *const MultibootInfo, device_map_ptr: *const DeviceMap) {
    #[cfg(feature = "serial-eprintln")]
    {
        unsafe { terminal::uart_16550::UART_16550.init() };
        eprintln!("you are in serial eprintln mode");
    }
    let multiboot_info: MultibootInfo = unsafe { *multiboot_info };

    /*
     * Enable CPU_ISR and memory system
     */
    unsafe {
        interrupts::disable();
        init_idt();

        let device_map = get_device_map_slice(device_map_ptr);
        init_memory_system(multiboot_info.get_memory_amount_nb_pages(), device_map)
            .expect("init memory system failed");
    }

    /*
     * Initialize output
     */
    SCREEN_MONAD.lock().switch_graphic_mode(0x118).unwrap();
    init_terminal();

    let size = SCREEN_MONAD.lock().query_window_size();
    printfixed!(
        Pos {
            line: 1,
            column: size.column - 17
        },
        "{}",
        "Turbo Fish v10.0".green()
    );

    /*
     * Initialize Pic8259 and base PIT0 drivers
     */
    unsafe {
        PIC_8259.lock().init();

        PIT0.lock().configure(OperatingMode::RateGenerator);
        PIT0.lock().start_at_frequency(1000.).unwrap();
        log::info!("PIT FREQUENCY: {:?} hz", PIT0.lock().get_frequency());

        PIC_8259.lock().enable_irq(irq::Irq::SystemTimer, None);

        watch_dog();
        interrupts::enable();
    }

    /*
     * Initialize ACPI driver
     */
    match Acpi::init() {
        Ok(()) => match ACPI.lock().expect("acpi init failed").enable() {
            Ok(()) => log::info!("ACPI driver initialized"),
            Err(e) => log::error!("Cannot initialize ACPI: {:?}", e),
        },
        Err(e) => log::error!("Cannot initialize ACPI: {:?}", e),
    };

    /*
     * Initialize PCI driver
     */
    log::info!("Scanning PCI buses ...");
    PCI.lock().scan_pci_buses();
    PCI.lock().list_pci_devices();
    log::info!("PCI buses has been scanned");

    // crate::test_helpers::really_lazy_hello_world(Duration::from_millis(100));

    // let mut rtc = Rtc::new();
    // rtc.update_unix_time();
    // let date = rtc.read_date();
    // rtc.enable_periodic_interrupts(15); // lowest possible frequency for RTC = 2 Hz.
    watch_dog();

    crate::drivers::storage::init(&multiboot_info);
}
