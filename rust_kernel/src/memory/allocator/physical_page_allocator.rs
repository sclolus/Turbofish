use super::BuddyAllocator;
use crate::memory::tools::*;
use bitflags::bitflags;

#[derive(Debug)]
pub struct PhysicalPageAllocator {
    allocator: BuddyAllocator<Phys>,
}

bitflags! {
    pub struct AllocFlags: u32 {
        const KERNEL_MEMORY = 1 << 0;
        const USER_MEMORY = 1 << 0;
    }
}

impl PhysicalPageAllocator {
    pub fn new(phys_start: Page<Phys>, size: NbrPages) -> Self {
        Self { allocator: BuddyAllocator::new(phys_start, size) }
    }
    /// size in bytes
    pub fn alloc(&mut self, size: NbrPages, flags: AllocFlags) -> Result<Page<Phys>> {
        if flags.contains(AllocFlags::KERNEL_MEMORY) {
            let order = size.into();
            let res = self.allocator.alloc(order)?;
            assert!(res.to_addr().0 > symbol_addr!(high_kernel_physical_start));
            // eprintln!("{:x?}", res.to_addr());
            Ok(res)
        } else {
            unimplemented!()
        }
    }

    pub fn reserve(&mut self, addr: Page<Phys>, size: NbrPages) -> Result<()> {
        self.allocator.reserve_exact(addr, size)
    }
    /// size in bytes
    pub fn free(&mut self, paddr: Page<Phys>) -> Result<()> {
        let order = self.ksize(paddr)?.into();

        Ok(self.allocator.free(paddr, order)?)
    }

    pub fn ksize(&mut self, paddr: Page<Phys>) -> Result<NbrPages> {
        Ok(self.allocator.ksize(paddr)?.nbr_pages())
    }
}

pub static mut PHYSICAL_ALLOCATOR: Option<PhysicalPageAllocator> = None;

pub unsafe fn init_physical_allocator(system_memory_amount: NbrPages, device_map: &[DeviceMap]) {
    eprintln!("kernel physical end: {:x?}", symbol_addr!(high_kernel_physical_end));
    eprintln!(
        "kernel physical end alligned: {:x?}",
        Phys(symbol_addr!(high_kernel_physical_end)).align_next(PAGE_SIZE)
    );

    let mut pallocator = PhysicalPageAllocator::new(Page::new(0), system_memory_amount);

    pallocator.reserve(Page::new(0), (symbol_addr!(high_kernel_physical_end) - 0).into()).unwrap();
    // Reserve in memory the regions that are not usable according to the device_map slice,
    // making them effectively unusable by the memory system.
    for region in device_map.iter() {
        println!("{:x?}", region);
        println!("addr: {:x?}", region.low_addr);
        println!("len: {}ko", region.low_length >> 10);
        if RegionType::Usable == region.region_type {
            continue;
        }
        if let Err(e) =
            pallocator.reserve(Page::containing(Phys(region.low_addr as usize)), (region.low_length as usize).into())
        {
            println!("some error were occured on pallocator ! {:?}", e);
        }
    }
    PHYSICAL_ALLOCATOR = Some(pallocator);
}
