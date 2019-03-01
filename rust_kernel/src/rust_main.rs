use crate::debug;
use crate::interrupts;
use crate::interrupts::pit::*;
use crate::interrupts::{pic_8259, PIC_8259};
use crate::mm;
use crate::monitor::bmp_loader::*;
use crate::monitor::*;
use crate::multiboot::{save_multiboot_info, MultibootInfo, MULTIBOOT_INFO};
use crate::timer::Rtc;
use core::time::Duration;

extern "C" {
    static _asterix_bmp_start: BmpImage;
    static _wanggle_bmp_start: BmpImage;
}

#[no_mangle]
pub extern "C" fn kmain(multiboot_info: *const MultibootInfo) -> u32 {
    save_multiboot_info(multiboot_info);
    unsafe {
        println!("multiboot_infos {:#?}", MULTIBOOT_INFO);
        println!("base memory: {:?} {:?}", MULTIBOOT_INFO.unwrap().mem_lower, MULTIBOOT_INFO.unwrap().mem_upper);
    }

    unsafe {
        interrupts::init();

        SCREEN_MONAD.switch_graphic_mode(Some(0x118)).unwrap();
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
    unsafe {
        println!("multiboot_infos {:#?}", MULTIBOOT_INFO);
    }
    debug::bench_start();
    // fucking_big_string(3);
    let t = debug::bench_end();
    println!("{:?} ms ellapsed", t);

    println!("from {}", function!());

    println!("irqs state: {}", interrupts::get_interrupts_state());

    unsafe {
        println!("irq mask: {:b}", PIC_8259.get_masks());
    }

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
    unsafe {
        println!("pit: {:?}", PIT0);
    }
    unsafe {
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::Green).unwrap();
        print!("H");
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::Red).unwrap();
        print!("E");
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::Blue).unwrap();
        print!("L");
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::Yellow).unwrap();
        print!("L");
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::Cyan).unwrap();
        print!("O");
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::Brown).unwrap();
        print!(" ");
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::Magenta).unwrap();
        print!("W");
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::White).unwrap();
        print!("O");
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::Green).unwrap();
        print!("R");
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::Red).unwrap();
        print!("L");
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::Blue).unwrap();
        print!("D");
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::Yellow).unwrap();
        print!(" ");
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::Cyan).unwrap();
        println!("!");
        PIT0.sleep(Duration::from_millis(200));
        SCREEN_MONAD.set_text_color(Color::White).unwrap();
    }
    let t = debug::bench_end();
    println!("{:?} ms ellapsed", t);
    let mut rtc = Rtc::new();
    let date = rtc.read_date();
    println!("{}", date);

    use alloc::vec;
    use alloc::vec::Vec;
    //TODO: we should init paging at the begin of code
    //test Bootstrap allocator
    //let mut vec: Vec<u32> = vec![0; 100];

    //println!("{:?}", vec);
    unsafe {
        mm::init_memory_system().unwrap();
    }
    let mut vec: Vec<u8> = Vec::new();
    for index in 0..100 {
        vec.push(index)
    }

    let vec2 = vec![42_u8; 42];
    println!("{:?}", vec2);
    println!("{:?}", vec);
    //loop {}
    0
}
