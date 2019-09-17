//! This file contains the main function of the module

use kernel_modules::{ModConfig, ModError, ModResult, ModReturn, SymbolList, WRITER};

use alloc::boxed::Box;

/// Main function of the module
pub fn rust_main(symtab_list: SymbolList) -> ModResult {
    (symtab_list.write)("I've never install GNU/Linux.\n");
    unsafe {
        WRITER.set_write_callback(symtab_list.write);
        #[cfg(not(test))]
        crate::MEMORY_MANAGER.set_methods(symtab_list.alloc_tools);
    }
    if let ModConfig::Dummy = symtab_list.kernel_callback {
        let b = Box::new("Displaying allocated String !\n");
        (symtab_list.write)(&b);
        println!("Test println!");
        Ok(ModReturn::Dummy)
    } else {
        Err(ModError::BadIdentification)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
