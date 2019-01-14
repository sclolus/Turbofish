#![feature(lang_items)]
#![feature(asm)]
#![no_std]
#![feature(format_args_nl)]

#[macro_use]
pub mod monitor;

pub mod support; // For Rust lang items
pub mod rust_main;
pub mod multiboot;

use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    print!("Just a panic, not a SegFault");
    loop {}
}
