use crate::vga::*;

#[no_mangle]
pub extern "C" fn kmain() {
    clear_screen();
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
    loop {}
}
