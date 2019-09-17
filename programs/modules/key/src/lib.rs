#![cfg_attr(not(test), no_std)]
#![feature(alloc_error_handler)]
#![feature(const_fn)]

extern crate alloc;

mod rust_main;
use rust_main::rust_main;

pub mod memory;
#[cfg(not(test))]
use crate::memory::RustGlobalAlloc;

/// As a matter of fact, we can't declare the MemoryManager inside a submodule.
#[cfg(not(test))]
#[global_allocator]
static mut MEMORY_MANAGER: RustGlobalAlloc = RustGlobalAlloc::new();

#[macro_use]
extern crate kernel_modules;

use kernel_modules::{ModResult, SymbolList, WRITER};

#[cfg(not(test))]
#[no_mangle]
fn _start(symtab_list: SymbolList) -> ModResult {
    rust_main(symtab_list)
}

#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
