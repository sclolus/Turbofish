//! This crate provide a kernel module toolkit
#![cfg_attr(not(test), no_std)]

use keyboard::CallbackKeyboard;

/// Fixed Virtual address of the modules
#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum ModAddress {
    Dummy = 0xE0000000,
    Keyboard = 0xE1000000,
}

/// Errors on module
#[derive(Debug, Copy, Clone)]
pub enum ModError {
    BadIdentification,
}

/// Result of a _start request
pub type ModResult = core::result::Result<ModReturn, ModError>;

/// This enum describes kernel functions specifics that the module could call
#[derive(Debug, Copy, Clone)]
pub enum ModConfig {
    /// The dummy module need nothing !
    Dummy,
    /// The RTC module have to set an IDT entry
    RTC(fn(usize, Option<unsafe extern "C" fn()>)),
    /// The Keyboard module have to set an IDT entry and return kerboard activities with a kernel callback
    Keyboard(fn(usize, Option<unsafe extern "C" fn()>), CallbackKeyboard),
}

/// This module describes function specifics that the kernel could call
#[derive(Debug, Copy, Clone)]
pub enum ModReturn {
    /// The kernel cannot ask the Dummy module
    Dummy,
    /// The RTC can be stopped and should give the time to the kernel
    RTC(fn(), fn() -> u32),
    /// The keyboard can be stopped but The kernel cannot ask it
    Keyboard(fn()),
}

/// This structure is passed zhen _start point of the module is invoqued
#[derive(Copy, Clone)]
pub struct SymbolList {
    /// Allow to debug the module
    pub write: fn(&str),
    /// Allow modules to allocate or free memory
    pub alloc_tools: ForeignAllocMethods,
    /// Specifics methods given by the kernel
    pub kernel_callback: ModConfig,
}

/// The allocators methods are passed by the kernel while module is initialized
#[derive(Copy, Clone)]
pub struct ForeignAllocMethods {
    /// KMalloc like a boss
    pub kmalloc: unsafe extern "C" fn(usize) -> *mut u8,
    /// alloc with zeroed
    pub kcalloc: unsafe extern "C" fn(usize, usize) -> *mut u8,
    /// Free Memory
    pub kfree: unsafe extern "C" fn(*mut u8),
    /// Realloc in place if possible
    pub krealloc: unsafe extern "C" fn(*mut u8, usize) -> *mut u8,
}
