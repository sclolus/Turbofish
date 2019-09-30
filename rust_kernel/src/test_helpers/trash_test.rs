//! This module contains trash code test that can be used by others when we do a merge requests

pub fn sa_va_castagner() {
    panic!("you failed");
}

// rust inline this fonction by default even with the --force frame pointer. To investigate
#[inline(never)]
pub fn kpanic() {
    unsafe {
        *(0x10000000 as *mut u8) = 0x42;
    }
}
