use super::BuddyAllocator;
use crate::memory::allocator::buddy_allocator::Order;
use crate::memory::tools::*;

#[derive(Debug)]
pub struct PhysicalPageAllocator {
    allocator: BuddyAllocator<Phys>,
}

impl PhysicalPageAllocator {
    pub fn new(phys_start: Page<Phys>, size: NbrPages) -> Self {
        Self { allocator: BuddyAllocator::new(phys_start, size) }
    }

    pub fn alloc(&mut self, size: NbrPages, flags: AllocFlags) -> Result<Page<Phys>> {
        if flags.contains(AllocFlags::KERNEL_MEMORY) {
            let order = size.into();
            let res = self.allocator.alloc(order)?;
            // eprintln!("{:x?}", res.to_addr());
            Ok(res)
        } else {
            unimplemented!()
        }
    }

    pub fn reserve(&mut self, addr: Page<Phys>, size: NbrPages) -> Result<()> {
        self.allocator.reserve_exact(addr, size)
    }

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
    eprintln!("kernel physical end: {:x?}", symbol_addr!(kernel_physical_end));
    eprintln!("kernel physical end alligned: {:x?}", Phys(symbol_addr!(kernel_physical_end)).align_next(PAGE_SIZE));

    let mut pallocator = PhysicalPageAllocator::new(Page::new(0), system_memory_amount);

    // Calculate how much non-existent memory the physical buddy allocator thinks it actually has.
    let reserve_order: Order = system_memory_amount.into();
    let overflowing_memory_amount: NbrPages = NbrPages::from(reserve_order) - system_memory_amount;
    let high_mem_limit: Page<Phys> = Page::new(0) + system_memory_amount;

    // Reserve the non-existent memory so that the buddy allocator cannot ever give it to anyone (except if this is freed at some point...).
    pallocator.reserve(high_mem_limit, overflowing_memory_amount).unwrap();

    pallocator.reserve(Page::new(0), (symbol_addr!(kernel_physical_end) - 0).into()).unwrap();
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
