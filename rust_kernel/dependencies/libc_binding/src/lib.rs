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
pub type Pid = i32;

#[derive(Debug)]
pub struct InvalidSignum;

use core::convert::TryFrom;
use core::mem::transmute;
/// TryFrom boilerplate to get a Signum relative to raw value
impl TryFrom<u32> for Signum {
    type Error = InvalidSignum;
    fn try_from(n: u32) -> Result<Self, Self::Error> {
        if n >= 32 {
            return Err(InvalidSignum);
        } else {
            Ok(unsafe { transmute(n) })
        }
    }
}

impl Default for termios {
    fn default() -> Self {
        termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: (ECHO | ICANON | ISIG),
            c_cc: [0; 42],
        }
    }
}
