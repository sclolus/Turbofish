#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![no_std]
#![feature(compiler_builtins_lib)]

//extern crate rlibc;

pub mod support; // For Rust lang items


#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut u8,
                                src: *const u8,
                                n: usize)
                                -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
    dest
}

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i16, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.offset(i as isize) = c as u8;
        i += 1;
    }
    s
}

#[no_mangle]
pub extern "C" fn kmain() {
    loop { }
}

