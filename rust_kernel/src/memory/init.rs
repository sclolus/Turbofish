use crate::memory::allocator::{
    init_kernel_virtual_allocator, init_physical_allocator, physical_page_allocator::DeviceMap,
};
//use crate::memory::mmu::*;
use crate::memory::tools::*;
//use crate::memory::*;

pub unsafe fn init_memory_system(system_memory_amount: NbrPages, device_map_ptr: *const DeviceMap) -> Result<()> {
    init_physical_allocator(system_memory_amount, device_map_ptr);
    init_kernel_virtual_allocator();

    Ok(())
}
