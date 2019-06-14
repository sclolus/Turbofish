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

#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{:#X?}", info);
    loop {}
}

mod shell;
use shell::shell;

#[cfg(not(test))]
#[no_mangle]
fn main() -> ! {
    shell();
}
