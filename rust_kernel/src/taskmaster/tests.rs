//! This file contains some dummy process tests

pub mod rust_kernel_processes;

pub mod klibc;

#[allow(unused_imports)]
use klibc::{_user_exit, _user_fork, _user_write};

extern "C" {
    pub static _dummy_asm_process_code: u8;
    pub static _dummy_asm_process_len: usize;

    pub static dummy_c_process: u8;

    pub static kernel_stack: u8;
}
