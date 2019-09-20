//! This file contains the main function of the module

use kernel_modules::{
    ModConfig, ModError, ModResult, ModReturn, ModSpecificReturn, SymbolList, WRITER,
};

use alloc::boxed::Box;

static mut CTX: Option<Ctx> = None;

struct Ctx {
    b: Box<OutBox>,
}

impl Ctx {
    fn new() -> Self {
        println!("New Dummy Context created !");
        Self {
            b: Box::new(OutBox::new(42)),
        }
    }
}

impl Drop for Ctx {
    fn drop(&mut self) {
        println!("Dummy Context droped !");
    }
}

struct OutBox {
    value: u32,
}

impl OutBox {
    fn new(value: u32) -> Self {
        println!("New OutBox created !");
        Self { value }
    }
}

impl Drop for OutBox {
    fn drop(&mut self) {
        println!("Dummy OutBox droped !");
    }
}

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
        unsafe {
            CTX = Some(Ctx::new());
        }
        Ok(ModReturn {
            stop: drop_module,
            spec: ModSpecificReturn::DummyReturn {},
        })
    } else {
        Err(ModError::BadIdentification)
    }
}

fn drop_module() {
    unsafe {
        CTX = None;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
