use super::BuddyAllocator;
use crate::memory::tools::*;

#[derive(Debug)]
pub struct PhysicalPageAllocator {
    allocator: BuddyAllocator<Phys>,
}

impl PhysicalPageAllocator {
    pub fn new(phys_start: Page<Phys>, size: NbrPages) -> Self {
        Self { allocator: BuddyAllocator::new(phys_start, size).expect("new physical buddy failed") }
    }

    pub fn alloc(&mut self, size: NbrPages, _flags: AllocFlags) -> Result<Page<Phys>> {
        // Commented this as this is currently handled by the upper abstraction.
        // This is to be modified eventually.
        // if flags.contains(AllocFlags::KERNEL_MEMORY) {
        let order = size.into();
        let res = self.allocator.alloc(order)?;
        // eprintln!("{:x?}", res.to_addr());
        Ok(res)
        // } else {
        //     unimplemented!()
        // }
    }

    pub fn reserve(&mut self, addr: Page<Phys>, size: NbrPages) -> Result<()> {
        self.allocator.reserve_exact(addr, size)
    }

    pub fn free(&mut self, paddr: Page<Phys>) -> Result<NbrPages> {
        let nbr_pages = self.ksize(paddr)?;
        let order = nbr_pages.into();
        self.allocator.free(paddr, order)?;
        Ok(nbr_pages)
    }

    pub fn ksize(&mut self, paddr: Page<Phys>) -> Result<NbrPages> {
        Ok(self.allocator.ksize(paddr)?.nbr_pages())
    }
}

pub static mut PHYSICAL_ALLOCATOR: Option<PhysicalPageAllocator> = None;

pub unsafe fn init_physical_allocator(system_memory_amount: NbrPages, device_map: &[DeviceMap]) {
    //eprintln!("kernel physical end: {:x?}", symbol_addr!(kernel_physical_end));
    //eprintln!("kernel physical end alligned: {:x?}", Phys(symbol_addr!(kernel_physical_end)).align_next(PAGE_SIZE));

    let mut pallocator = PhysicalPageAllocator::new(Page::new(0), system_memory_amount);

    pallocator.reserve(Page::new(0), (symbol_addr!(kernel_physical_end) - 0).into()).unwrap();
    // Reserve in memory the regions that are not usable according to the device_map slice,
    // making them effectively unusable by the memory system.
    for region in device_map.iter() {
        //println!("{:x?}", region);
        //println!("addr: {:x?}", region.low_addr);
        //println!("len: {}ko", region.low_length >> 10);
        if RegionType::Usable == region.region_type {
            continue;
        }
        if let Err(_e) =
            pallocator.reserve(Page::containing(Phys(region.low_addr as usize)), (region.low_length as usize).into())
        {
            //println!("some error were occured on pallocator ! {:?}", e);
        }
    }
    PHYSICAL_ALLOCATOR = Some(pallocator);
}
