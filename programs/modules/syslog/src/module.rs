//! This file contains the main function of the module

use kernel_modules::{
    KernelSymbolList, ModConfig, ModError, ModResult, ModReturn, ModSpecificReturn, SymbolList,
    SyslogReturn, WRITER,
};

static mut CTX: Option<Ctx> = None;

#[allow(dead_code)]
/// Main Context of the module
struct Ctx {
    kernel_symbol_list: KernelSymbolList,
}

/// Main Context implementation
impl Ctx {
    /// New fn
    fn new(kernel_symbol_list: KernelSymbolList) -> Self {
        print!("New Syslog Context created !");
        Self { kernel_symbol_list }
    }
}

/// Drop boilerplate implementation
impl Drop for Ctx {
    fn drop(&mut self) {
        print!("Syslog Context droped !");
    }
}

/// Constructor
pub fn module_start(symtab_list: SymbolList) -> ModResult {
    unsafe {
        WRITER.set_write_callback(symtab_list.write);
        #[cfg(not(test))]
        crate::MEMORY_MANAGER.set_methods(symtab_list.alloc_tools);
    }
    if let ModConfig::Syslog = symtab_list.kernel_callback {
        unsafe {
            CTX = Some(Ctx::new(symtab_list.kernel_symbol_list));
        }

        // unsafe {
        // Just do a test for kernel symbol list
        // let ksymbol_test = CTX
        //     .as_ref()
        //     .unwrap()
        //     .kernel_symbol_list
        //     .get_entry("add_syslog_entry");
        // if let Some(addr) = ksymbol_test {
        //     let f: fn() = core::mem::transmute(addr);
        //     f();
        // } else {
        //     print!("Symbol Test Not Founded !");
        // }
        // }

        Ok(ModReturn {
            stop: drop_module,
            spec: ModSpecificReturn::Syslog(SyslogReturn { add_entry }),
        })
    } else {
        Err(ModError::BadIdentification)
    }
}

fn add_entry(_entry: &str) {
    // unsafe {}
}

/// Destructor
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
