#![cfg_attr(not(test), no_std)]
#![cfg_attr(test, allow(unused_imports))]
#![feature(const_raw_ptr_to_usize_cast)] // rust is being annoying on the types of ffi functions
#![feature(core_intrinsics)] // for function! macro
#![feature(asm)] // for inline asm
#![feature(stdsimd)] // for has_cpuid, dafuq rust.
#![feature(slice_index_methods)]
#![cfg_attr(test, feature(allocator_api))]
#![feature(alloc_error_handler)]
#![feature(underscore_const_names)]
#![feature(stmt_expr_attributes)]
#![feature(try_reserve)]
#![feature(vec_remove_item)]
#![feature(type_alias_enum_variants)]
#![feature(const_vec_new)]
// #![deny(missing_docs)]

extern crate alloc;

// Our Crates
extern crate io;
extern crate keyboard;
extern crate mbr;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate terminal;

#[macro_use]
pub mod utils;

#[macro_use]
pub mod debug;

#[macro_use]
pub mod ffi;

#[macro_use]
pub mod interrupts;

#[macro_use]
pub mod system;
pub mod taskmaster;
#[macro_use]
pub mod drivers;
pub mod math;
pub mod memory;
pub mod multiboot;
pub mod panic;
pub mod registers;
#[cfg(not(feature = "test"))]
pub mod rust_main;
pub mod tests;
pub mod timer;

pub mod watch_dog;
pub use watch_dog::*;

pub use sync::{Spinlock, SpinlockGuard};
pub mod elf_loader;

use crate::memory::RustGlobalAlloc;

/// As a matter of fact, we can't declare the MemoryManager inside a submodule.
#[cfg(not(test))]
#[global_allocator]
static MEMORY_MANAGER: RustGlobalAlloc = RustGlobalAlloc;
pub mod test_helpers;
