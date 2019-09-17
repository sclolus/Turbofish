//! This file contains the main function of the module

use kernel_modules::{ModConfig, ModError, ModResult, ModReturn, SymbolList, WRITER};

use keyboard::{init_keyboard_driver, KEYBOARD_DRIVER};

pub fn rust_main(symtab_list: SymbolList) -> ModResult {
    (symtab_list.write)("Inserting Keyboard module\n");
    unsafe {
        WRITER.set_write_callback(symtab_list.write);
        #[cfg(not(test))]
        crate::MEMORY_MANAGER.set_methods(symtab_list.alloc_tools);
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
