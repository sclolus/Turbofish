use super::BuddyAllocator;
use super::KERNEL_PHYSICAL_MEMORY;
use crate::memory::tools::*;
use alloc::vec;
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
        Self { allocator: BuddyAllocator::new(phys_start, size, vec![0; BuddyAllocator::<Phys>::metadata_size(size)]) }
    }
    /// size in bytes
    pub fn alloc(&mut self, size: NbrPages, flags: AllocFlags) -> Result<Page<Phys>> {
        if flags.contains(AllocFlags::KERNEL_MEMORY) {
            let order = size.into();
            let res = self.allocator.alloc(order)?;
            // let addr: Phys = res.into();
            // eprintln!("physical alloc: {:x?}", addr);
            Ok(res)
        } else {
            unimplemented!()
        }
    }

    pub fn reserve(&mut self, addr: Page<Phys>, size: NbrPages) -> Result<()> {
        self.allocator.reserve_exact(addr, size)
    }
    /// size in bytes
    pub fn free(&mut self, paddr: Page<Phys>, size: NbrPages) -> Result<()> {
        let order = size.into();
        Ok(self.allocator.free(paddr, order)?)
    }
}

pub static mut PHYSICAL_ALLOCATOR: Option<PhysicalPageAllocator> = None;

pub unsafe fn init_physical_allocator(_device_map_ptr: *const DeviceMap) {
    eprintln!("kernel physical end: {:x?}", symbol_addr!(kernel_physical_end));
    eprintln!("kernel physical end alligned: {:x?}", Phys(symbol_addr!(kernel_physical_end)).align_on(PAGE_SIZE));
    let pallocator = PhysicalPageAllocator::new(
        Phys(symbol_addr!(kernel_physical_end)).align_on(PAGE_SIZE).into(),
        KERNEL_PHYSICAL_MEMORY,
    );

    // let device_map_len = {
    //     let mut i: usize = 0;
    //     use core::mem::size_of;
    //     loop {
    //         if *(device_map_ptr.offset(i as isize) as *const [u8; size_of::<DeviceMap>()])
    //             == [0; size_of::<DeviceMap>()]
    //         {
    //             break i;
    //         }
    //         i += 1;
    //     }
    // };
    //DOESNT WORK I DONT KNOW WHY
    // pallocator.reserve(Phys(0), NbrPages::_1MB.into()).unwrap();
    // pallocator
    //     .reserve(
    //         Phys(symbol_addr!(kernel_physical_start)),
    //         symbol_addr!(kernel_physical_end) - symbol_addr!(kernel_physical_start),
    //     )
    //     .unwrap();
    // let device_map_slice = core::slice::from_raw_parts(device_map_ptr, device_map_len);
    // for d in device_map_slice {
    //     println!("{:x?}", d);
    //     println!("addr: {:x?}", d.low_addr);
    //     println!("len: {}ko", d.low_length >> 10);
    //     match d.region_type {
    //         RegionType::Usable => {}
    //         _ => {
    //             //TODO: see that
    //             pallocator.reserve(Page::containing(Phys(d.low_addr as usize)), (d.low_length as usize).into());
    //         }
    //     }
    // }
    PHYSICAL_ALLOCATOR = Some(pallocator);
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RegionType {
    /// (normal) RAM
    Usable = 1,
    /// unusable
    Reserved = 2,
    /// reclaimable memory
    ACPI = 3,
    ACPINVS = 4,
    ///    containing bad memory
    Area = 5,
}

/// Show how devices are mapped in physical memory and also available space
/// For reading all structures map, just run away with offset 32 until a zeroed structure
#[derive(Copy, Clone, Debug)]
#[repr(C)]
#[repr(align(32))]
pub struct DeviceMap {
    /// addr in the first 4GB
    pub low_addr: u32,
    /// used only in 64 bit
    pub high_addr: u32,
    pub low_length: u32,
    pub high_length: u32,
    pub region_type: RegionType,
    pub acpi_reserved: u32,
}
