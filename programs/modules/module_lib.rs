#![cfg_attr(not(test), no_std)]
#![feature(alloc_error_handler)]
#![feature(const_fn)]
#![cfg_attr(test, allow(unused_imports))]
#![cfg_attr(test, allow(dead_code))]

extern crate alloc;

#[allow(unused)]
#[macro_use]
extern crate interrupts;

mod module;
use module::module_start;

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
    module_start(symtab_list)
}

#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
