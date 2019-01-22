#![no_std]
#![feature(core_intrinsics)] // for function! macro


#[macro_use]
pub mod debug;

#[macro_use]
pub mod monitor;
pub mod multiboot;
pub mod registers;
pub mod rust_main;
pub mod support; // For Rust lang items
pub mod panic;
pub mod ffi;


use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    print!("Just a panic, not a SegFault");
    loop {}
}

