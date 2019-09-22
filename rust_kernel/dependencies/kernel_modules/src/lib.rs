//! This crate provide a kernel module toolkit
#![cfg_attr(not(test), no_std)]
#![feature(const_fn)]
// The Writer give print! && println! macros for modules
#![macro_use]
pub mod writer;
pub use writer::WRITER;

pub use irq::Irq;
pub use messaging::MessageTo;
pub use time::Date;

use core::sync::atomic::AtomicU32;

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

/// Fixed Virtual address of the modules
#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum ModAddress {
    Dummy = 0xE0000000,
    Keyboard = 0xE1000000,
    RTC = 0xE2000000,
}

/// Errors on module
#[derive(Debug, Copy, Clone)]
pub enum ModError {
    BadIdentification,
}

/// Result of a _start request
pub type ModResult = core::result::Result<ModReturn, ModError>;

/// Status of a given module
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Status {
    /// Module is inactive and unloaded
    Inactive,
    /// Module is active and loaded
    Active,
}

/// Default boilerplate for status
impl Default for Status {
    fn default() -> Self {
        Status::Inactive
    }
}

/// Status of all modules
#[derive(Debug, Default, Copy, Clone)]
pub struct ModStatus {
    /// Dummy module status
    pub dummy: Status,
    /// RTC module status
    pub rtc: Status,
    /// Keyboard module status
    pub keybard: Status,
}

/// This enum describes kernel functions specifics that the module could call
#[derive(Debug, Copy, Clone)]
pub enum ModConfig {
    /// The dummy module need nothing !
    Dummy,
    /// The RTC module have to set an IDT entry
    RTC(RTCConfig),
    /// The Keyboard module have to set an IDT entry and return kerboard activities with a kernel callback
    Keyboard(KeyboardConfig),
}

/// Configuration parameters of the RTC module
#[derive(Debug, Copy, Clone)]
pub struct RTCConfig {
    /// Give ability to redirect an IDT entry to a specific function
    pub enable_irq: fn(Irq, unsafe extern "C" fn()),
    /// Give ability to disable IDT entry
    pub disable_irq: fn(Irq),
    /// reference of current_unix_time kernel globale
    pub current_unix_time: &'static AtomicU32,
}

/// Configuration parameters of the Keyboard module
#[derive(Debug, Copy, Clone)]
pub struct KeyboardConfig {
    /// Give ability to redirect an IDT entry to a specific function
    pub enable_irq: fn(Irq, unsafe extern "C" fn()),
    /// Give ability to disable IDT entry
    pub disable_irq: fn(Irq),
    /// Keyboard callback given by the kernel
    pub callback: fn(MessageTo),
}

/// Standard mod return
#[derive(Debug, Copy, Clone)]
pub struct ModReturn {
    /// Stop the module
    pub stop: fn(),
    /// Specific module return
    pub spec: ModSpecificReturn,
}

/// This module describes function specifics that the kernel could call
#[derive(Debug, Copy, Clone)]
pub enum ModSpecificReturn {
    /// The kernel cannot ask the Dummy module
    DummyReturn,
    /// The RTC can be stopped and should give the time to the kernel
    RTC(RTCReturn),
    /// The keyboard can be stopped but The kernel cannot ask it
    Keyboard(KeyboardReturn),
}

/// Return parameters of the RTC module
#[derive(Debug, Copy, Clone)]
pub struct RTCReturn {
    /// Get the date from the RTC module
    pub read_date: fn() -> Date,
}

/// Return parameters of the Keyboard module
#[derive(Debug, Copy, Clone)]
pub struct KeyboardReturn {
    /// Enable to reboot computer with the PS2 controler
    pub reboot_computer: fn(),
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
