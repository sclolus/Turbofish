use bitflags::bitflags;

bitflags! {
    pub struct AllocFlags: u32 {
        const KERNEL_MEMORY = 1 << 0;
        const USER_MEMORY = 1 << 1;
    }
}
