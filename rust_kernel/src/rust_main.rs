use crate::debug;
use crate::drivers::pci::*;
use crate::interrupts;
use crate::interrupts::pit::*;
use crate::interrupts::{pic_8259, PIC_8259};
use crate::memory;
use crate::memory::tools::DeviceMap;
use crate::monitor::bmp_loader::*;
use crate::monitor::*;
use crate::multiboot::MultibootInfo;
use crate::timer::Rtc;

extern "C" {
    static _asterix_bmp_start: BmpImage;
    static _wanggle_bmp_start: BmpImage;
}

#[no_mangle]
pub extern "C" fn kmain(multiboot_info: *const MultibootInfo, device_map_ptr: *const DeviceMap) -> u32 {
    #[cfg(feature = "serial-eprintln")]
    {
        unsafe { crate::io::UART_16550.init() };
        eprintln!("you are in serial eprintln mode");
    }
    let multiboot_info: MultibootInfo = unsafe { *multiboot_info };
    // TODO Like multigrub structure, it could be cool to save the entire device map here !
    println!("multiboot_infos {:#?}", multiboot_info);
    dbg!(multiboot_info.mem_lower);
    dbg!(multiboot_info.mem_upper);

    unsafe {
        interrupts::init();
        crate::watch_dog();

        let device_map = crate::memory::tools::get_device_map_slice(device_map_ptr);
        memory::init_memory_system(multiboot_info.get_memory_amount_nb_pages(), device_map).unwrap();

        crate::watch_dog();
        eprintln!("bonjour");
        SCREEN_MONAD.switch_graphic_mode(Some(0x118)).unwrap();
        SCREEN_MONAD.set_text_color(Color::Green).unwrap();

        SCREEN_MONAD.set_text_color(Color::Blue).unwrap();

        SCREEN_MONAD.clear_screen();

        SCREEN_MONAD
            .draw_graphic_buffer(|buffer: *mut u8, width: usize, height: usize, bpp: usize| {
                draw_image(&_asterix_bmp_start, buffer, width, height, bpp)
            })
            .unwrap();
        SCREEN_MONAD.set_text_color(Color::Cyan).unwrap();
        eprintln!("bonjour");
    }
    printfixed!(111, 46, "Turbo Fish v{}+", 0.2);

    unsafe {
        PIT0.configure(OperatingMode::RateGenerator);
        PIT0.start_at_frequency(1000.).unwrap();
        PIC_8259.enable_irq(pic_8259::Irq::SystemTimer);
    }
    debug::bench_start();
    //    crate::test_helpers::fucking_big_string(3);
    let t = debug::bench_end();
    println!("{:?} ms ellapsed", t);

    println!("from {}", function!());

    println!("irqs state: {}", interrupts::get_interrupts_state());

    println!("irq mask: {:b}", unsafe { PIC_8259.get_masks() });

    let eflags = crate::registers::Eflags::get_eflags();
    println!("{:x?}", eflags);

    unsafe {
        PIT0.start_at_frequency(1000.).unwrap();
    }
    unsafe {
        SCREEN_MONAD
            .draw_graphic_buffer(|buffer: *mut u8, width: usize, height: usize, bpp: usize| {
                draw_image(&_wanggle_bmp_start, buffer, width, height, bpp)
            })
            .unwrap();
        SCREEN_MONAD.set_text_color(Color::Green).unwrap();
    }

    unsafe {
        PCI.scan_pci_buses();
        PCI.list_pci_devices();
    }

    debug::bench_start();

    println!("pit: {:?}", unsafe { &PIT0 });

    let t = debug::bench_end();
    println!("{:?} ms ellapsed", t);

    crate::test_helpers::really_lazy_hello_world();
    let mut rtc = Rtc::new();
    let date = rtc.read_date();
    println!("{}", date);

    use alloc::vec;
    use alloc::vec::Vec;
    //TODO: we should init paging at the begin of code
    //test Bootstrap allocator

    println!("begin test 1");
    debug::bench_start();
    let mut sum: u32 = 0;
    for i in 0..2 {
        let v: Vec<u8> = vec![(i & 0xff) as u8; 4096 * 16];
        sum += v[0] as u32;
        drop(v);
    }
    let t = debug::bench_end();
    println!("{:?} ms ellapsed", t);
    println!("multiboot_infos {:#?}", multiboot_info);
    println!("device map ptr: {:#?}", device_map_ptr);
    println!("first structure: {:?}", unsafe { *device_map_ptr });

    //crate::test_helpers::trash_test::sa_va_castagner();
    //crate::test_helpers::trash_test::kpanic();
    crate::watch_dog();
    sum
}
