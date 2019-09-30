use super::*;

#[no_mangle]
pub unsafe extern "C" fn kreserve(virt: *mut u8, phys: *mut u8, size: usize) -> *mut u8 {
    match &mut KERNEL_ALLOCATOR {
        KernelAllocator::Bootstrap(_) => {
            panic!("Attempting to kreserve while in bootstrap allocator")
        }
        KernelAllocator::Kernel => {
            HIGH_KERNEL_MEMORY
                .as_mut()
                .unwrap()
                .reserve(
                    Virt(virt as usize).into(),
                    Phys(phys as usize).into(),
                    size.into(),
                )
                .map(|_| Page::containing(Virt(virt as usize)))
                .unwrap_or(Page::containing(Virt(0x0)))
                .to_addr()
                .0 as *mut u8
        }
    }
}

/// FFI safe function: Allocate Kernel virtual Memory
#[no_mangle]
pub unsafe extern "C" fn vmalloc(size: usize) -> *mut u8 {
    match &mut KERNEL_ALLOCATOR {
        KernelAllocator::Kernel => {
            KERNEL_VIRTUAL_PAGE_ALLOCATOR
                .as_mut()
                .unwrap()
                .valloc(size.into(), AllocFlags::KERNEL_MEMORY)
                .unwrap_or(Page::containing(Virt(0x0)))
                .to_addr()
                .0 as *mut u8
        }
        KernelAllocator::Bootstrap(_) => panic!("Bootstrap allocator does not implement vmalloc()"),
    }
}

/// FFI safe function: De-allocate Kernel virtual Memory
#[no_mangle]
pub unsafe extern "C" fn vfree(addr: *mut u8) {
    match &mut KERNEL_ALLOCATOR {
        KernelAllocator::Kernel => KERNEL_VIRTUAL_PAGE_ALLOCATOR
            .as_mut()
            .unwrap()
            .free(Page::containing(Virt(addr as usize)))
            .expect("Cannot free Kernel virtual Memory"),
        KernelAllocator::Bootstrap(_) => panic!("Bootstrap allocator does not implement vfree()"),
    }
}

/// FFI safe function: Get the internal size of a vmalloc allocation
#[no_mangle]
pub unsafe extern "C" fn vsize(addr: *mut u8) -> usize {
    match &mut KERNEL_ALLOCATOR {
        KernelAllocator::Kernel => KERNEL_VIRTUAL_PAGE_ALLOCATOR
            .as_mut()
            .unwrap()
            .ksize(Page::containing(Virt(addr as usize)))
            .map(|nbr_pages| nbr_pages.to_bytes())
            .expect("Cannot get vsize of valloced allocation"),
        KernelAllocator::Bootstrap(_) => panic!("Bootstrap allocator does not implement ksize()"),
    }
}

/// FFI safe function: Map a physical addr
#[no_mangle]
pub unsafe extern "C" fn map(phy_addr: *mut u8, size: usize) -> *mut u8 {
    match &mut KERNEL_ALLOCATOR {
        KernelAllocator::Kernel => {
            let addr = Phys(phy_addr as usize);
            match KERNEL_VIRTUAL_PAGE_ALLOCATOR.as_mut().unwrap().map_addr(
                addr.into(),
                ((addr + size).align_next(PAGE_SIZE) - addr.align_prev(PAGE_SIZE)).into(),
            ) {
                Err(_) => 0 as *mut u8,
                Ok(virt_addr) => (virt_addr.to_addr().0 as *mut u8).add(addr.offset()),
            }
        }
        KernelAllocator::Bootstrap(_) => {
            panic!("Mapping memory while in bootstrap allocator is unsafe")
        }
    }
}

/// FFI safe function: Unmap a physical addr corresponding to a physical addr
#[no_mangle]
pub unsafe extern "C" fn unmap(virt_addr: *mut u8, size: usize) -> i32 {
    match &mut KERNEL_ALLOCATOR {
        KernelAllocator::Kernel => {
            let addr = Virt(virt_addr as usize);
            match KERNEL_VIRTUAL_PAGE_ALLOCATOR.as_mut().unwrap().unmap_addr(
                addr.into(),
                ((addr + size).align_next(PAGE_SIZE) - addr.align_prev(PAGE_SIZE)).into(),
            ) {
                Err(_) => -1,
                Ok(_) => 0,
            }
        }
        KernelAllocator::Bootstrap(_) => {
            panic!("Unmapping memory while in bootstrap allocator is unsafe")
        }
    }
}

/// get the physical addr which is map to virtual addr `addr`
/// TODO: Should call get_physical_addr on the current process allocator and not the kernel_virtual_allocator
pub extern "C" fn get_physical_addr(addr: Virt) -> Option<Phys> {
    unsafe {
        match &mut KERNEL_ALLOCATOR {
            KernelAllocator::Kernel => KERNEL_VIRTUAL_PAGE_ALLOCATOR
                .as_mut()
                .unwrap()
                .get_physical_addr(addr),
            KernelAllocator::Bootstrap(_) => {
                panic!("call to get_physical_addr while in bootstrap allocator ")
            }
        }
    }
}

/// Unsafe function. Usable by C allocator
#[no_mangle]
unsafe fn get_kernel_pages(len: usize) -> *mut u8 {
    match KERNEL_VIRTUAL_PAGE_ALLOCATOR
        .as_mut()
        .unwrap()
        .alloc(len.into(), AllocFlags::KERNEL_MEMORY)
    {
        Ok(addr) => addr.to_addr().0 as _,
        Err(_) => -1 as _,
    }
}

/// Unsafe function. Usable by C allocator
#[no_mangle]
unsafe fn free_kernel_pages(addr: *mut u8, _len: usize) -> i32 {
    KERNEL_VIRTUAL_PAGE_ALLOCATOR
        .as_mut()
        .unwrap()
        .free(Page::containing(Virt(addr as usize)))
        .expect("Pointer being free'd was not allocated");
    0
}
