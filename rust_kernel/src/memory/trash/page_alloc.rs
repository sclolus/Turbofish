use super::buddy_allocator::BuddyAllocator;
// use super::dummy_allocator::DummyAllocator;
use super::MemoryError;
use super::PAGE_DIRECTORY;
use super::PAGE_SIZE;
use super::{PhysicalAddr, VirtualAddr};
#[allow(unused_imports)]
use core::convert::{From, Into};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PhysicalAllocatorType {
    Normal,
    Dma,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum VirtualAllocatorType {
    KernelSpace,
    UserSpace,
}

pub const PAGE_CONST: u8 = 0x2;
pub const PAGE_CACHABLE_DISABLED: u8 = 0x4;
pub const PAGE_USER: u8 = 0x8;
pub const MAY_SLEEP: u8 = 0x10;
pub const NO_RETRY: u8 = 0x40;
pub const PAGE_DMA: u8 = 0x80;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct AllocFlags(pub u8);

pub const ALLOC_DMA: u8 = PAGE_CACHABLE_DISABLED | PAGE_DMA;
pub const ALLOC_ATOMIC: u8 = NO_RETRY;
pub const ALLOC_NORMAL: u8 = MAY_SLEEP;

pub struct PageAllocator<'a, 'b: 'a> {
    //different lifetimes for each type of buddy allocators ?
    zoned_physical_buddy_allocators: &'b mut [(BuddyAllocator<'a, PhysicalAddr>, PhysicalAllocatorType)],
    zoned_virtual_buddy_allocators: &'b mut [(BuddyAllocator<'a, VirtualAddr>, VirtualAllocatorType)],
    // zoned_physical_buddy_allocators: &'b mut [(DummyAllocator<'a>, PhysicalAllocatorType)],
    // zoned_virtual_buddy_allocators: &'b mut [(DummyAllocator<'a>, VirtualAllocatorType)],
}

impl<'a, 'b: 'a> PageAllocator<'a, 'b> {
    pub fn new(
        zoned_physical_buddy_allocators: &'b mut [(BuddyAllocator<'a, PhysicalAddr>, PhysicalAllocatorType)],
        zoned_virtual_buddy_allocators: &'b mut [(BuddyAllocator<'a, VirtualAddr>, VirtualAllocatorType)],
        // zoned_physical_buddy_allocators: &'b mut [(DummyAllocator<'a>, PhysicalAllocatorType)],
        // zoned_virtual_buddy_allocators: &'b mut [(DummyAllocator<'a>, VirtualAllocatorType)],
    ) -> Self {
        Self { zoned_physical_buddy_allocators, zoned_virtual_buddy_allocators }
    }
    fn select_buddy_allocators(
        &mut self,
        flags: AllocFlags,
    ) -> Option<(&mut BuddyAllocator<'a, VirtualAddr>, &mut BuddyAllocator<'a, PhysicalAddr>)> {
        use PhysicalAllocatorType::*;
        use VirtualAllocatorType::*;
        let pbuddy;
        let vbuddy;
        let requested_pbuddy_type;
        let requested_vbuddy_type;

        if flags.0 & PAGE_DMA != 0 {
            requested_pbuddy_type = Dma;
        } else {
            requested_pbuddy_type = Normal;
        }
        pbuddy = self
            .zoned_physical_buddy_allocators
            .iter_mut()
            .find(|(_, btype)| *btype == requested_pbuddy_type)
            .map(|(buddy, _)| buddy)?;

        if flags.0 & PAGE_USER != 0 {
            requested_vbuddy_type = UserSpace;
        } else {
            requested_vbuddy_type = KernelSpace;
        }
        vbuddy = self
            .zoned_virtual_buddy_allocators
            .iter_mut()
            .find(|(_, btype)| *btype == requested_vbuddy_type)
            .map(|(buddy, _)| buddy)?;
        Some((vbuddy, pbuddy))
    }

    pub fn reserve(
        &mut self,
        vaddr: VirtualAddr,
        paddr: PhysicalAddr,
        nbr_pages: usize,
        flags: AllocFlags,
    ) -> Result<(), MemoryError> {
        let (vbuddy, pbuddy) = self.select_buddy_allocators(flags).ok_or(MemoryError::NotSatifiableFlags)?;

        assert!(vaddr.0 % 4096 == 0);
        assert!(paddr.0 % 4096 == 0);
        match vbuddy.reserve(vaddr, nbr_pages.into()) {
            Ok(_) => (),
            Err(e) => {
                println!("Virtual Allocator failed to reserve {:x?}", vaddr);
                return Err(e);
            }
        }

        pbuddy.reserve(paddr, nbr_pages.into()).or_else(|e| {
            // WARNING: if buddy segmente it wont work
            println!("Physical allocator failed to reserve {:x?}", paddr);
            vbuddy.free(vaddr, nbr_pages.into());
            Err(e)
        })?;

        unsafe {
            PAGE_DIRECTORY.remap_range_addr(vaddr, paddr, nbr_pages).or_else(|_err| {
                println!("Failed to remap addr range: [{:x?}:{:x?}[", vaddr, vaddr.0 + PAGE_SIZE * nbr_pages);
                vbuddy.free(vaddr, nbr_pages.into());
                pbuddy.free(paddr, nbr_pages.into());
                Err(MemoryError::AlreadyMapped)
            })
        }
    }

    pub fn alloc(&mut self, nbr_pages: usize, flags: AllocFlags) -> Option<VirtualAddr> {
        let (vbuddy, pbuddy) = self.select_buddy_allocators(flags)?;
        let vaddr = vbuddy.alloc(nbr_pages.into())?;
        let paddr = pbuddy.alloc(nbr_pages.into()).or_else(|| {
            vbuddy.free(vaddr, nbr_pages.into());
            None
        })?;

        unsafe {
            PAGE_DIRECTORY
                .remap_range_addr(vaddr, paddr, nbr_pages)
                .or_else(|err| {
                    vbuddy.free(vaddr, nbr_pages.into());
                    pbuddy.free(paddr, nbr_pages.into());
                    Err(err)
                })
                .ok()?;
        }

        Some(vaddr)
    }

    pub fn free(&mut self, _addr: usize, _nbr_pages: usize) {}
}
