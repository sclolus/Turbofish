use super::{BuddyAllocator, SlabAllocator, VirtualPageAllocator};
use crate::memory::mmu::{Entry, PageDirectory, _enable_paging, BIOS_PAGE_TABLE, PAGE_TABLES};
use crate::memory::tools::*;
use alloc::boxed::Box;
use core::alloc::{GlobalAlloc, Layout};

mod bootstrap;
use bootstrap::*;

mod rust_global_alloc;
pub use rust_global_alloc::{set_faillible_context, unset_faillible_context, RustGlobalAlloc};

pub mod ffi;
pub use ffi::*;

pub static mut KERNEL_ALLOCATOR: KernelAllocator =
    KernelAllocator::Bootstrap(BootstrapKernelAllocator::new());

pub static mut KERNEL_VIRTUAL_PAGE_ALLOCATOR: Option<VirtualPageAllocator> = None;

pub enum KernelAllocator {
    Bootstrap(BootstrapKernelAllocator),
    Kernel(SlabAllocator),
}

pub unsafe fn init_kernel_virtual_allocator() {
    let virt_start: Page<Virt> = Virt(symbol_addr!(virtual_offset)).into();
    let virt_end: Page<Virt> = Virt(symbol_addr!(kernel_virtual_end))
        .align_next(PAGE_SIZE)
        .into();

    let mut buddy =
        BuddyAllocator::new(virt_start, NbrPages::_1GB).expect("new kernel buddy failed");
    buddy
        .reserve_exact(virt_start, virt_end - virt_start)
        .expect("failed to reserve the virtual kernel");

    // reserve the trics addresses in the buddy
    buddy
        .reserve_exact(Page::containing(Virt(0xFFC00000)), (1024 * 4096).into())
        .expect("Cannot reserve the MMU tricks area");

    let mut pd = Box::new(PageDirectory::new());
    pd.set_page_tables(0, &BIOS_PAGE_TABLE);
    pd.set_page_tables(768, &PAGE_TABLES);
    pd.map_range_page_init(
        Virt(0).into(),
        Phys(0).into(),
        NbrPages::_1MB,
        Entry::READ_WRITE | Entry::PRESENT,
    )
    .expect("Could not identity map the first megabyte of memory");
    pd.map_range_page_init(
        virt_start,
        Page::new(0),
        virt_end - virt_start,
        Entry::READ_WRITE | Entry::PRESENT,
    )
    .expect("Init: Could not map the kernel");

    pd.unmap_page_init(Virt(symbol_addr!(stack_overflow_zone)).into())
        .expect("Init: Could not unmap the stack overflow zone");
    let raw_pd = pd.as_mut();
    let phys_pd = Phys(raw_pd as *mut PageDirectory as usize - symbol_addr!(virtual_offset));

    pd.self_map_tricks(phys_pd);

    _enable_paging(phys_pd);

    let virt = VirtualPageAllocator::new(buddy, pd);

    KERNEL_VIRTUAL_PAGE_ALLOCATOR = Some(virt);
    KERNEL_ALLOCATOR = KernelAllocator::Kernel(SlabAllocator::new());
}
