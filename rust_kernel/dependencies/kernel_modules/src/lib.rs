//! This crate provide a kernel module toolkit
#![cfg_attr(not(test), no_std)]

#[derive(Debug, Copy, Clone)]
pub enum ModuleError {
    BadIdentification,
}

pub type ModuleResult<T> = core::result::Result<T, ModuleError>;

#[derive(Debug, Copy, Clone)]
pub enum ModuleName {
    Dummy,
    RTC,
    Keyboard,
}

#[derive(Copy, Clone)]
pub struct SymbolList {
    pub write: fn(&str),
}

// fn begin(symtab_list: *const SymbolList, module_type: ModuleName) -> ModuleResult<()>;

// use core::mem;
// use raw_data::define_raw_data;

// define_raw_data!(FlatBinaryExecutableCode, 440);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
