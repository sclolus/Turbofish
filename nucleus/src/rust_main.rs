use vga::VGA_TERM;
#[no_mangle]
pub extern "C" fn kmain() {
    println!("test\nPrintln");
    println!("vga term {:#?}", VGA_TERM);
    println!();
    print!("E");
    println!("RTV");
    println!("RTV");
    loop { }
}
