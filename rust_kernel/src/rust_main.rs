use crate::monitor::core_monitor::*;
use crate::monitor::vbe_mode::*;
use crate::monitor::*;
use crate::multiboot::{save_multiboot_info, MultibootInfo, MULTIBOOT_INFO};

#[no_mangle]
extern "C" {
    pub fn _isr_divide_by_zero(cs: u32, iflag: u32) -> ();
}

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
    init_graphic_mode(Some(0x118)).unwrap();
    unsafe {
        VBE_MODE.unwrap().put_pixel(100, 100);
        VBE_MODE.unwrap().fill_screen(RGB::blue());
        use core::fmt::Write;
        VBE_MODE.unwrap().write_str("\
Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum
");
    }
    loop {}
}

