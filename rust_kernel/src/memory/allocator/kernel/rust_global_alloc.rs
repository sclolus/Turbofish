use super::*;

pub struct RustGlobalAlloc;

unsafe impl GlobalAlloc for RustGlobalAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match &mut KERNEL_ALLOCATOR {
            KernelAllocator::Kernel(a) => {
                if layout.size() <= PAGE_SIZE && layout.align() <= 16 {
                    // TODO: Handle the align layout in SlabAllocator then remove 16
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
            KernelAllocator::Bootstrap(b) => b.alloc_bootstrap(layout).unwrap_or(Virt(0x0)).0 as *mut u8,
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        match &mut KERNEL_ALLOCATOR {
            KernelAllocator::Kernel(a) => {
                if layout.size() <= PAGE_SIZE && layout.align() <= 16 {
                    // TODO: Handle the align layout in SlabAllocator then remove 16
                    a.free_with_size(Virt(ptr as usize), layout.size());
                } else {
                    KERNEL_VIRTUAL_PAGE_ALLOCATOR.as_mut().unwrap().free(Page::containing(Virt(ptr as usize))).unwrap()
                }
            }
            KernelAllocator::Bootstrap(_) => panic!("Attempting to free while in bootstrap allocator"),
        }
    }
}

#[alloc_error_handler]
#[cfg(not(test))]
fn out_of_memory(_: core::alloc::Layout) -> ! {
    panic!("Out of memory: Failed to allocate a rust data structure");
}
