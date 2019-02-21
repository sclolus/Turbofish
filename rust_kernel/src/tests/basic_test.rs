use crate::debug;
use crate::interrupts;
use crate::interrupts::pit::*;
use crate::interrupts::{pic_8259, PIC_8259};
use crate::io::{Pio, UART_16550};
use crate::monitor::bmp_loader::*;
use crate::monitor::*;
use crate::multiboot::{save_multiboot_info, MultibootInfo, MULTIBOOT_INFO};
use crate::tests::helpers::exit_qemu;
use crate::timer::Rtc;

extern "C" {
    static _asterix_bmp_start: BmpImage;
    static _wanggle_bmp_start: BmpImage;
}

#[no_mangle]
pub extern "C" fn kmain(multiboot_info: *const MultibootInfo) -> u32 {
    unsafe {
        UART_16550.init();
    }
    save_multiboot_info(multiboot_info);
    unsafe {
        println!("multiboot_infos {:#?}", MULTIBOOT_INFO);
        println!("base memory: {:?} {:?}", MULTIBOOT_INFO.unwrap().mem_lower, MULTIBOOT_INFO.unwrap().mem_upper);
    }

    unsafe {
        interrupts::init();
    }

    unsafe {
        SCREEN_MONAD.switch_graphic_mode(Some(0x118)).unwrap();
        SCREEN_MONAD.set_text_color(Color::Blue).unwrap();
        SCREEN_MONAD.clear_screen();
        SCREEN_MONAD
            .draw_graphic_buffer(|buffer: *mut u8, width: usize, height: usize, bpp: usize| {
                draw_image(&_asterix_bmp_start, buffer, width, height, bpp)
            })
            .unwrap();

        PIT0.configure(OperatingMode::RateGenerator);
        PIT0.start_at_frequency(18.0).unwrap();
        PIC_8259.enable_irq(pic_8259::Irq::SystemTimer);
    }
    debug::bench_start();
    println!("from {}", function!());

    println!("irqs state: {}", interrupts::get_interrupts_state());
    let _keyboard_port = Pio::<u8>::new(0x60);

    unsafe {
        println!("irq mask: {:b}", PIC_8259.get_masks());
    }

    /*
    unsafe {
        assert_eq!(_idt, interrupts::get_idtr().get_interrupt_table());
        for (index, gate) in interrupts::get_idtr().get_interrupt_table().as_slice()[..48].iter().enumerate() {
            println!("{}: {:?}", index, gate);
        }
    }
    */
    let eflags = crate::registers::Eflags::get_eflags();
    //println!("idtr: {:x?}", interrupts::get_idtr());
    println!("{}", eflags);
    println!("{:x?}", eflags);

    println!("from {}", function!());
    println!("{:?} ms ellapsed", debug::bench_end());
    unsafe {
        PIT0.start_at_frequency(18.).unwrap();
    }
    debug::bench_start();
    unsafe {
        println!("pit: {:?}", PIT0);
    }
    unsafe {
        SCREEN_MONAD.set_text_color(Color::Green).unwrap();
        print!("H");
        SCREEN_MONAD.set_text_color(Color::Red).unwrap();
        print!("E");
        SCREEN_MONAD.set_text_color(Color::Blue).unwrap();
        print!("L");
        SCREEN_MONAD.set_text_color(Color::Yellow).unwrap();
        print!("L");
        SCREEN_MONAD.set_text_color(Color::Cyan).unwrap();
        print!("O");
        SCREEN_MONAD.set_text_color(Color::Brown).unwrap();
        print!(" ");
        SCREEN_MONAD.set_text_color(Color::Magenta).unwrap();
        print!("W");
        SCREEN_MONAD.set_text_color(Color::White).unwrap();
        print!("O");
        SCREEN_MONAD.set_text_color(Color::Green).unwrap();
        print!("R");
        SCREEN_MONAD.set_text_color(Color::Red).unwrap();
        print!("L");
        SCREEN_MONAD.set_text_color(Color::Blue).unwrap();
        print!("D");
        SCREEN_MONAD.set_text_color(Color::Yellow).unwrap();
        print!(" ");
        SCREEN_MONAD.set_text_color(Color::Cyan).unwrap();
        println!("!");
        SCREEN_MONAD.set_text_color(Color::White).unwrap();
    }
    unsafe {
        SCREEN_MONAD
            .draw_graphic_buffer(|buffer: *mut u8, width: usize, height: usize, bpp: usize| {
                draw_image(&_wanggle_bmp_start, buffer, width, height, bpp)
            })
            .unwrap();
        SCREEN_MONAD.set_text_color(Color::Green).unwrap();
    }
    let mut rtc = Rtc::new();
    let date = rtc.read_date();
    println!("{}", date);
    println!("THIS IS A BASIC TEST");
    exit_qemu(0);
    0
}
