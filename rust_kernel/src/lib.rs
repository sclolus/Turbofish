#![no_std]

#[macro_use]
pub mod monitor;
pub mod multiboot;
pub mod registers;
pub mod rust_main;
pub mod support; // For Rust lang items

use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    print!("Just a panic, not a SegFault");
    loop {}
}
