#![cfg_attr(not(test), no_std)]
#![feature(alloc_error_handler)]

pub mod memory;
#[cfg(not(test))]
use crate::memory::RustGlobalAlloc;

/// As a matter of fact, we can't declare the MemoryManager inside a submodule.
#[cfg(not(test))]
#[global_allocator]
static mut MEMORY_MANAGER: RustGlobalAlloc = RustGlobalAlloc::new();

extern crate alloc;

use alloc::boxed::Box;

use kernel_modules::{ModuleName, ModuleResult, SymbolList};

#[no_mangle]
fn _start(symtab_list: SymbolList, _module_type: ModuleName) -> ModuleResult<()> {
    (symtab_list.write)("I've never install GNU/Linux.\n");
    unsafe {
        MEMORY_MANAGER.set_methods(symtab_list.alloc_tools);
    }
    let b = Box::new("Displaying allocated String !\n");
    (symtab_list.write)(&b);
    Ok(())
}

#[panic_handler]
#[no_mangle]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
