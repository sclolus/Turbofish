use super::*;

pub struct RustGlobalAlloc;

/// This globale variable may be use to check the faillible behavior of something
/// if is set manually to TRUE, It simulates an out of memory situation.
static mut HOOK_FAILLIBLE_CHECKER: bool = false;

#[allow(dead_code)]
pub fn set_faillible_context() {
    unsafe {
        HOOK_FAILLIBLE_CHECKER = true;
    }
}

#[allow(dead_code)]
pub fn unset_faillible_context() {
    unsafe {
        HOOK_FAILLIBLE_CHECKER = false;
    }
}

/// pointer returned by GlobalAlloc::alloc when requesting a 0 size
/// allocation
const DEVIL_POINTER: *mut u8 = 0x666 as *mut u8;

extern "C" {
    fn kmalloc(len: usize) -> *mut u8;
    fn kcalloc(count: usize, size: usize) -> *mut u8;
    fn kfree(ptr: *mut u8);
    fn krealloc(addr: *mut u8, new_size: usize) -> *mut u8;
}

unsafe impl GlobalAlloc for RustGlobalAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // This is for allocation of Empty struct rust, we can't
        // return 0x0 because it is considered as an allocErr by rust,
        // and we want to avoid alloc so we return DEVIL_POINTER
        if layout.size() == 0 && layout.align() == 1 {
            return DEVIL_POINTER;
        }
        // This condition is just made for checking faillible contextes
        if HOOK_FAILLIBLE_CHECKER == true {
            return 0x0 as *mut u8;
        }
        match &mut KERNEL_ALLOCATOR {
            KernelAllocator::Kernel => {
                if layout.align() <= 16 {
                    kmalloc(layout.size())
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
            KernelAllocator::Bootstrap(b) => {
                b.alloc_bootstrap(layout).unwrap_or(Virt(0x0)).0 as *mut u8
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if ptr == DEVIL_POINTER {
            return;
        }
        match &mut KERNEL_ALLOCATOR {
            KernelAllocator::Kernel => {
                if layout.align() <= 16 {
                    kfree(ptr);
                } else {
                    KERNEL_VIRTUAL_PAGE_ALLOCATOR
                        .as_mut()
                        .unwrap()
                        .free(Page::containing(Virt(ptr as usize)))
                        .expect("Cannot dealloc page");
                }
            }
            KernelAllocator::Bootstrap(_) => {
                panic!("Attempting to free while in bootstrap allocator")
            }
        }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        assert!(layout.align() <= 16);
        // This is for allocation of Empty struct rust, we can't
        // return 0x0 because it is considered as an allocErr by rust,
        // and we want to avoid alloc so we return DEVIL_POINTER
        if layout.size() == 0 && layout.align() == 1 {
            return DEVIL_POINTER;
        }
        match &mut KERNEL_ALLOCATOR {
            KernelAllocator::Kernel => {
                if layout.align() <= 16 {
                    kcalloc(1, layout.size())
                } else {
                    let ptr = self.alloc(layout);
                    if !ptr.is_null() {
                        ptr.write_bytes(0, layout.size());
                    }
                    ptr
                }
            }
            KernelAllocator::Bootstrap(_) => {
                let ptr = self.alloc(layout);
                if !ptr.is_null() {
                    ptr.write_bytes(0, layout.size());
                }
                ptr
            }
        }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // This is for allocation of Empty struct rust, we can't
        // return 0x0 because it is considered as an allocErr by rust,
        // and we want to avoid alloc so we return DEVIL_POINTER
        if layout.size() == 0 && layout.align() == 1 {
            return DEVIL_POINTER;
        }
        // This condition is just made for checking faillible contextes
        if HOOK_FAILLIBLE_CHECKER == true {
            return 0x0 as *mut u8;
        }
        match &mut KERNEL_ALLOCATOR {
            KernelAllocator::Kernel => {
                if layout.align() <= 16 {
                    krealloc(ptr, new_size)
                } else {
                    // The main KERNEL_VIRTUAL_PAGE_ALLOCATOR cannot realloc:
                    // Allocate with new_size -> copy from old_ptr -> free old_ptr
                    let new_ptr = KERNEL_VIRTUAL_PAGE_ALLOCATOR
                        .as_mut()
                        .unwrap()
                        .alloc(new_size.into(), AllocFlags::KERNEL_MEMORY)
                        .unwrap_or(Page::containing(Virt(0x0)))
                        .to_addr()
                        .0 as *mut u8;

                    if !new_ptr.is_null() {
                        let min = core::cmp::min(layout.size(), new_size);
                        new_ptr.copy_from(ptr, min);
                        KERNEL_VIRTUAL_PAGE_ALLOCATOR
                            .as_mut()
                            .unwrap()
                            .free(Page::containing(Virt(ptr as usize)))
                            .expect("Cannot dealloc page");
                    }
                    new_ptr
                }
            }
            KernelAllocator::Bootstrap(_) => {
                panic!("Attempting to realloc while in bootstrap allocator")
            }
        }
    }
}

#[alloc_error_handler]
#[cfg(not(test))]
fn out_of_memory(layout: core::alloc::Layout) -> ! {
    panic!(
        "Out of memory: Failed to allocate a rust data structure {:?}",
        layout
    );
}
