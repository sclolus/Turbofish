use crate::monitor::*;
use crate::multiboot::{save_multiboot_info, MultibootInfo, MULTIBOOT_INFO};
use crate::registers::{real_mode_op, BaseRegisters};

#[no_mangle]
pub extern "C" fn kmain(multiboot_info: *const MultibootInfo) {
    clear_screen();
    save_multiboot_info(multiboot_info);

    println!("multiboot_infos {:#?}", MULTIBOOT_INFO);
    println!("base memory: {:?} {:?}", MULTIBOOT_INFO.unwrap().mem_lower, MULTIBOOT_INFO.unwrap().mem_upper);

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

    let reg: BaseRegisters = BaseRegisters {
        edi: 0x1,
        esi: 0x2,
        ebp: 0x3,
        esp: 0x4,
        ebx: 0x118 | (1 << 14),
        edx: 0x6,
        ecx: 0x7,
        eax: 0x4F02,
    };
    print!("value: ");
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
