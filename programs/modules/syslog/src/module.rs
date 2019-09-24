//! This file contains the main function of the module

use kernel_modules::{
    ConfigurableCallback, KernelEvent, KernelSymbolList, ModConfig, ModError, ModResult, ModReturn,
    ModSpecificReturn, SymbolList, WRITER,
};

static mut CTX: Option<Ctx> = None;

use alloc::string::String;
use alloc::vec::Vec;
use fallible_collections::{try_vec, vec::FallibleVec};
use libc_binding::Errno;
use log::Record;

#[allow(dead_code)]
/// Main Context of the module
struct Ctx {
    kernel_symbol_list: KernelSymbolList,
    write_syslog: fn(&str) -> Result<(), Errno>,
    cache: Vec<String>,
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
            cache: Vec::new(),
        }
    }

    /// Write stored log entry onto the syslog
    fn write_to_syslog(&mut self) {
        for item in self.cache.iter() {
            (self.write_syslog)(item).expect("Woot ?");
        }
        self.cache.clear();
    }
}

/// Drop boilerplate implementation
impl Drop for Ctx {
    fn drop(&mut self) {
        // Fflush the cache into the syslog file
        self.write_to_syslog();
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
                    let configurable_callbacks_opt: Option<Vec<ConfigurableCallback>> = Some(
                        try_vec!(
                            ConfigurableCallback {
                                when: KernelEvent::Log,
                                what: add_entry as u32,
                            },
                            ConfigurableCallback {
                                when: KernelEvent::Second,
                                what: fflush_syslog as u32,
                            }
                        )
                        .map_err(|_| ModError::OutOfMemory)?,
                    );

                    let write_syslog: fn(&str) -> Result<(), Errno> = core::mem::transmute(addr);
                    CTX = Some(Ctx::new(symtab_list.kernel_symbol_list, write_syslog));
                    Ok(ModReturn {
                        stop: drop_module,
                        configurable_callbacks_opt,
                        spec: ModSpecificReturn::Syslog,
                    })
                }
            }
        }
    } else {
        Err(ModError::BadIdentification)
    }
}

/// Store a log entry into the module memory
fn add_entry(entry: &Record) {
    unsafe {
        // TODO: Make alloc::fmt::try_format for fallible context one day
        let _r = CTX
            .as_mut()
            .unwrap()
            .cache
            .try_push(alloc::fmt::format(format_args!("{:?}\n", entry)));
    }
}

/// Write the syslog cache into the file
fn fflush_syslog() {
    unsafe {
        CTX.as_mut().unwrap().write_to_syslog();
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
