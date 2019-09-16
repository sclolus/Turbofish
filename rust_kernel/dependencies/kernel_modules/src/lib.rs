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
    pub alloc_tools: ForeignAllocMethods,
}

#[derive(Copy, Clone)]
pub struct ForeignAllocMethods {
    pub kmalloc: unsafe extern "C" fn(usize) -> *mut u8,
    pub kcalloc: unsafe extern "C" fn(usize, usize) -> *mut u8,
    pub kfree: unsafe extern "C" fn(*mut u8),
    pub krealloc: unsafe extern "C" fn(*mut u8, usize) -> *mut u8,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
