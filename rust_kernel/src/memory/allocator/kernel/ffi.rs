use super::*;

/// FFI safe function: Allocate Kernel physical Memory
/// kmalloc like a boss
#[no_mangle]
pub unsafe extern "C" fn kmalloc(size: usize, flags: AllocFlags) -> *mut u8 {
    match Layout::from_size_align(size, 16) {
        Err(_) => 0 as *mut u8,
        Ok(layout) => match &mut KERNEL_ALLOCATOR {
            KernelAllocator::Bootstrap(_) => panic!("Attempting to kmalloc while in bootstrap allocator"),
            KernelAllocator::Kernel(a) => {
                if layout.size() <= PAGE_SIZE && (flags & !AllocFlags::KERNEL_MEMORY).is_empty() {
                    a.alloc(layout).unwrap_or(Virt(0x0)).0 as *mut u8
                } else {
                    KERNEL_VIRTUAL_PAGE_ALLOCATOR
                        .as_mut()
                        .unwrap()
                        .alloc(layout.size().into(), AllocFlags::KERNEL_MEMORY)
                        .unwrap_or(Page::containing(Virt(0x0)))
                        .to_addr()
                        .0 as *mut u8
                }
            }
        },
    }
}

/// FFI safe function: De-allocate Kernel physical Memory
#[no_mangle]
pub unsafe extern "C" fn kfree(addr: *mut u8) {
    match &mut KERNEL_ALLOCATOR {
        KernelAllocator::Kernel(a) => {
            if let Err(_) = a.free(Virt(addr as usize)) {
                KERNEL_VIRTUAL_PAGE_ALLOCATOR
                    .as_mut()
                    .unwrap()
                    .free(Page::containing(Virt(addr as usize)))
                    .expect("Pointer being free'd was not allocated");
            }
        }
        KernelAllocator::Bootstrap(_) => panic!("Attempting to free while in bootstrap allocator"),
    }
}

/// FFI safe function: Get the internal size of a kmalloc allocation
#[no_mangle]
pub unsafe extern "C" fn ksize(addr: *mut u8) -> usize {
    match &mut KERNEL_ALLOCATOR {
        KernelAllocator::Kernel(a) => {
            let res = a.ksize(Virt(addr as usize));
            if let Err(_) = res {
                KERNEL_VIRTUAL_PAGE_ALLOCATOR
                    .as_mut()
                    .unwrap()
                    .ksize(Page::containing(Virt(addr as usize)))
                    .map(|nbr_pages| nbr_pages.to_bytes())
                    .unwrap()
            } else {
                res.unwrap()
            }
        }
        KernelAllocator::Bootstrap(_) => panic!("Bootstrap allocator does not implement ksize()"),
    }
}

/// FFI safe function: Allocate Kernel virtual Memory
#[no_mangle]
pub unsafe extern "C" fn vmalloc(size: usize) -> *mut u8 {
    assert!(in_kernel_mode(), "Bootstrap allocator does not implement vmalloc()");
    KERNEL_VIRTUAL_PAGE_ALLOCATOR
        .as_mut()
        .unwrap()
        .valloc(size.into(), AllocFlags::KERNEL_MEMORY)
        .unwrap_or(Page::containing(Virt(0x0)))
        .to_addr()
        .0 as *mut u8
}

/// FFI safe function: De-allocate Kernel virtual Memory
#[no_mangle]
pub unsafe extern "C" fn vfree(addr: *mut u8) {
    assert!(in_kernel_mode(), "Bootstrap allocator does not implement vfree()");
    KERNEL_VIRTUAL_PAGE_ALLOCATOR.as_mut().unwrap().free(Page::containing(Virt(addr as usize))).unwrap()
}

/// FFI safe function: Get the internal size of a vmalloc allocation
#[no_mangle]
pub unsafe extern "C" fn vsize(addr: *mut u8) -> usize {
    assert!(in_kernel_mode(), "Bootstrap allocator does not implement ksize()");
    KERNEL_VIRTUAL_PAGE_ALLOCATOR
        .as_mut()
        .unwrap()
        .ksize(Page::containing(Virt(addr as usize)))
        .map(|nbr_pages| nbr_pages.to_bytes())
        .unwrap()
}

/// FFI safe function: Map a physical addr
#[no_mangle]
pub unsafe extern "C" fn map(phy_addr: *mut u8, size: usize) -> *mut u8 {
    assert!(in_kernel_mode(), "Mapping memory while in bootstrap allocator is unsafe");
    let addr = Phys(phy_addr as usize);
    match KERNEL_VIRTUAL_PAGE_ALLOCATOR.as_mut().unwrap().map_addr(
        addr.into(),
        ((addr + size).align_next(PAGE_SIZE) - addr.align_prev(PAGE_SIZE)).into(),
        Entry::new(),
    ) {
        Err(_) => 0 as *mut u8,
        Ok(virt_addr) => (virt_addr.to_addr().0 as *mut u8).add(addr.offset()),
    }
}

/// FFI safe function: Unmap a physical addr corresponding to a physical addr
#[no_mangle]
pub unsafe extern "C" fn unmap(virt_addr: *mut u8, size: usize) -> i32 {
    assert!(in_kernel_mode(), "Unmapping memory while in bootstrap allocator is unsafe");
    let addr = Virt(virt_addr as usize);
    match KERNEL_VIRTUAL_PAGE_ALLOCATOR
        .as_mut()
        .unwrap()
        .unmap_addr(addr.into(), ((addr + size).align_next(PAGE_SIZE) - addr.align_prev(PAGE_SIZE)).into())
    {
        Err(_) => -1,
        Ok(_) => 0,
    }
}
