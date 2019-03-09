use crate::debug;
use crate::interrupts;
use crate::interrupts::pit::*;
use crate::interrupts::{pic_8259, PIC_8259};
use crate::memory;
use crate::monitor::bmp_loader::*;
use crate::monitor::*;
use crate::multiboot::MultibootInfo;
use crate::timer::Rtc;

extern "C" {
    static _asterix_bmp_start: BmpImage;
    static _wanggle_bmp_start: BmpImage;
}

#[no_mangle]
pub extern "C" fn kmain(multiboot_info: *const MultibootInfo) -> u32 {
    let multiboot_info: MultibootInfo = unsafe { *multiboot_info };
    unsafe {
        memory::init_memory_system(multiboot_info.get_memory_amount_nb_pages()).unwrap();
    }

    println!("multiboot_infos {:#?}", multiboot_info);
    dbg!(multiboot_info.mem_lower);
    dbg!(multiboot_info.mem_upper);

    unsafe {
        interrupts::init();

        SCREEN_MONAD.switch_graphic_mode(Some(0x118)).unwrap();
        SCREEN_MONAD.set_text_color(Color::Green).unwrap();

        SCREEN_MONAD.set_text_color(Color::Blue).unwrap();

        SCREEN_MONAD.clear_screen();

        SCREEN_MONAD
            .draw_graphic_buffer(|buffer: *mut u8, width: usize, height: usize, bpp: usize| {
                draw_image(&_asterix_bmp_start, buffer, width, height, bpp)
            })
            .unwrap();

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
    for i in 0..4096 {
        let v: Vec<u8> = vec![(i & 0xff) as u8; 4096 * 16];
        sum += v[0] as u32;
        drop(v);
    }
    let t = debug::bench_end();
    println!("{:?} ms ellapsed", t);

    /*
    use crate::memory::kernel_allocator::{Allocator, KernelAllocator, ALLOCATOR};
    use crate::memory::{MemoryError, VirtualAddr};

    extern "C" {
        fn ft_memset(v: VirtualAddr, u: u8, s: usize) -> VirtualAddr;
    }

    use core::alloc::Layout;

    println!("begin test 2");
    debug::bench_start();
    for i in 0..4096 {
        let v: VirtualAddr =

        unsafe {
        match &mut ALLOCATOR {
            Allocator::Kernel(a) => a.alloc(4096 * 16).unwrap(), //.unwrap_or(PhysicalAddr(0x0)).0 as *mut u8
            Allocator::Bootstrap(_) => panic!("panic sa mere"),
        }
        };

        unsafe {
            ft_memset(v, (i & 0xff) as u8, 4096 * 16);
            sum += *(v.0 as *const u8) as u32;
        }
    }
    let t = debug::bench_end();
    println!("{:?} ms ellapsed {:?}", t, sum);
    */
    sum
}
