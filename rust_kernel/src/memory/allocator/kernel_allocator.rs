use super::virtual_page_allocator::{VirtualPageAllocator, KERNEL_VIRTUAL_PAGE_ALLOCATOR};
use super::KERNEL_VIRTUAL_MEMORY;
use crate::memory::mmu::{Entry, PageDirectory, _enable_paging, BIOS_PAGE_TABLE, PAGE_TABLES};
use crate::memory::tools::*;
use crate::memory::{BuddyAllocator, SlabAllocator};
use alloc::boxed::Box;
use core::alloc::{GlobalAlloc, Layout};

mod bootstrap;
use bootstrap::*;

pub static mut KERNEL_ALLOCATOR: KernelAllocator = KernelAllocator::Bootstrap(BootstrapKernelAllocator::new());

pub enum KernelAllocator {
    Bootstrap(BootstrapKernelAllocator),
    Kernel(SlabAllocator),
}

pub struct RustGlobalAlloc;

unsafe impl GlobalAlloc for RustGlobalAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        match &mut KERNEL_ALLOCATOR {
            KernelAllocator::Kernel(a) => {
                if layout.size() <= PAGE_SIZE {
                    a.alloc(layout).unwrap_or(Virt(0x0)).0 as *mut u8
                } else {
                    KERNEL_VIRTUAL_PAGE_ALLOCATOR
                        .as_mut()
                        .unwrap()
                        .alloc(layout.size().into())
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
                if layout.size() <= PAGE_SIZE {
                    a.free_with_size(Virt(ptr as usize), layout.size());
                } else {
                    KERNEL_VIRTUAL_PAGE_ALLOCATOR.as_mut().unwrap().free(Page::containing(Virt(ptr as usize))).unwrap()
                }
            }
            KernelAllocator::Bootstrap(_) => panic!("Attempting to free while in bootstrap allocator"),
        }
    }
}

/// FFI safe function: Allocate Kernel physical Memory
/// kmalloc like a boss
#[no_mangle]
pub unsafe extern "C" fn kmalloc(size: usize) -> *mut u8 {
    match Layout::from_size_align(size, 16) {
        Err(_) => 0 as *mut u8,
        Ok(layout) => match &mut KERNEL_ALLOCATOR {
            KernelAllocator::Bootstrap(_) => panic!("Attempting to kmalloc while in bootstrap allocator"),
            KernelAllocator::Kernel(a) => {
                if layout.size() <= PAGE_SIZE {
                    a.alloc(layout).unwrap_or(Virt(0x0)).0 as *mut u8
                } else {
                    KERNEL_VIRTUAL_PAGE_ALLOCATOR
                        .as_mut()
                        .unwrap()
                        .alloc(layout.size().into())
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
    match &mut KERNEL_ALLOCATOR {
        KernelAllocator::Kernel(_) => {
            KERNEL_VIRTUAL_PAGE_ALLOCATOR
                .as_mut()
                .unwrap()
                .valloc(size.into())
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
        KernelAllocator::Kernel(_) => {
            KERNEL_VIRTUAL_PAGE_ALLOCATOR.as_mut().unwrap().free(Page::containing(Virt(addr as usize))).unwrap()
        }
        KernelAllocator::Bootstrap(_) => panic!("Bootstrap allocator does not implement vfree()"),
    }
}

/// FFI safe function: Get the internal size of a vmalloc allocation
#[no_mangle]
pub unsafe extern "C" fn vsize(addr: *mut u8) -> usize {
    match &mut KERNEL_ALLOCATOR {
        KernelAllocator::Kernel(_) => KERNEL_VIRTUAL_PAGE_ALLOCATOR
            .as_mut()
            .unwrap()
            .ksize(Page::containing(Virt(addr as usize)))
            .map(|nbr_pages| nbr_pages.to_bytes())
            .unwrap(),
        KernelAllocator::Bootstrap(_) => panic!("Bootstrap allocator does not implement ksize()"),
    }
}

/// FFI safe function: Map a physical addr
#[no_mangle]
pub unsafe extern "C" fn map(phy_addr: *mut u8, mut size: usize) -> *mut u8 {
    match &mut KERNEL_ALLOCATOR {
        KernelAllocator::Kernel(_) => {
            // if addr is between two pages boundaries, increment size by one page
            if size % PAGE_SIZE != 0 {
                size += PAGE_SIZE;
            }
            match KERNEL_VIRTUAL_PAGE_ALLOCATOR.as_mut().unwrap().map_addr(Phys(phy_addr as usize).into(), size.into())
            {
                Err(_) => 0 as *mut u8,
                Ok(virt_addr) => (virt_addr.to_addr().0 as *mut u8).add(phy_addr as usize % PAGE_SIZE),
            }
        }
        KernelAllocator::Bootstrap(_) => panic!("Mapping memory while in bootstrap allocator is unsafe"),
    }
}

/// FFI safe function: Unmap a physical addr corresponding to a physical addr
#[no_mangle]
pub unsafe extern "C" fn unmap(virt_addr: *mut u8, mut size: usize) -> i32 {
    match &mut KERNEL_ALLOCATOR {
        KernelAllocator::Kernel(_) => {
            // if addr is between two pages boundaries, increment size by one page
            if size % PAGE_SIZE != 0 {
                size += PAGE_SIZE;
            }
            match KERNEL_VIRTUAL_PAGE_ALLOCATOR
                .as_mut()
                .unwrap()
                .unmap_addr(Virt(virt_addr as usize).into(), size.into())
            {
                Err(_) => -1,
                Ok(_) => 0,
            }
        }
        KernelAllocator::Bootstrap(_) => panic!("Unmapping memory while in bootstrap allocator is unsafe"),
    }
}

#[alloc_error_handler]
#[cfg(not(test))]
fn out_of_memory(_: core::alloc::Layout) -> ! {
    panic!("Out of memory: Failed to allocate a rust data structure");
}

pub unsafe fn init_kernel_virtual_allocator() {
    let virt_start: Page<Virt> = Virt(symbol_addr!(virtual_offset)).into();
    let virt_end: Page<Virt> = Virt(symbol_addr!(high_kernel_virtual_end)).align_next(PAGE_SIZE).into();

    let mut buddy = BuddyAllocator::new(virt_start, KERNEL_VIRTUAL_MEMORY);
    buddy.reserve_exact(virt_start, virt_end - virt_start).expect("failed to reserve the virtual kernel");

    let mut pd = Box::new(PageDirectory::new());
    pd.set_page_tables(0, &BIOS_PAGE_TABLE);
    pd.set_page_tables(768, &PAGE_TABLES);
    pd.map_range_page_init(Virt(0).into(), Phys(0).into(), NbrPages::_1MB, Entry::READ_WRITE | Entry::PRESENT)
        .expect("Could not identity map the first megabyte of memory");
    pd.map_range_page_init(virt_start, Page::new(0), virt_end - virt_start, Entry::READ_WRITE | Entry::PRESENT)
        .expect("Init: Could not map the kernel");

    let raw_pd = pd.as_mut();
    let phys_pd = Phys(raw_pd as *mut PageDirectory as usize - symbol_addr!(virtual_offset));

    pd.self_map_tricks(phys_pd);

    _enable_paging(phys_pd);

    let virt = VirtualPageAllocator::new(buddy, pd);

    KERNEL_VIRTUAL_PAGE_ALLOCATOR = Some(virt);
    KERNEL_ALLOCATOR = KernelAllocator::Kernel(SlabAllocator::new());
}
