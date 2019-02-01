#![feature(const_raw_ptr_to_usize_cast)] // rust is being annoying on the types of ffi functions
#![feature(integer_atomics)]
#![no_std]
#![feature(core_intrinsics)] // for function! macro
#![feature(asm)] // for inline asm
#![allow(unused_unsafe)]

#[macro_use]
pub mod debug;

#[macro_use]
pub mod ffi;

#[macro_use]
pub mod monitor;
pub mod interrupts;
pub mod io;
pub mod multiboot;
pub mod panic;
pub mod registers;
pub mod rust_main;
pub mod support; // For Rust lang items
pub mod timer;

use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    print!("Just a panic, not a SegFault");
    loop {}
}
