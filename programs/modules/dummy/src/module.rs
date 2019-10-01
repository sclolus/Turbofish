//! This file contains the main function of the module

use kernel_modules::{ModConfig, ModError, ModResult, ModReturn, ModSpecificReturn, SymbolList};

use alloc::boxed::Box;

static mut CTX: Option<Ctx> = None;

struct Ctx {
    _b: Box<OutBox>,
}

impl Ctx {
    fn new() -> Self {
        print!("New Dummy Context created !");
        Self {
            _b: Box::new(OutBox::new(42)),
        }
    }
}

impl Drop for Ctx {
    fn drop(&mut self) {
        print!("Dummy Context droped !");
    }
}

struct OutBox {
    _value: u32,
}

impl OutBox {
    fn new(_value: u32) -> Self {
        print!("New OutBox created !");
        Self { _value }
    }
}

impl Drop for OutBox {
    fn drop(&mut self) {
        print!("Dummy OutBox droped !");
    }
}

/// Main function of the module
pub fn module_start(symtab_list: SymbolList) -> ModResult {
    (symtab_list.write)("I've never install GNU/Linux.");
    unsafe {
        kernel_modules::init_config(&symtab_list, &mut super::MEMORY_MANAGER);
    }
    if let ModConfig::Dummy = symtab_list.kernel_callback {
        let b = Box::new("Displaying allocated String !");
        (symtab_list.write)(&b);
        print!("Test print!");
        unsafe {
            CTX = Some(Ctx::new());
        }
        Ok(ModReturn {
            stop: drop_module,
            configurable_callbacks_opt: None,
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
