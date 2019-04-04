use crate::memory::allocator::{init_kernel_virtual_allocator, init_physical_allocator};
//use crate::memory::mmu::*;
use crate::memory::tools::*;
//use crate::memory::*;

pub unsafe fn init_memory_system(system_memory_amount: NbrPages, device_map_ptr: &[DeviceMap]) -> Result<()> {
    init_physical_allocator(system_memory_amount, device_map_ptr);
    init_kernel_virtual_allocator();

    Ok(())
}
