use crate::vga::*;
use crate::multiboot::{MULTIBOOT_INFO, MultibootInfo, save_multiboot_info};

#[no_mangle]
pub extern "C" fn kmain(multiboot_info: *const MultibootInfo) {
    clear_screen();
    save_multiboot_info(multiboot_info);
    println!("multiboot_infos {:?}", MULTIBOOT_INFO);
    /*
    set_text_color("yellow").unwrap();
    for _x in 0..2 {
        println!("test\nPrintln");
        println!("vga term {:#?}", VGA_TERM);
        println!();
        print!("E");
        println!("RTV");
        println!("RTV");
    }
    match set_text_color("alacrityKikooColor") {
        Ok(()) => (),
        Err(e) => println!("{:?}", e),
    }
    match set_text_color("brown") {
        Ok(()) => (),
        Err(e) => println!("{:?}", e),
    }
    match set_cursor_position(40, 24) {
        Ok(()) => (),
        Err(e) => println!("{:?}", e),
    }
    match set_cursor_position(42, 42) {
        Ok(()) => (),
        Err(e) => println!("{:?}", e),
    }
    set_cursor_position(42, 42).unwrap();
    */
    loop {}
}
