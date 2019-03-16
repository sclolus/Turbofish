use crate::interrupts;
use crate::io::UART_16550;
use crate::memory;
use crate::memory::allocator::physical_page_allocator::DeviceMap;
use crate::multiboot::MultibootInfo;
use crate::tests::helpers::exit_qemu;

#[no_mangle]
pub extern "C" fn kmain(multiboot_info: *const MultibootInfo, device_map_ptr: *const DeviceMap) -> u32 {
    unsafe {
        UART_16550.init();
    }
    let multiboot_info: MultibootInfo = unsafe { *multiboot_info };
    unsafe {
        interrupts::init();
    }
    crate::watch_dog();
    unsafe {
        memory::init_memory_system(multiboot_info.get_memory_amount_nb_pages(), device_map_ptr).unwrap();
    }
    crate::watch_dog();

    use crate::math::random::srand;

    fn make_somization<T: Fn() -> usize>(nb_tests: usize, max_alloc: usize, alloc_size_fn: T) -> Result<(), ()> {
        use alloc::vec;
        use alloc::vec::Vec;

        const MAX_ALLOCATION_ARRAY_SIZE: usize = 1 << 16;
        if max_alloc > MAX_ALLOCATION_ARRAY_SIZE {
            return Err(());
        }

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

        for _i in 0..nb_tests {
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
    crate::math::random::srand_init(42).unwrap();
    make_somization(1024, 1000, || 4096).expect("failed sodo 0");
    make_somization(1024, 1000, || srand::<u32>(16) as usize * 4096).expect("failed sodo 1");
    make_somization(1024, 1000, || srand::<u32>(32) as usize * 4096).expect("failed sodo 2");
    make_somization(1024, 1000, || srand::<u32>(64) as usize * 4096).expect("failed sodo 3");

    exit_qemu(0);
    0
}
