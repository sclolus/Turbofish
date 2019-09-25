//! This crate provide methods to read Master Boot record
#![cfg_attr(not(test), no_std)]

use core::mem;
use raw_data::define_raw_data;

/// Main crate structure
#[derive(Debug, Copy, Clone)]
pub struct Mbr {
    physical_mbr: PhysicalMbr,
    bootable: bool,
    pub parts: [Partition; 4],
}

#[derive(Debug, Copy, Clone)]
pub struct Partition {
    /// Drive attributes (bit 7 set = active or bootable)
    drive_attribute: u8,
    part_type: PartitionType,
    pub start: u32,
    pub size: u32,
}

impl Partition {
    /// return if the partition is active
    pub fn is_active(&self) -> bool {
        // is that correct ?
        (self.drive_attribute & (1 << 7)) != 0
    }
}

#[derive(Debug, Copy, Clone)]
enum PartitionType {
    LinuxExtendedPartition,
    Dos12bitsFat,
    Empty,
    Unknown,
}

/// Defined real MBR Structure
#[derive(Debug, Copy, Clone)]
#[repr(packed)]
struct PhysicalMbr {
    /// MBR Bootstrap (flat binary executable code)
    /*0    */
    mbr_bootstrap: FlatBinaryExecutableCode,
    /// Optional "Unique Disk ID / Signature"
    /*1B8  */
    unique_disk_id: u32,
    /// Optional, reserved 0x0000, The 2 byte reserved is usually 0x0000. 0x5A5A means read-only
    /*1BC  */
    reserved: u16,
    /// First partition table entry
    /*1BE  */
    partitions: [PhysicalPartitionField; 4],
    /// (0x55, 0xAA) "Valid bootsector" signature bytes
    /*1FE  */
    magic: u16,
    /*200  */
}

define_raw_data!(FlatBinaryExecutableCode, 440);

/// Defined real Partition structure
#[derive(Debug, Copy, Clone)]
#[repr(packed)]
struct PhysicalPartitionField {
    /// Drive attributes (bit 7 set = active or bootable)
    /*0    */
    drive_attribute: u8,
    /// CHS Address of partition start
    /*1    */
    start_chs_address: [u8; 3],
    /// Partition type
    /*4    */
    partition_type: u8,
    /// CHS address of last partition sector
    /*5    */
    end_chs_address: [u8; 3],
    /// LBA of partition start
    /*8    */
    lba_start: u32,
    /// Number of sectors in partition
    /*C    */
    nb_sector: u32,
    /*10   */
}

/// MbrResult is just made to handle module errors
pub type MbrResult<T> = core::result::Result<T, MbrError>;

/// Common errors for this module
#[derive(Debug, Copy, Clone)]
pub enum MbrError {
    /// Not a valid MBR structure
    UnknownStructure,
}

impl Mbr {
    /// Create a new MBR object
    pub unsafe fn new(data: &[u8; 512]) -> Self {
        let physical_mbr: PhysicalMbr = mem::transmute_copy(data);
        let bootable = if physical_mbr.magic == 0xAA55 { true } else { false };
        let mut parts: [Partition; 4] = core::mem::uninitialized();
        for (i, elem) in parts.iter_mut().enumerate() {
            *elem = Partition {
                part_type: PartitionType::from(physical_mbr.partitions[i].partition_type),
                start: physical_mbr.partitions[i].lba_start,
                size: physical_mbr.partitions[i].nb_sector,
                drive_attribute: physical_mbr.partitions[i].drive_attribute,
            };
        }
        Self { physical_mbr, bootable, parts }
    }
}

impl From<u8> for PartitionType {
    fn from(part_number: u8) -> PartitionType {
        use PartitionType::*;
        match part_number {
            0x83 => LinuxExtendedPartition,
            0x01 => Dos12bitsFat,
            0x00 => Empty,
            _ => Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
