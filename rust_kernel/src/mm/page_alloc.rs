use super::buddy_allocator::BuddyAllocator;
use super::dummy_allocator::DummyAllocator;
use super::MemoryError;
use super::PAGE_DIRECTORY;
use super::PAGE_SIZE;
use super::PAGE_TABLES;
use core::fmt::Debug;
use core::mem;
use core::ops::{Index, IndexMut, Range};

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
    // zoned_physical_buddy_allocators: &'b mut [(BuddyAllocator<'a>, PhysicalAllocatorType)],
    // zoned_virtual_buddy_allocators: &'b mut [(BuddyAllocator<'a>, VirtualAllocatorType)],
    zoned_physical_buddy_allocators: &'b mut [(DummyAllocator<'a>, PhysicalAllocatorType)],
    zoned_virtual_buddy_allocators: &'b mut [(DummyAllocator<'a>, VirtualAllocatorType)],
}

impl<'a, 'b: 'a> PageAllocator<'a, 'b> {
    pub fn new(
        // zoned_physical_buddy_allocators: &'b mut [(BuddyAllocator<'a>, PhysicalAllocatorType)],
        // zoned_virtual_buddy_allocators: &'b mut [(BuddyAllocator<'a>, VirtualAllocatorType)],
        zoned_physical_buddy_allocators: &'b mut [(DummyAllocator<'a>, PhysicalAllocatorType)],
        zoned_virtual_buddy_allocators: &'b mut [(DummyAllocator<'a>, VirtualAllocatorType)],
    ) -> Self {
        Self { zoned_physical_buddy_allocators, zoned_virtual_buddy_allocators }
    }
    fn select_buddy_allocators(
        &mut self,
        flags: AllocFlags,
    ) -> Option<(&mut DummyAllocator<'a>, &mut DummyAllocator<'a>)> {
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
            .find(|(buddy, btype)| *btype == requested_pbuddy_type)
            .map(|(buddy, _)| buddy)?;

        if flags.0 & PAGE_USER != 0 {
            requested_vbuddy_type = UserSpace;
        } else {
            requested_vbuddy_type = KernelSpace;
        }
        vbuddy = self
            .zoned_virtual_buddy_allocators
            .iter_mut()
            .find(|(buddy, btype)| *btype == requested_vbuddy_type)
            .map(|(buddy, _)| buddy)?;
        Some((vbuddy, pbuddy))
    }

    pub fn reserve(
        &mut self,
        vaddr: usize,
        paddr: usize,
        nbr_pages: usize,
        flags: AllocFlags,
    ) -> Result<(), MemoryError> {
        let (vbuddy, pbuddy) = self.select_buddy_allocators(flags).ok_or(MemoryError::NotSatifiableFlags)?;
        // vbuddy.reserve(vaddr, nbr_pages)?;

        match vbuddy.reserve(vaddr, nbr_pages) {
            Ok(_) => (),
            Err(e) => {
                println!("Virtual Allocator failed to reserve {:x}", vaddr);
                return Err(e);
            }
        }

        pbuddy.reserve(paddr, nbr_pages).or_else(|e| {
            // WARNING: if buddy segmente it wont work
            println!("Physical allocator failed to reserve {:x}", paddr);
            vbuddy.free(vaddr, nbr_pages);
            Err(e)
        })?;

        unsafe {
            PAGE_DIRECTORY.remap_range_addr(vaddr, paddr, nbr_pages).or_else(|err| {
                println!("Failed to remap addr range: [{:x}:{:x}[", vaddr, vaddr + PAGE_SIZE * nbr_pages);
                vbuddy.free(vaddr, nbr_pages);
                pbuddy.free(vaddr, nbr_pages);
                Err(MemoryError::AlreadyMapped)
            })
        }
    }

    pub fn alloc(&mut self, nbr_pages: usize, flags: AllocFlags) -> Option<usize> {
        let (vbuddy, pbuddy) = self.select_buddy_allocators(flags)?;
        let vaddr = vbuddy.alloc(nbr_pages)?;
        let paddr = pbuddy.alloc(nbr_pages).or_else(|| {
            vbuddy.free(vaddr, nbr_pages);
            None
        })?;

        unsafe {
            PAGE_DIRECTORY
                .remap_range_addr(vaddr, paddr, nbr_pages)
                .or_else(|err| {
                    vbuddy.free(vaddr, nbr_pages);
                    pbuddy.free(vaddr, nbr_pages);
                    Err(err)
                })
                .ok()?;
        }

        Some(vaddr)
    }

    pub fn free(&mut self, addr: usize, nbr_pages: usize) {}
}
