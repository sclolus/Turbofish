#![cfg_attr(not(test), no_std)]
#![cfg_attr(test, allow(unused_imports))]
#![feature(const_raw_ptr_to_usize_cast)] // rust is being annoying on the types of ffi functions
#![feature(core_intrinsics)] // for function! macro
#![feature(asm)] // for inline asm
#![feature(try_from)]
#![allow(unused_unsafe)]
#![feature(stdsimd)] // for has_cpuid, dafuq rust.
#![feature(slice_index_methods)]

#[macro_use]
pub mod utils;

#[macro_use]
pub mod debug;

#[macro_use]
pub mod ffi;

#[macro_use]
pub mod monitor;

#[macro_use]
pub mod interrupts;

#[macro_use]
pub mod io;
pub mod keyboard;
pub mod multiboot;
pub mod panic;
pub mod registers;
#[cfg(not(feature = "test"))]
pub mod rust_main;
pub mod tests;
pub mod timer;

pub mod test_helpers;
