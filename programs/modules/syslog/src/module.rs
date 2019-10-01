//! This file contains the main function of the module

use kernel_modules::{
    ConfigurableCallback, KernelEvent, KernelSymbolList, ModConfig, ModError, ModResult, ModReturn,
    ModSpecificReturn, SymbolList,
};

static mut CTX: Option<Ctx> = None;

use alloc::string::String;
use alloc::vec::Vec;
use ansi_escape_code::Colored;
use fallible_collections::{try_vec, tryformat, vec::FallibleVec};
use libc_binding::Errno;
use log::{Level, Record};

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
        kernel_modules::init_config(&symtab_list, &mut super::MEMORY_MANAGER);
    }
    if let ModConfig::Syslog = symtab_list.kernel_callback {
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

                let write_syslog: fn(&str) -> Result<(), Errno> =
                    unsafe { core::mem::transmute(addr) };
                unsafe {
                    CTX = Some(Ctx::new(symtab_list.kernel_symbol_list, write_syslog));
                }
                Ok(ModReturn {
                    stop: drop_module,
                    configurable_callbacks_opt,
                    spec: ModSpecificReturn::Syslog,
                })
            }
        }
    } else {
        Err(ModError::BadIdentification)
    }
}

const LOG_FORMAT_MAX_CAPACITY: usize = 4096;

/// Store a log entry into the module memory
fn add_entry(record: &Record) {
    let context = unsafe { &mut CTX.as_mut().unwrap() };

    let level_str = match record.level() {
        Level::Info => "INFO".green(),
        Level::Trace => "TRACE".white(),
        Level::Error => "ERROR".red(),
        Level::Warn => "WARN".yellow(),
        Level::Debug => "DEBUG".cyan(),
    };

    let file = match record.file() {
        Some(file) => file,
        None => "unknown",
    };

    let line = tryformat!(
        LOG_FORMAT_MAX_CAPACITY,
        "{}",
        match record.line() {
            Some(line_number) => line_number,
            None => 0,
        }
    );

    let msg = tryformat!(LOG_FORMAT_MAX_CAPACITY, "{}", record.args());

    if let (Ok(line), Ok(msg)) = (line, msg) {
        match tryformat!(
            LOG_FORMAT_MAX_CAPACITY,
            "{} msg: {} from: {} line: {}\n",
            level_str,
            msg.cyan(),
            file.yellow(),
            line.yellow()
        ) {
            Ok(string) => {
                let r = context.cache.try_push(string);
                if let Err(_e) = r {
                    emergency_print!("Cannot push entry into syslog cache");
                }
            }
            Err(_e) => {
                emergency_print!("Cannot allocate enough memory to format syslog entry");
            }
        }
    } else {
        emergency_print!("Cannot allocate some stuff");
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
