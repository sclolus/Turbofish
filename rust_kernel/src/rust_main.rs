use crate::drivers::pci::PCI;
use crate::drivers::pit_8253::{OperatingMode, PIT0};
use crate::drivers::{pic_8259, PIC_8259};
use crate::interrupts;
use crate::keyboard::init_keyboard_driver;
use crate::memory;
use crate::memory::tools::DeviceMap;
use crate::multiboot::MultibootInfo;
use crate::shell::shell;
use crate::terminal::ansi_escape_code::color::Colored;
use crate::terminal::init_terminal;
use crate::terminal::monitor::Drawer;
use crate::terminal::monitor::SCREEN_MONAD;
use crate::timer::Rtc;
use crate::watch_dog;
use alloc::boxed::Box;
// use alloc::vec::Vec;
use core::time::Duration;
use interrupts::interrupt_manager::*;

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

    unsafe {
        INTERRUPT_MANAGER = Some(Manager::new().unwrap());
        let interrupt_manager = INTERRUPT_MANAGER.as_mut().unwrap();

        interrupt_manager.register(Box::new(GenericManager::new()), 12 + 32).unwrap();
        let handler: FnHandler = FnHandler::new(Box::new(|num| {
            println!("In interrupt context: {}", num);
            HandlingState::NotHandled
        }));

        interrupt_manager.register(Box::new(handler), 12 + 32).unwrap();

        extern "C" {
            pub fn _isr_timer_handler();
        }

        let pit_handler = FnHandler::new(Box::new(|_num| {
            PIC_8259.lock().send_eoi(pic_8259::Irq::MouseOnPS2Controller);
            log::info!("Successfully reached this handler");
            HandlingState::Handled
        }));
        interrupt_manager.register(Box::new(pit_handler), 12 + 32).unwrap();
    }
    unsafe {
        PIC_8259.lock().enable_irq(pic_8259::Irq::KeyboardController); // enable only the keyboard.
        PIC_8259.lock().enable_irq(pic_8259::Irq::SystemTimer);
        PIC_8259.lock().enable_irq(pic_8259::Irq::MouseOnPS2Controller);
    }
    log::info!("Keyboard has been initialized: IRQ mask: {:X?}", PIC_8259.lock().get_masks());

    let size = SCREEN_MONAD.lock().query_window_size();
    printfixed!(Pos { line: 1, column: size.column - 17 }, "{}", "Turbo Fish v0.2+".green());
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
    unsafe {
        asm!("int 44":::: "volatile", "intel");
    }
    shell();
    0
}
