#![cfg_attr(not(test), no_std)]
#![feature(alloc_error_handler)]
#![feature(const_fn)]

#[macro_use]
pub mod writer;
pub use writer::WRITER;

pub mod memory;
#[cfg(not(test))]
use crate::memory::RustGlobalAlloc;

/// As a matter of fact, we can't declare the MemoryManager inside a submodule.
#[cfg(not(test))]
#[global_allocator]
static mut MEMORY_MANAGER: RustGlobalAlloc = RustGlobalAlloc::new();

extern crate alloc;

use kernel_modules::{ModConfig, ModError, ModResult, ModReturn, SymbolList};

use keyboard::{init_keyboard_driver, KEYBOARD_DRIVER};

#[no_mangle]
fn _start(symtab_list: SymbolList) -> ModResult {
    (symtab_list.write)("Inserting Keyboard module\n");
    unsafe {
        WRITER.set_write_callback(symtab_list.write);
        MEMORY_MANAGER.set_methods(symtab_list.alloc_tools);
    }
    if let ModConfig::Keyboard(_idt_fn, _callback_fn) = symtab_list.kernel_callback {
        init_keyboard_driver();
        // Set IRQ/IDT
        // CallBack function
        Ok(ModReturn::Keyboard(drop_module))
    } else {
        Err(ModError::BadIdentification)
    }
}

fn drop_module() {}

#[panic_handler]
#[no_mangle]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
