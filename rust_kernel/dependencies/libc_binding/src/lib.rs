#![allow(non_camel_case_types)]
#![cfg_attr(not(test), no_std)]
pub mod libc;
pub use libc::*;
// ::std::os::raw::c_char
pub type c_char = i8;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_void = i32;
pub type c_longlong = i64;
pub type c_long = i32;
pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
