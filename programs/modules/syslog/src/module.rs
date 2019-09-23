//! This file contains the main function of the module

use kernel_modules::{
    KernelSymbolList, ModConfig, ModError, ModResult, ModReturn, ModSpecificReturn, SymbolList,
    SyslogReturn, WRITER,
};

static mut CTX: Option<Ctx> = None;

use libc_binding::Errno;

#[allow(dead_code)]
/// Main Context of the module
struct Ctx {
    kernel_symbol_list: KernelSymbolList,
    write_syslog: fn(&str) -> Result<(), Errno>,
}

/// Main Context implementation
impl Ctx {
    /// New fn
    fn new(
        kernel_symbol_list: KernelSymbolList,
        write_syslog: fn(&str) -> Result<(), Errno>,
    ) -> Self {
        print!("New Syslog Context created !");
        Self {
            kernel_symbol_list,
            write_syslog,
        }
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
            let add_syslog_entry = symtab_list.kernel_symbol_list.get_entry("add_syslog_entry");
            match add_syslog_entry {
                None => Err(ModError::DependencyNotSatisfied),
                Some(addr) => {
                    let write_syslog: fn(&str) -> Result<(), Errno> = core::mem::transmute(addr);
                    CTX = Some(Ctx::new(symtab_list.kernel_symbol_list, write_syslog));
                    // Write something into the log to check if all is okay
                    add_entry("Syslog launched\n");
                    Ok(ModReturn {
                        stop: drop_module,
                        spec: ModSpecificReturn::Syslog(SyslogReturn { add_entry }),
                    })
                }
            }
        }
    } else {
        Err(ModError::BadIdentification)
    }
}

fn add_entry(entry: &str) {
    unsafe {
        (CTX.as_ref().unwrap().write_syslog)(entry).expect("Woot ?");
    }
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
