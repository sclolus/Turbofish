use crate::drivers::pit_8253::OperatingMode;
use crate::drivers::{pic_8259, Acpi, ACPI, PCI, PIC_8259, PIT0};
use crate::interrupts;
use crate::keyboard::init_keyboard_driver;
use crate::memory;
use crate::memory::tools::device_map::get_device_map_slice;
use crate::memory::tools::DeviceMap;
use crate::multiboot::MultibootInfo;
use crate::process::scheduler::Scheduler;
use crate::syscall;
use crate::terminal::ansi_escape_code::color::Colored;
use crate::terminal::init_terminal;
use crate::terminal::monitor::Drawer;
use crate::terminal::monitor::SCREEN_MONAD;
use crate::timer::Rtc;
use crate::watch_dog;
use core::time::Duration;

use crate::registers::Eflags;
use crate::system::BaseRegisters;

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

        watch_dog();
        interrupts::enable();

        let device_map = get_device_map_slice(device_map_ptr);
        memory::init_memory_system(multiboot_info.get_memory_amount_nb_pages(), device_map).unwrap();
    }
    SCREEN_MONAD.lock().switch_graphic_mode(0x118).unwrap();
    init_terminal();
    println!("TTY system initialized");

    PIT0.lock().configure(OperatingMode::RateGenerator);
    PIT0.lock().start_at_frequency(1000.).unwrap();

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

    // Initialize Syscall system
    syscall::init();

    // Initialize the TSS segment: TODO: What about DS/ES/FS/GS segments AND premptivity ?
    use crate::process::tss::Tss;
    let _t = unsafe { Tss::init(&kernel_stack as *const u8 as u32, 0x18) };
    Tss::display();

    unsafe {
        Scheduler::start();
    }

    use crate::process::Process;

    // Create an entire C dummy process
    let p1 = unsafe { Process::new(&dummy_c_process, 4096) };
    println!("{:#X?}", p1);

    // Create an entire ASM dummy process
    let p2 = unsafe { Process::new(&_dummy_asm_process_code, _dummy_asm_process_len) };
    println!("{:#X?}", p2);

    let selected_process = &p1;

    // Switch to process Page Directory
    unsafe {
        selected_process.virtual_allocator.context_switch();
    }

    // Switch to ring 3
    // user SS segment is defined as 0x30
    // user CS segment is defined as 0x20
    // user DATA segment is defined as 0x28
    unsafe {
        _launch_process(
            0x30 + 3,
            selected_process.cpu_state.esp,
            0x20 + 3,
            selected_process.cpu_state.eip,
            0x28 + 3,
            selected_process.cpu_state.eflags,
            &selected_process.cpu_state.registers,
        );
    }

    crate::shell::shell();
    loop {}
}

// scheduler::init();

extern "C" {
    static _dummy_asm_process_code: u8;
    static _dummy_asm_process_len: usize;

    static dummy_c_process: u8;

    static kernel_stack: u8;

    fn _launch_process(
        ss: u16,
        esp: u32,
        cs: u16,
        eip: u32,
        data_segment: u32,
        eflags: Eflags,
        registers: *const BaseRegisters,
    );
}
