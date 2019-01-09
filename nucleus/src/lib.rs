#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![no_std]
#![feature(compiler_builtins_lib)]

//extern crate rlibc;

pub mod support; // For Rust lang items

#[no_mangle]
pub extern "C" fn kmain() {
    loop { }
}

