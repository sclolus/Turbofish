use crate::vga::VGA_TERM;
use crate::vga::clear_screen;

#[no_mangle]
pub extern "C" fn kmain() {
    clear_screen();
    for _x in 0..5 {
        println!("test\nPrintln");
        println!("vga term {:#?}", VGA_TERM);
        println!();
        print!("E");
        println!("RTV");
        println!("RTV");
    }
    loop { }
}
