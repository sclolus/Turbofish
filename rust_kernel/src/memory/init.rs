use crate::memory::allocator::{init_kernel_virtual_allocator, init_physical_allocator};
//use crate::memory::mmu::*;
use crate::memory::tools::*;
//use crate::memory::*;

pub unsafe fn init_memory_system(_system_memory_amount: NbrPages) -> Result<(), ()> {
    init_physical_allocator();
    init_kernel_virtual_allocator();

    Ok(())
}
