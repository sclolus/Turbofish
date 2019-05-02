//! This file describe all the superblock model

use super::Block;

use bitflags::bitflags;

use core::fmt;

/// Common structure of a SuperBlock
#[derive(Debug, Copy, Clone)]
#[repr(packed)]
pub struct SuperBlock {
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
    block_containing_superblock: Block,
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
    file_system_state: FileSystemState,
    /// What to do when an error is detected (see below)
    /*60 	61 	2*/
    error_handling_methods: ErrorHandlingMethods,
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
    creator_operating_system: CreatorOperatingSystem,
    /// Major portion of version (combine with Minor portion above to construct full version field)
    /*76 	79 	4*/
    major_version: u32,
    /// User ID that can use reserved blocks
    /*80 	81 	2*/
    user_id_reserved_blocks: u16,
    /// Group ID that can use reserved blocks
    /*82 	83 	2*/
    group_id_reserved_blocks: u16,
    /// First non-reserved inode in file system. (In versions < 1.0, this is fixed as 11)
    /*84   87   4 */
    first_non_reserved_inode: u32,
    /// Size of each inode structure in bytes. (In versions < 1.0, this is fixed as 128)
    /*88   89   2 */
    size_inode: u16,
    /// Block group that this superblock is part of (if backup copy)
    /*90   91   2 */
    block_group_of_superblock: u16,
    /// Optional features present (features that are not required to read or write, but usually result in a performance increase. see below)
    /*92   95   4 */
    optional_features_flag: OptionalFeaturesFlag,
    /// Required features present (features that are required to be supported to read or write. see below)
    /*96   99   4 */
    required_features_flag: RequiredFeaturesFlag,
    /// Features that if not supported, the volume must be mounted read-only see below)
    /*100  103  4 */
    feature_must_read_only: ReadOnlyFeaturesFlag,
    /// File system ID (what is output by blkid)
    /*104  119  16*/
    file_system_id: u16,
    /// Volume name (C-style string: characters terminated by a 0 byte)
    /*120  135  16*/
    volume_name: u16,
    /// Path volume was last mounted to (C-style string: characters terminated by a 0 byte)
    /*136  199  64*/
    path_volume_last_mounted: PathVolumeLastMounted,
    /// Compression algorithms used (see Required features above)
    /*200  203  4 */
    compression_algorithms_used: u32,
    /// Number of blocks to preallocate for files
    /*204  204  1 */
    number_of_blocks_to_preallocate_for_files: u8,
    /// Number of blocks to preallocate for directories
    /*205  205  1 */
    number_of_blocks_to_preallocate_for_directories: u8,
    /// (Unused)
    /*206  207  2 */
    unused: u16,
    /// Journal ID (same style as the File system ID above)
    /*208  223  16*/
    journal_id: u16,
    /// Journal inode
    /*224  227  4 */
    journal_inode: u32,
    /// Journal device
    /*228  231  4 */
    journal_device: u32,
    /// Head of orphan inode list
    /*232  235  4 */
    head_of_orphan_inode_list: u32,
}

impl SuperBlock {
    /// Get ext2 signature
    pub fn get_ext2_signature(&self) -> u16 {
        self.ext2_signature
    }

    /// Get the number of block per block group
    pub fn get_nbr_block_grp(&self) -> u32 {
        div_rounded_up(self.nbr_blocks, self.block_per_block_grp)
    }

    /// Get the number of inode per block group
    pub fn get_inode_block_grp(&self) -> u32 {
        div_rounded_up(self.nbr_inode, self.inodes_per_block_grp)
    }

    /// Get the superblock official block per block group
    pub fn get_block_per_block_grp(&self) -> u32 {
        self.block_per_block_grp
    }

    /// Get the superblock official inode per block group
    pub fn get_inode_per_block_grp(&self) -> u32 {
        self.inodes_per_block_grp
    }

    /// Get the log2 (block size) - 10. (In other words, the number to shift 1,024 to the left by to obtain the block size)
    pub fn get_log2_block_size(&self) -> u32 {
        self.log2_block_size
    }

    /// Get the size of each inode structure in bytes. (In versions < 1.0, this is fixed as 128)
    pub fn get_size_inode(&self) -> u16 {
        self.size_inode
    }
}

/// SuperBlock contains the file System state
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
#[repr(u16)]
enum FileSystemState {
    Unknown = 0,
    IsClean = 1,
    HasErrors = 2,
}

/// SuperBlock contains the action ti take if some errors were found in the filesystem
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
#[repr(u16)]
enum ErrorHandlingMethods {
    IgnoreTheError = 1,
    RemountFileSystemAsReadOnly = 2,
    KernelPanic = 3,
}

/// Superblock contains a indication about witch OS create the filesystem
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
#[repr(u32)]
enum CreatorOperatingSystem {
    /// I never used GNU-LINUX
    Linux = 0,
    HURD,
    /// (an operating system developed by Rémy Card, one of the developers of ext2)
    MASIX,
    FreeBSD,
    /// "Lites" (BSD4.4-Lite derivatives such as NetBSD, OpenBSD, XNU/Darwin, etc.)
    Other,
}

// These features if present on a file system are required to be supported by an implementation
// in order to correctly read from or write to the file system.
bitflags! {
    struct RequiredFeaturesFlag: u32 {
        const COMPRESSION_IS_USED = 0x1;
        const DIRECTORY_ENTRIES_CONTAIN_A_TYPE_FIELD = 0x2;
        const FILE_SYSTEM_NEEDS_TO_REPLAY_ITS_JOURNAL = 0x4;
        const FILE_SYSTEM_USES_A_JOURNAL_DEVICE = 0x8;
    }
}

// These are optional features for an implementation to support, but offer performance or
// reliability gains to implementations that do support them.
bitflags! {
    struct OptionalFeaturesFlag: u32 {
        const PREALLOCATE_SOME_NUMBER_OF_BLOCKS_A_DIRECTORY_WHEN_CREATING_A_NEW_ONE = 0x0001;
        const AFS_SERVER_INODES_EXIST = 0x0002;
        const FILE_SYSTEM_HAS_A_JOURNAL = 0x0004;
        const INODES_HAVE_EXTENDED_ATTRIBUTES = 0x0008;
        const FILE_SYSTEM_CAN_RESIZE_ITSELF_FOR_LARGER_PARTITIONS = 0x0010;
        const DIRECTORIES_USE_HASH_INDEX = 0x0020;
    }
}

// These features, if present on a file system, are required in order for an implementation
// to write to the file system, but are not required to read from the file system.
bitflags! {
    struct ReadOnlyFeaturesFlag: u32 {
        const SPARSE_SUPERBLOCKS_AND_GROUP_DESCRIPTOR_TABLES = 0x1;
        const FILE_SYSTEM_USES_A_64_BIT_FILE_SIZE = 0x2;
        const DIRECTORY_CONTENTS_ARE_STORED_IN_THE_FORM_OF_A_BINARY_TREE = 0x3;
    }
}

/// Indication about the last mount moment
#[derive(Copy, Clone)]
#[repr(transparent)]
struct PathVolumeLastMounted([u8; 64]);

/// Debug boilerplate for PathVolumeLastMounted
impl fmt::Debug for PathVolumeLastMounted {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", "PathVolumeLastMounted")
    }
}

/// Roundup style function
fn div_rounded_up(a: u32, b: u32) -> u32 {
    (a + b - 1) / b
}
