use crate::drivers::pit_8253::OperatingMode;
// use crate::drivers::{pic_8259, Acpi, ACPI, PCI, PIC_8259, PIT0};
use crate::drivers::{pic_8259, Acpi, ACPI, PIC_8259, PIT0};

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
use crate::timer::Rtc;
use crate::watch_dog;
use core::time::Duration;

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

    // TODO: Find why it crashs in Sclolus Qemu version
    // log::info!("Scanning PCI buses ...");
    // PCI.lock().scan_pci_buses();
    // log::info!("PCI buses has been scanned");

    crate::test_helpers::really_lazy_hello_world(Duration::from_millis(100));

    let mut rtc = Rtc::new();
    log::info!("RTC system seems to be working perfectly");
    let date = rtc.read_date();
    println!("{}", date);

    log::error!("this is an example of error");

    watch_dog();

    crate::drivers::storage::init(&multiboot_info);

    use crate::taskmaster::{Process, TaskOrigin, UserProcess};
    // Load some processes into the scheduler
    let user_process_list = unsafe {
        vec![
            UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/shell")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/shell")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Raw(&_dummy_asm_process_code_a, _dummy_asm_process_len_a)).unwrap(),
            // UserProcess::new(TaskOrigin::Raw(&_dummy_asm_process_code_b, _dummy_asm_process_len_b)).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/richard")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/vincent")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/fork_fucker")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/fork_me_baby")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/prempt_me")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/prempt_me")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/prempt_me")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/fork_fucker")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/stack_overflow")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/sys_stack_overflow")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/mordak")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/mordak")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/mordak")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/fork_bomb")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/WaitChildDieBefore")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/WaitChildDieAfter")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/sleepers")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/sleepers")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/Timer")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/ConnectionlessSimpleTest")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/ConnectionOrientedSimpleTest")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/DummyRead")[..])).unwrap(),
            /*
             * Signal tests
             */
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/SegFault")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/Ud2")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/Csignal")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/SonKillFather")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/RecursiveSignal")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/recursive_signal_no_defer")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/SaRestart")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/NoSaRestart")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/SaRestartMultiple")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/NoSaRestartMultiple")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/Continue")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/SignalSimple")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/SignalSimpleDuo")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/SignalSimpleDuoRecurse")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/SignalSimpleStopContinue")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/SignalStopContinueOverload")[..])).unwrap(),
            // UserProcess::new(TaskOrigin::Elf(&include_bytes!("userland/Clone")[..])).unwrap(),
        ]
    };
    crate::taskmaster::start(user_process_list);
}
