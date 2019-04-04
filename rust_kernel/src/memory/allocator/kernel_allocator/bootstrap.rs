//! This file contains the bootstrap allocator used to allocate the kernel allocator.

use crate::memory::tools::*;
use core::alloc::Layout;

/// 4 MB for the bootstrap
const MEMORY_BOOTSTRAP_KERNEL_ALLOCATOR: usize = 0x400_000;

static mut BSS_MEMORY: [u8; MEMORY_BOOTSTRAP_KERNEL_ALLOCATOR] = [0; MEMORY_BOOTSTRAP_KERNEL_ALLOCATOR];

#[derive(Debug)]
pub struct BootstrapKernelAllocator {
    current_offset: usize,
}

impl BootstrapKernelAllocator {
    pub const fn new() -> Self {
        BootstrapKernelAllocator { current_offset: 0 }
    }
    pub unsafe fn alloc_bootstrap(&mut self, layout: Layout) -> Result<Virt> {
        let base_address = &BSS_MEMORY[0] as *const u8 as usize;
        let mut address = Virt(&BSS_MEMORY[self.current_offset] as *const u8 as usize);
        address = address.align_next(layout.align());
        self.current_offset = address.0 - base_address + layout.size();
        if self.current_offset > BSS_MEMORY.len() {
            panic!("No more bootstrap memory");
        }
        Ok(address)
    }
}
