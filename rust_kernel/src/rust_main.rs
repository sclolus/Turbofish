use crate::monitor::*;
use crate::multiboot::{MULTIBOOT_INFO, MultibootInfo, save_multiboot_info};
use crate::registers::*;

#[no_mangle]
pub extern "C" fn kmain(multiboot_info: *const MultibootInfo) {
    clear_screen();
    save_multiboot_info(multiboot_info);

    println!("ebp = {:?}", _get_ebp());

    println!("multiboot_infos {:#?}", MULTIBOOT_INFO);
    println!("base memory: {:?} {:?}",
             MULTIBOOT_INFO.unwrap().mem_lower, MULTIBOOT_INFO.unwrap().mem_upper);

    set_text_color("yellow").unwrap();
    println!("vga term {:#?}", crate::monitor::vga_text_mode::VGA_TEXT);

    match set_text_color("alacrityKikooColor") {
        Ok(()) => (),
        Err(e) => println!("{:?}", e),
    }
    match set_text_color("brown") {
        Ok(()) => (),
        Err(e) => println!("{:?}", e),
    }

    let reg:BaseRegisters = BaseRegisters {
        edi: 0x11111, esi: 0x22222, ebp: 0x33333, esp: 0x44444, ebx: 0x55555, edx: 0x66666, ecx: 0x77777, eax: 0x88888
    };
    print!("value: ");
    println!("esp = {:?}", _get_esp());
    println!("ebp = {:?}", _get_ebp());
    loop {}
    println!("{:?}", real_mode_op(reg, 0x10));
    match set_cursor_position(4, 24) {
        Ok(()) => (),
        Err(e) => println!("{:?}", e),
    }
    match set_cursor_position(42, 42) {
        Ok(()) => (),
        Err(e) => println!("{:?}", e),
    }
    set_cursor_position(42, 42).unwrap();
    loop {}
}
