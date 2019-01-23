extern "C" {
    fn asm_inb(port: u16) -> u8;
    fn asm_outb(byte: u8, port: u16) -> u8;
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn _inb(port: u16) -> u8 {
    unsafe { asm_inb(port) }
}

#[no_mangle]
#[inline(never)]
pub extern "C" fn _outb(byte: u8, port: u16) -> u8 {
    unsafe { asm_outb(byte, port) }
}
