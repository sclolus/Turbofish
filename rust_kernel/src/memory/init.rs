use crate::memory::allocator::{init_kernel_virtual_allocator, init_physical_allocator};
//use crate::memory::mmu::*;
//use crate::memory::tools::*;
//use crate::memory::*;

pub unsafe fn init_memory_system() -> Result<(), ()> {
    init_physical_allocator();
    init_kernel_virtual_allocator();

    Ok(())
}
