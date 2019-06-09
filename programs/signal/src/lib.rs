//! This crate is a tiny rust sodo static lib linked with a static tiny libc

#![cfg_attr(not(test), no_std)]
#![cfg_attr(test, feature(allocator_api))]
#![feature(alloc)]
#![feature(alloc_error_handler)]
#![feature(asm)]
#![feature(core_intrinsics)]
#![warn(missing_docs)]

pub mod memory;
#[cfg(not(test))]
use crate::memory::RustGlobalAlloc;

/// As a matter of fact, we can't declare the MemoryManager inside a submodule.
#[cfg(not(test))]
#[global_allocator]
static MEMORY_MANAGER: RustGlobalAlloc = RustGlobalAlloc;

extern crate alloc;

#[macro_use]
pub mod writer;
pub use writer::*;

pub mod math;

#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{:#X?}", info);
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
fn main() -> i32 {
    println!("initialise Signals test");

    let ret = unsafe {
        // 2 is the number of SIGINT
        signal(2, hello_signal)
    };
    println!("signal function return: {:?}", ret);

    loop {}

    #[allow(unreachable_code)]
    0
}

#[no_mangle]
extern "C" fn hello_signal(signum: i32) -> () {
    println!("Signal Received 5/5: {:?}", signum);
}

extern "C" {
    fn signal(signum: i32, function: extern "C" fn(i32)) -> extern "C" fn(i32);
}
