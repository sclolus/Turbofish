use super::{BuddyAllocator, SlabAllocator, VirtualPageAllocator};
use crate::memory::mmu::{Entry, PageDirectory, _enable_paging, BIOS_PAGE_TABLE, PAGE_TABLES};
use crate::memory::tools::*;
use alloc::boxed::Box;
use core::alloc::{GlobalAlloc, Layout};
use core::mem::size_of;

mod bootstrap;
use bootstrap::*;

mod rust_global_alloc;
pub use rust_global_alloc::RustGlobalAlloc;

pub mod ffi;
pub use ffi::*;

pub static mut KERNEL_ALLOCATOR: KernelAllocator = KernelAllocator::Bootstrap(BootstrapKernelAllocator::new());

pub static mut KERNEL_VIRTUAL_PAGE_ALLOCATOR: Option<VirtualPageAllocator> = None;

pub enum KernelAllocator {
    Bootstrap(BootstrapKernelAllocator),
    Kernel(SlabAllocator),
}

pub unsafe fn init_kernel_virtual_allocator() {
    let virt_start: Page<Virt> = Virt(symbol_addr!(virtual_offset)).into();
    let virt_end: Page<Virt> = Virt(symbol_addr!(kernel_virtual_end)).align_next(PAGE_SIZE).into();

    let mut buddy = BuddyAllocator::new(virt_start, NbrPages::_1GB);
    buddy.reserve_exact(virt_start, virt_end - virt_start).expect("failed to reserve the virtual kernel");

    // reserve the trics addresses in the buddy
    buddy.reserve_exact(Page::containing(Virt(0xFFC00000)), (1024 * 4096).into()).unwrap();

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

/// map the physical ptr into virtual memory
pub unsafe fn mmap<T>(phy_addr: *mut T, entry: Entry) -> Result<*mut T> {
    assert!(in_kernel_mode(), "Mapping memory while in bootstrap allocator is unsafe");
    let addr = Phys(phy_addr as usize);
    let size = size_of::<T>();
    let virt_addr = KERNEL_VIRTUAL_PAGE_ALLOCATOR.as_mut().unwrap().map_addr(
        Page::containing(addr),
        ((addr + size).align_next(PAGE_SIZE) - addr.align_prev(PAGE_SIZE)).into(),
        entry,
    )?;
    Ok((virt_addr.to_addr().0 as *mut u8).add(addr.offset()) as *mut T)
}

/// map the physical ptr into virtual memory
pub unsafe fn munmap<T>(phy_addr: *mut T) -> Result<()> {
    assert!(in_kernel_mode(), "UnMapping memory while in bootstrap allocator is unsafe");
    let addr = Virt(phy_addr as usize);
    let size = size_of::<T>();
    KERNEL_VIRTUAL_PAGE_ALLOCATOR
        .as_mut()
        .unwrap()
        .unmap_addr(Page::containing(addr), ((addr + size).align_next(PAGE_SIZE) - addr.align_prev(PAGE_SIZE)).into())
}

/// get the physical addr which is map to virtual addr `addr`
/// TODO: Should call get_physical_addr on the current process allocator and not the kernel_virtual_allocator
pub extern "C" fn get_physical_addr(addr: Virt) -> Option<Phys> {
    assert!(in_kernel_mode(), "call to get_physical_addr while in bootstrap allocator ");
    unsafe { KERNEL_VIRTUAL_PAGE_ALLOCATOR.as_mut().unwrap().get_physical_addr(addr) }
}

pub fn kmalloc<T>(flags: AllocFlags) -> *mut T {
    unsafe { ffi::kmalloc(size_of::<T>(), flags) as *mut T }
}

pub fn kfree<T>(t: *mut T) {
    unsafe {
        ffi::kfree(t as *mut u8);
    }
}

/// return true if the allocator is in kernel mode and not in bootstrap mode anymore
pub fn in_kernel_mode() -> bool {
    unsafe {
        match &KERNEL_ALLOCATOR {
            KernelAllocator::Bootstrap(_) => false,
            KernelAllocator::Kernel(_) => true,
        }
    }
}
