#![feature(const_raw_ptr_to_usize_cast)] // rust is being annoying on the types of ffi functions
#![no_std]
#![feature(core_intrinsics)] // for function! macro
#![feature(asm)] // for inline asm
#![feature(try_from)]
#![allow(unused_unsafe)]
#![feature(stdsimd)] // for has_cpuid, dafuq rust.

#[macro_use]
pub mod debug;

#[macro_use]
pub mod ffi;

#[macro_use]
pub mod monitor;
#[macro_use]
pub mod interrupts;
pub mod io;
pub mod multiboot;
pub mod panic;
pub mod registers;
pub mod rust_main;
pub mod timer;
