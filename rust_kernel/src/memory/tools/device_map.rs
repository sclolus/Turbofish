/// This file contains the code for the device map obtained from GRUB.
/// It is an array of entries that describe zones across the whole RAM-space.
use core::mem;

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

// Do not have any lifetime bound to make so I'll just make it static
pub unsafe fn get_device_map_slice(device_map_ptr: *const DeviceMap) -> &'static [DeviceMap] {
    let device_map_len = {
        let mut i = 0;
        let mut ptr: *const [u8; mem::size_of::<DeviceMap>()] = device_map_ptr as *const _;

        while *ptr != [0; mem::size_of::<DeviceMap>()] {
            ptr = ptr.add(1);
            i += 1;
        }
        i
    };
    core::slice::from_raw_parts(device_map_ptr, device_map_len)
}
