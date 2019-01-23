#![feature(const_raw_ptr_to_usize_cast)] // rust is being annoying on the types of ffi functions
#![no_std]
#![feature(core_intrinsics)] // for function! macro

#[macro_use]
pub mod debug;

#[macro_use]
pub mod ffi;

#[macro_use]
pub mod monitor;
pub mod multiboot;
pub mod panic;
pub mod registers;
pub mod rust_main;
pub mod support; // For Rust lang items
pub mod io;
pub mod interrupts;

use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    print!("Just a panic, not a SegFault");
    loop {}
}
