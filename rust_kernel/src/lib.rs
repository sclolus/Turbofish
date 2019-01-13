#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![no_std]
#![feature(compiler_builtins_lib)]
#![feature(format_args_nl)]

#[macro_use]
pub mod vga;

pub mod support; // For Rust lang items
pub mod rust_main;

use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    print!("Just a panic, not a SegFault");
    loop {}
}
