use crate::math::random::srand;
use core::slice;

pub fn make_somization<T: Fn() -> usize>(
    nb_tests: usize,
    max_alloc: usize,
    allocator: unsafe extern "C" fn(usize) -> *mut u8,
    deallocator: unsafe extern "C" fn(*mut u8),
    size_verifier: unsafe extern "C" fn(*mut u8) -> usize,
    alloc_size_fn: T,
) -> Result<(), ()> {
    const MAX_ALLOCATION_ARRAY_SIZE: usize = 1 << 16;
    if max_alloc > MAX_ALLOCATION_ARRAY_SIZE {
        return Err(());
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Allocation {
        size: usize,
        random_u8: u8,
        v: *mut u8,
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

    for _i in 0..nb_tests {
        match srand::<bool>(true) {
            true => {
                if max_alloc != nb_allocations {
                    let n: u8 = srand(core::u8::MAX);
                    let size = alloc_size_fn();
                    let new_alloc = Allocation { size: size, random_u8: n, v: unsafe { allocator(size) } };
                    assert_ne!(new_alloc.v, 0 as *mut u8);
                    let size = unsafe { size_verifier(new_alloc.v) };
                    assert!(size >= new_alloc.size);
                    let ptr = unsafe { slice::from_raw_parts_mut(new_alloc.v, new_alloc.size) };
                    for elmt in ptr.iter_mut() {
                        *elmt = n;
                    }
                    s[nb_allocations] = Some(new_alloc);
                    nb_allocations += 1;
                }
            }
            false => match nb_allocations {
                0 => {}
                _ => {
                    let elmt_number = srand((nb_allocations - 1) as u32) as usize;
                    let elmt = s[elmt_number].take().unwrap();
                    let ptr = unsafe { slice::from_raw_parts(elmt.v, elmt.size) };
                    for i in 0..elmt.size {
                        assert_eq!(elmt.random_u8, ptr[i], "i: {}", _i);
                    }
                    unsafe {
                        deallocator(elmt.v);
                    }
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
