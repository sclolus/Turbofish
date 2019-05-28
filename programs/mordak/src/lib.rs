#![cfg_attr(not(test), no_std)]
#![cfg_attr(test, feature(allocator_api))]
#![feature(alloc)]
#![feature(alloc_error_handler)]
#![feature(asm)]

pub mod memory;
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

#[panic_handler]
#[no_mangle]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{:#X?}", info);
    loop {}
}

#[no_mangle]
fn main() -> i32 {
    println!("initialise Mordak's Sodo-test");

    use alloc::vec;
    use alloc::vec::Vec;

    use crate::math::random::{srand, srand_init};;

    fn make_somization<T: Fn() -> usize>(max_alloc: usize, alloc_size_fn: T) -> Result<(), ()> {
        const MAX_ALLOCATION_ARRAY_SIZE: usize = 1024;
        const NB_TESTS: usize = 1024;

        #[derive(Debug, Clone, PartialEq, Eq)]
        struct Allocation {
            size: usize,
            random_u8: u8,
            v: Vec<u8>,
        }
        let mut s: [Option<Allocation>; MAX_ALLOCATION_ARRAY_SIZE] = unsafe { core::mem::zeroed() };

        // this is a default for Rust to initialize as None when zeroed. but we cannot ensure that it will be always true in future
        for i in s.iter_mut() {
            *i = None;
        }

        // check if fields are as None
        for i in s.iter() {
            assert_eq!(*i, None);
        }

        let mut nb_allocations: usize = 0;

        for _i in 0..NB_TESTS {
            match srand::<bool>(true) {
                true => {
                    if max_alloc != nb_allocations {
                        let n: u8 = srand(core::u8::MAX);
                        let size = alloc_size_fn();
                        let new_alloc = Allocation { size: size, random_u8: n, v: vec![n; size] };
                        s[nb_allocations] = Some(new_alloc);
                        nb_allocations += 1;
                    }
                }
                false => match nb_allocations {
                    0 => {}
                    _ => {
                        let elmt_number = srand((nb_allocations - 1) as u32) as usize;
                        let elmt = s[elmt_number].take().unwrap();
                        for i in 0..elmt.size {
                            assert_eq!(elmt.random_u8, elmt.v[i], "i: {}", _i);
                            if elmt.random_u8 != elmt.v[i] {
                                return Err(());
                            }
                        }
                        drop(elmt);
                        if elmt_number != nb_allocations - 1 {
                            s[elmt_number] = s[nb_allocations - 1].take();
                        }
                        nb_allocations -= 1;
                    }
                },
            }
        }
        Ok(())
    }

    srand_init(42).unwrap();

    make_somization(1024, || srand::<u32>(512) as usize).expect("failed sodo 0");
    println!("test 1 passed");
    make_somization(1024, || srand::<u32>(16) as usize * 128).expect("failed sodo 1");
    println!("test 2 passed");
    make_somization(1024, || srand::<u32>(32) as usize * 128).expect("failed sodo 2");
    println!("test 3 passed");
    make_somization(1024, || srand::<u32>(64) as usize * 128).expect("failed sodo 3");
    println!("test 4 passed");
    make_somization(1024 * 4, || srand::<u32>(4096) as usize).expect("failed sodo 4");
    println!("test 5 passed");

    0
}
