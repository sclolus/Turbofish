//! This crate is a tiny rust sodo static lib linked with a static tiny libc

#![cfg_attr(not(test), no_std)]
#![cfg_attr(test, feature(allocator_api))]
#![feature(alloc)]
#![feature(alloc_error_handler)]
#![feature(asm)]
#![feature(core_intrinsics)]
#![warn(missing_docs)]

pub mod memory;
#[cfg(not(test))]
use crate::memory::RustGlobalAlloc;

/// As a matter of fact, we can't declare the MemoryManager inside a submodule.
#[cfg(not(test))]
#[global_allocator]
static MEMORY_MANAGER: RustGlobalAlloc = RustGlobalAlloc;

extern crate alloc;

#[macro_use]
pub mod writer;
pub use writer::*;

pub mod math;

#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{:#X?}", info);
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
fn main() -> i32 {
    println!("initialise Sleeper's Sodo-test");

    use crate::math::random::{srand, srand_init};;

    srand_init(42).unwrap();

    loop {
        let seconds: u32 = srand::<u32>(3) + 1;

        println!("I will on sleeping for {:?} seconds", seconds);

        let ret = unsafe { sleep(seconds) };

        if ret != 0 {
            println!(
                "Ny sleep was interrupted !!! I remain {:?} seconds ...",
                ret
            );
        }

        println!("Now, il attempt to sleept with nano.");

        let input: Timespec = Timespec {
            seconds: srand::<u32>(1),
            nanoseconds: (srand::<u32>(1000) * 1000000) as i32,
        };
        let mut output: Timespec = Timespec {
            ..Default::default()
        };

        println!("I will on nano sleeping , my time struct is {:#?}", input);

        let ret = unsafe { nanosleep(&input as *const _, &mut output as *mut _) };

        if ret == -1 {
            println!(
                "Ny nanosleep was interrupted !!! I time struct remain is {:#?} ...",
                output
            );
        }
    }
    #[allow(unreachable_code)]
    0
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(C)]
struct Timespec {
    seconds: u32,
    nanoseconds: i32,
}

extern "C" {
    fn sleep(seconds: u32) -> u32;
    fn nanosleep(req: *const Timespec, rem: *mut Timespec) -> i32;
}
