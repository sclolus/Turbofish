use crate::memory::mmu::Entry;
use bitflags::bitflags;

bitflags! {
    pub struct AllocFlags: u32 {
        const KERNEL_MEMORY = 1 << 0;
        const USER_MEMORY = 1 << 1;
        const READ_ONLY = 1 << 2;
        const CACHE_DISABLE = 1 << 3;
        const DMA = 1 << 4;
    }
}

impl From<AllocFlags> for Entry {
    fn from(flags: AllocFlags) -> Entry {
        let mut entry = Entry::default();

        if flags.contains(AllocFlags::USER_MEMORY) {
            entry |= Entry::USER;
        }

        if !flags.contains(AllocFlags::READ_ONLY) {
            entry |= Entry::READ_WRITE;
        }

        if flags.contains(AllocFlags::CACHE_DISABLE) {
            entry |= Entry::CACHE_DISABLE;
        }

        if flags.contains(AllocFlags::DMA) {
            entry |= Entry::CACHE_DISABLE;
            entry |= Entry::WRITE_THROUGH;
        }

        entry
    }
}
