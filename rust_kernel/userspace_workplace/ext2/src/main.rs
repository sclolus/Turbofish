use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
struct BlockNumber(u32);

const EXT2_SIGNATURE_MAGIC: u16 = 0xef53;

#[derive(Debug, Copy, Clone)]
#[repr(packed)]
struct BaseSuperBlock {
    /// Total number of inodes in file system
    /*0 	3 	4*/
    nbr_inode: u32,
    /// Total number of blocks in file system
    /*4 	7 	4*/
    nbr_blocks: u32,
    /// Number of blocks reserved for superuser (see offset 80)
    /*8 	11 	4*/
    nbr_blocks_reserved: u32,
    /// Total number of unallocated blocks
    /*12 	15 	4*/
    nbr_unallocated_blocks: u32,
    /// Total number of unallocated inodes
    /*16 	19 	4*/
    nbr_unallocated_inodes: u32,
    /// Block number of the block containing the superblock
    /*20 	23 	4*/
    block_containing_superblock: BlockNumber,
    /// log2 (block size) - 10. (In other words, the number to shift 1,024 to the left by to obtain the block size)
    /*24 	27 	4*/
    log2_block_size: u32,
    /// log2 (fragment size) - 10. (In other words, the number to shift 1,024 to the left by to obtain the fragment size)
    /*28 	31 	4*/
    log2_fragment_size: u32,
    /// Number of blocks in each block group
    /*32 	35 	4*/
    block_per_block_grp: u32,
    /// Number of fragments in each block group
    /*36 	39 	4*/
    fragment_per_block_grp: u32,
    /// Number of inodes in each block group
    /*40 	43 	4*/
    inodes_per_block_grp: u32,
    /// Last mount time (in POSIX time)
    /*44 	47 	4*/
    last_mount_time: u32,
    /// Last written time (in POSIX time)
    /*48 	51 	4*/
    last_written_time: u32,
    /// Number of times the volume has been mounted since its last consistency check (fsck)
    /*52 	53 	2*/
    nbr_of_mount_since_last_consistency_check: u16,
    /// Number of mounts allowed before a consistency check (fsck) must be done
    /*54 	55 	2*/
    nbr_of_mounts_allowed_before_conistency_check: u16,
    /// Ext2 signature (0xef53), used to help confirm the presence of Ext2 on a volume
    /*56 	57 	2*/
    ext2_signature: u16,
    /// File system state (see below)
    /*58 	59 	2*/
    file_system_state: u16,
    /// What to do when an error is detected (see below)
    /*60 	61 	2*/
    error_case: u16,
    /// Minor portion of version (combine with Major portion below to construct full version field)
    /*62 	63 	2*/
    minor_version: u16,
    /// POSIX time of last consistency check (fsck)
    /*64 	67 	4*/
    last_consistency_check: u32,
    /// Interval (in POSIX time) between forced consistency checks (fsck)
    /*68 	71 	4*/
    interval_between_forced_consistency_checks: u32,
    /// Operating system ID from which the filesystem on this volume was created (see below)
    /*72 	75 	4*/
    operating_system_id: u32,
    /// Major portion of version (combine with Minor portion above to construct full version field)
    /*76 	79 	4*/
    major_version: u32,
    /// User ID that can use reserved blocks
    /*80 	81 	2*/
    user_id_reserved_blocks: u16,
    /// Group ID that can use reserved blocks
    /*82 	83 	2*/
    group_id_reserved_blocks: u16,
}

fn main() {
    let mut f = File::open("simple_diskp1").unwrap();
    let mut buf = [0; 4096];

    f.seek(SeekFrom::Start(1024)).unwrap();
    f.read(&mut buf[0..core::mem::size_of::<BaseSuperBlock>()]).unwrap();
    let base_superblock: BaseSuperBlock = unsafe { core::mem::transmute_copy(&buf) };
    println!("{:#x?}", base_superblock);

    assert_eq!(base_superblock.ext2_signature, EXT2_SIGNATURE_MAGIC);
}
