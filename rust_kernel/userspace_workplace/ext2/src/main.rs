//! this module contains a ext2 driver
//! see [osdev](https://wiki.osdev.org/Ext2)
use bit_field::BitArray;
use bitflags::bitflags;
use core::cmp::min;
use core::mem::size_of;
use core::ops::Add;
use std::fs::File as StdFile;
use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(transparent)]
struct BlockNumber(u32);

impl Add<Self> for BlockNumber {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

const EXT2_SIGNATURE_MAGIC: u16 = 0xef53;

#[derive(Debug, Copy, Clone)]
#[repr(packed)]
struct SuperBlock {
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

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
#[repr(u16)]
enum FileSystemState {
    Unknown = 0,
    IsClean = 1,
    HasErrors = 2,
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
#[repr(u16)]
enum ErrorHandlingMethods {
    IgnoreTheError = 1,
    RemountFileSystemAsReadOnly = 2,
    KernelPanic = 3,
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
#[repr(u32)]
enum CreatorOperatingSystem {
    Linux = 0,
    HURD,
    /// (an operating system developed by Rémy Card, one of the developers of ext2)
    MASIX,
    FreeBSD,
    /// "Lites" (BSD4.4-Lite derivatives such as NetBSD, OpenBSD, XNU/Darwin, etc.)
    Other,
}

bitflags! {
    struct RequiredFeaturesFlag: u32 {
        const COMPRESSION_IS_USED = 0x1;
        const DIRECTORY_ENTRIES_CONTAIN_A_TYPE_FIELD = 0x2;
        const FILE_SYSTEM_NEEDS_TO_REPLAY_ITS_JOURNAL = 0x4;
        const FILE_SYSTEM_USES_A_JOURNAL_DEVICE = 0x8;
    }
}

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

bitflags! {
    struct ReadOnlyFeaturesFlag: u32 {
        const SPARSE_SUPERBLOCKS_AND_GROUP_DESCRIPTOR_TABLES = 0x1;
        const FILE_SYSTEM_USES_A_64_BIT_FILE_SIZE = 0x2;
        const DIRECTORY_CONTENTS_ARE_STORED_IN_THE_FORM_OF_A_BINARY_TREE = 0x3;
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
struct PathVolumeLastMounted([u8; 64]);

use core::fmt;

impl fmt::Debug for PathVolumeLastMounted {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", "PathVolumeLastMounted")
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(packed)]
struct BlockGroupDescriptor {
    /// Block address of block usage bitmap
    /*0 	3 	4*/
    block_usage_bitmap: BlockNumber,
    /// Block address of inode usage bitmap
    /*4 	7 	4*/
    inode_usage_bitmap: BlockNumber,
    /// Starting block address of inode table
    /*8 	11 	4*/
    inode_table: BlockNumber,
    /// Number of unallocated blocks in group
    /*12 	13 	2*/
    nbr_unallocated_blocks: u16,
    /// Number of unallocated inodes in group
    /*14 	15 	2*/
    nbr_unallocated_inodes: u16,
    /// Number of directories in group
    /*16 	17 	2*/
    nbr_directories: u16,
}

#[derive(Debug, Copy, Clone)]
#[repr(packed)]
struct Inode {
    /// Type and Permissions (see below)
    /*0 	1       2*/
    type_and_perm: TypeAndPerm,
    /// User ID
    /*2 	3       2*/
    user_id: u16,
    /// Lower 32 bits of size in bytes
    /*4 	7       4*/
    low_size: u32,
    /// Last Access Time (in POSIX time)
    /*8 	11      4*/
    last_access_time: u32,
    /// Creation Time (in POSIX time)
    /*12 	15      4*/
    creation_time: u32,
    /// Last Modification time (in POSIX time)
    /*16 	19      4*/
    last_modification_time: u32,
    /// Deletion time (in POSIX time)
    /*20 	23      4*/
    deletion_time: u32,
    /// Group ID
    /*24 	25      2*/
    group_id: u16,
    /// Count of hard links (directory entries) to this inode. When this reaches 0, the data blocks are marked as unallocated.
    /*26 	27      2*/
    nbr_hard_links: u16,
    /// Count of disk sectors (not Ext2 blocks) in use by this inode, not counting the actual inode structure nor directory entries linking to the inode.
    /*28 	31      4*/
    nbr_disk_sectors: u32,
    /// Flags (see below)
    /*32 	35      4*/
    flags: InodeFlags,
    /// Operating System Specific value #1
    /*36 	39      4*/
    operating_system_specific_value_1: u32,
    /// Direct Block Pointers
    /*40 	43      4*/
    direct_block_pointers: [BlockNumber; 12],
    /// Singly Indirect Block Pointer (Points to a block that is a list of block pointers to data)
    /*88 	91      4*/
    singly_indirect_block_pointers: BlockNumber,
    /// Doubly Indirect Block Pointer (Points to a block that is a list of block pointers to Singly Indirect Blocks)
    /*92 	95      4*/
    doubly_indirect_block_pointers: BlockNumber,
    /// Triply Indirect Block Pointer (Points to a block that is a list of block pointers to Doubly Indirect Blocks)
    /*96 	99      4*/
    triply_indirect_block_pointers: BlockNumber,
    /// Generation number (Primarily used for NFS)
    /*100 	103 	4*/
    generation_number: u32,
    /// In Ext2 version 0, this field is reserved. In version >= 1, Extended attribute block (File ACL).
    /*104 	107 	4*/
    extended_attribute_block: u32,
    /// In Ext2 version 0, this field is reserved. In version >= 1, Upper 32 bits of file size (if feature bit set) if it's a file, Directory ACL if it's a directory
    /*108 	111 	4*/
    upper_size: u32,
    /// Block address of fragment
    /*112 	115 	4*/
    fragment_addr: BlockNumber,
    /// Operating System Specific Value #2
    /*116 	127 	12*/
    operating_system_specific_value_2: u32,
}

bitflags! {
    struct TypeAndPerm: u16 {
        const FIFO = 0x1000;
        const CHARACTER_DEVICE = 0x2000;
        const DIRECTORY = 0x4000;
        const BLOCK_DEVICE = 0x6000;
        const REGULAR_FILE = 0x8000;
        const SYMBOLIC_LINK = 0xA000;
        const UNIX_SOCKET = 0xC000;
        const OTHER_EXECUTE_PERMISSION = 0o0001;
        const OTHER_WRITE_PERMISSION = 0o0002;
        const OTHER_READ_PERMISSION = 0o0004;
        const GROUP_EXECUTE_PERMISSION = 0o0010;
        const GROUP_WRITE_PERMISSION = 0o0020;
        const GROUP_READ_PERMISSION = 0o0040;
        const USER_EXECUTE_PERMISSION = 0o0100;
        const USER_WRITE_PERMISSION = 0o0200;
        const USER_READ_PERMISSION = 0o0400;
        const STICKY_BIT = 0o1000;
        const SET_GROUP_ID = 0o2000;
        const SET_USER_ID = 0o4000;
    }
}

bitflags! {
    struct InodeFlags: u32 {
        const SECURE_DELETION = 0x00000001;
        const KEEP_A_COPY_OF_DATA_WHEN_DELETED = 0x00000002;
        const FILE_COMPRESSION = 0x00000004;
        const SYNCHRONOUS_UPDATES_NEW_DATA_IS_WRITTEN_IMMEDIATELY_TO_DISK = 0x00000008;
        const IMMUTABLE_FILE = 0x00000010;
        const APPEND_ONLY = 0x00000020;
        const FILE_IS_NOT_INCLUDED_IN_DUMP_COMMAND = 0x00000040;
        const LAST_ACCESSED_TIME_SHOULD_NOT_UPDATED = 0x00000080;
        const HASH_INDEXED_DIRECTORY = 0x00010000;
        const AFS_DIRECTORY = 0x00020000;
        const JOURNAL_FILE_DATA = 0x00040000;
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(packed)]
struct DirectoryEntryHeader {
    /// Inode
    /*0 	3 	4*/
    inode: u32,
    /// Total size of this entry (Including all subfields)
    /*4 	5 	2*/
    entry_size: u16,
    /// Name Length least-significant 8 bits
    /*6 	6 	1*/
    name_length: u8,
    /// Type indicator (only if the feature bit for "directory entries have file type byte" is set, else this is the most-significant 8 bits of the Name Length)
    /*7 	7 	1*/
    type_indicator: DirectoryEntryType,
    /// N 	Name characters
    /*8 	8+N-1*/
    filename: Filename,
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
#[repr(u8)]
enum DirectoryEntryType {
    UnknownType,
    RegularFile,
    Directory,
    CharacterDevice,
    BlockDevice,
    Fifo,
    Socket,
    SymbolicLink,
}

impl DirectoryEntryHeader {
    pub fn get_filename(&self) -> &str {
        unsafe {
            let slice: &[u8] = core::slice::from_raw_parts(&self.filename.0 as *const u8, self.name_length as usize);
            core::str::from_utf8_unchecked(slice)
        }
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Filename(pub [u8; 256]);

impl fmt::Debug for Filename {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", "Filename")
    }
}

fn div_rounded_up(a: u32, b: u32) -> u32 {
    (a + b - 1) / b
}

struct EntryIter<'a> {
    filesystem: &'a mut Ext2Filesystem,
    inode: &'a Inode,
    cur_offset: u32,
    cur_dir_index: u16,
}

impl<'a> Iterator for EntryIter<'a> {
    type Item = DirectoryEntryHeader;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(d) = self.filesystem.find_entry(&self.inode, self.cur_offset) {
            self.cur_dir_index += 1;
            self.cur_offset += d.entry_size as u32;
            if d.inode == 0 {
                self.next()
            } else {
                Some(d)
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct File {
    inode: Inode,
    inode_addr: u32,
    curr_offset: u32,
}

impl File {
    pub fn seek(&mut self, s: SeekFrom) {
        match s {
            SeekFrom::Start(n) => {
                self.curr_offset = n as u32;
            }
            _ => unimplemented!(),
        }
    }
}

/// This newtype handle a pure IO object
/// Must implements 'read', 'write', 'seek', 'flush' and 'try_clone'
struct ReaderDisk(StdFile);

impl ReaderDisk {
    /// Raw read. Fill the buf with readen data on file object
    pub fn disk_read_buffer(&mut self, offset: u32, buf: &mut [u8]) -> usize {
        self.0.seek(SeekFrom::Start(offset as u64 + START_OF_PARTITION)).unwrap();
        self.0.read(buf).unwrap()
    }

    /// Raw write. Write the buf inside file object
    pub fn disk_write_buffer(&mut self, offset: u32, buf: &[u8]) -> usize {
        self.0.seek(SeekFrom::Start(offset as u64 + START_OF_PARTITION)).unwrap();
        let ret = self.0.write(buf).unwrap();
        self.0.flush().expect("flush failed");
        ret
    }

    /// Read a particulary struct in file object
    pub fn disk_read_struct<T: Copy>(&mut self, offset: u32) -> T {
        let t: T;
        unsafe {
            t = core::mem::uninitialized();
            self.disk_read_buffer(offset, core::slice::from_raw_parts_mut(&t as *const T as *mut u8, size_of::<T>()));
        }
        t
    }

    /// Write a particulary struct inside file object
    pub fn disk_write_struct<T: Copy>(&mut self, offset: u32, t: &T) {
        let s = unsafe { core::slice::from_raw_parts(t as *const _ as *const u8, size_of::<T>()) };
        self.disk_write_buffer(offset, s);
    }

    /// Try to clone xD
    pub fn try_clone(&self) -> std::io::Result<Self> {
        Ok(Self(self.0.try_clone()?))
    }
}

#[derive(Debug, Copy, Clone)]
pub enum IoError {
    NoSuchFileOrDirectory,
    FileOffsetOutOfFile,
    NotADirectory,
    NoSpaceLeftOnDevice,
}

struct Ext2Filesystem {
    superblock: SuperBlock,
    nbr_block_grp: u32,
    block_size: u32,
    reader_disk: ReaderDisk,
}

const START_OF_PARTITION: u64 = 2048 * 512;

impl Ext2Filesystem {
    pub fn new(f: StdFile) -> Self {
        let mut reader_disk = ReaderDisk(f);
        let superblock: SuperBlock = reader_disk.disk_read_struct(1024);

        let signature = superblock.ext2_signature;
        assert_eq!(signature, EXT2_SIGNATURE_MAGIC);

        let nbr_block_grp = div_rounded_up(superblock.nbr_blocks, superblock.block_per_block_grp);
        let nbr_block_grp2 = div_rounded_up(superblock.nbr_inode, superblock.inodes_per_block_grp);

        // consistency check
        assert_eq!(nbr_block_grp, nbr_block_grp2);

        let block_size = 1024 << superblock.log2_block_size;

        Self { block_size, superblock, nbr_block_grp, reader_disk }
    }

    pub fn try_clone(&self) -> std::io::Result<Self> {
        Ok(Self { reader_disk: self.reader_disk.try_clone()?, ..*self })
    }

    /// read the block group descriptor from the block group number starting at 0
    pub fn block_grp_addr(&mut self, n: u32) -> BlockNumber {
        // The table is located in the block immediately following the Superblock. So if the block size (determined from a field in the superblock) is 1024 bytes per block, the Block Group Descriptor Table will begin at block 2. For any other block size, it will begin at block 1. Remember that blocks are numbered starting at 0, and that block numbers don't usually correspond to physical block addresses.
        assert!(n <= self.nbr_block_grp);
        let offset = if self.block_size == 1024 { 2 } else { 1 };

        BlockNumber(offset + n * self.superblock.block_per_block_grp)
    }
    pub fn find_block_grp(&mut self, n: u32) -> BlockGroupDescriptor {
        let block_grp_block_number = self.block_grp_addr(n);
        let block_grp_addr = self.to_addr(block_grp_block_number);
        self.reader_disk.disk_read_struct(block_grp_addr)
    }

    pub fn to_addr(&self, block_number: BlockNumber) -> u32 {
        self.block_size * block_number.0
    }

    pub fn find_inode(&mut self, inode: u32) -> (Inode, u32) {
        println!("ENTERING_FIND_INODE {}", inode);
        assert!(inode >= 1);
        let block_grp = (inode - 1) / self.superblock.inodes_per_block_grp;
        let index = (inode - 1) % self.superblock.inodes_per_block_grp;
        let inode_offset = index * self.superblock.size_inode as u32;

        let block_grp_descriptor = self.find_block_grp(block_grp);
        // dbg!(block_grp_descriptor);
        let inode_addr = self.to_addr(block_grp_descriptor.inode_table) + inode_offset;
        // dbg!(inode_addr);

        (self.reader_disk.disk_read_struct(inode_addr), inode_addr)
    }

    pub fn find_entry(&mut self, inode: &Inode, offset_entry: u32) -> Option<DirectoryEntryHeader> {
        if offset_entry >= inode.low_size {
            return None;
        }
        //TODO: remove unwrap
        let base_addr = self.inode_data_address_at_offset(&inode, offset_entry).unwrap() as u32;
        let dir_header: DirectoryEntryHeader = self.reader_disk.disk_read_struct(base_addr);
        // let ptr = self.reader_disk.disk_read_exact(base_addr + size_of::<DirectoryEntryHeader>() as u32, dir_header.name_length as u32);
        Some(dir_header)
    }

    pub fn iter_entries<'a>(&'a mut self, inode: &'a Inode) -> Result<EntryIter<'a>, IoError> {
        if unsafe { !inode.type_and_perm.contains(TypeAndPerm::DIRECTORY) } {
            return Err(IoError::NotADirectory);
        }
        Ok(EntryIter { filesystem: self, inode, cur_dir_index: 0, cur_offset: 0 })
    }

    pub fn open(&mut self, path: &str) -> Result<File, IoError> {
        let mut inode = self.find_inode(2);
        for p in path.split('/') {
            let entry =
                self.iter_entries(&inode.0)?.find(|x| x.get_filename() == p).ok_or(IoError::NoSuchFileOrDirectory)?;
            // dbg!(entry.get_path());
            inode = self.find_inode(entry.inode);
        }
        Ok(File { inode: inode.0, inode_addr: inode.1, curr_offset: 0 })
    }

    fn alloc_block_on_grp(&mut self, n: u32) -> Option<BlockNumber> {
        let block_grp_addr = self.block_grp_addr(n);
        let mut block_grp = self.find_block_grp(n);
        if block_grp.nbr_unallocated_blocks == 0 {
            return None;
        }
        // TODO: dynamic alloc ?
        let bitmap_addr = self.to_addr(block_grp.block_usage_bitmap);
        let mut bitmap: [u8; 1024] = self.reader_disk.disk_read_struct(bitmap_addr);
        for i in 0..self.superblock.block_per_block_grp {
            if bitmap.get_bit(i as usize) {
                bitmap.set_bit(i as usize, true);
                self.reader_disk.disk_write_struct(bitmap_addr + i / 8, &bitmap[(i / 8) as usize]);
                block_grp.nbr_unallocated_blocks -= 1;
                self.reader_disk.disk_write_struct(self.to_addr(block_grp_addr), &block_grp);
                return Some(block_grp_addr + BlockNumber(i));
            }
        }
        None
    }

    fn alloc_block(&mut self) -> Option<BlockNumber> {
        for n in 0..self.nbr_block_grp {
            if let Some(addr) = self.alloc_block_on_grp(n) {
                return Some(addr);
            }
        }
        None
    }

    fn alloc_inode_data_address_at_offset(
        &mut self,
        inode: &mut Inode,
        _inode_addr: u32,
        offset: u32,
    ) -> Result<u32, IoError> {
        let block_off = offset / self.block_size;
        let _blocknumber_per_block = self.block_size as usize / size_of::<BlockNumber>();
        // let none_if_zero = |x| if x == 0 { None } else { Some(x) };
        dbg!(offset);

        // Simple Addressing
        let offset_start = 0;
        let offset_end = 12;
        if block_off >= offset_start && block_off < offset_end {
            if unsafe { inode.direct_block_pointers[block_off as usize] == BlockNumber(0) } {
                inode.direct_block_pointers[block_off as usize] =
                    self.alloc_block().ok_or(IoError::NoSpaceLeftOnDevice)?;
            }
            return Ok(self.to_addr(inode.direct_block_pointers[block_off as usize]) + offset % self.block_size);
            //return Some(self.to_addr(inode.direct_block_pointers[block_off as usize]) + offset % self.block_size);
        }
        unimplemented!();
    }

    /// Get the file location at offset 'offset'
    fn inode_data_address_at_offset(&mut self, inode: &Inode, offset: u32) -> Option<u32> {
        let block_off = offset / self.block_size;
        let blocknumber_per_block = self.block_size as usize / size_of::<BlockNumber>();
        let none_if_zero = |x| if x == BlockNumber(0) { None } else { Some(x) };
        dbg!(offset);

        // let create_or_return = |addr| {
        //     if addr == 0 {
        //         if creat {
        //             self.allocate_block();
        //         } else {
        //             None
        //         }
        //     }
        //     Some(addr)
        // }

        // Simple Addressing
        let mut offset_start = 0;
        let mut offset_end = 12;
        if block_off >= offset_start && block_off < offset_end {
            return Some(
                self.to_addr(none_if_zero(inode.direct_block_pointers[block_off as usize])?) + offset % self.block_size,
            );
        }

        // Singly Indirect Addressing
        // 12 * blocksize .. 12 * blocksize + (blocksize / 4) * blocksize
        offset_start = offset_end;
        offset_end += blocknumber_per_block as u32;
        if block_off >= offset_start && block_off < offset_end {
            dbg!("singly indirect addressing");
            let off = block_off - offset_start;
            // let pointer_table = vec![BlockNumber(0); blocknumber_per_block];
            let pointer: BlockNumber = self.reader_disk.disk_read_struct(
                self.to_addr(inode.singly_indirect_block_pointers) + off * size_of::<BlockNumber>() as u32,
            );
            dbg!(pointer);

            return Some(self.to_addr(pointer) + offset % self.block_size);
        }

        // Doubly Indirect Addressing
        offset_start = offset_end;
        offset_end += (blocknumber_per_block * blocknumber_per_block) as u32;
        if block_off >= offset_start && block_off < offset_end {
            dbg!("doubly indirect addressing");
            let off = (block_off - offset_start) / blocknumber_per_block as u32;
            let pointer_to_pointer: BlockNumber = self.reader_disk.disk_read_struct(
                self.to_addr(inode.doubly_indirect_block_pointers) + off * size_of::<BlockNumber>() as u32,
            );

            let off = (block_off - offset_start) % blocknumber_per_block as u32;
            let pointer: BlockNumber = self
                .reader_disk
                .disk_read_struct(self.to_addr(pointer_to_pointer) + off * size_of::<BlockNumber>() as u32);

            return Some(self.to_addr(pointer) + offset % self.block_size);
        }

        // Triply Indirect Addressing
        offset_start = offset_end;
        offset_end += (blocknumber_per_block * blocknumber_per_block * blocknumber_per_block) as u32;
        if block_off >= offset_start && block_off < offset_end {
            dbg!("triply indirect addressing");
            let off = dbg!((block_off - offset_start) / (blocknumber_per_block * blocknumber_per_block) as u32);
            let pointer_to_pointer_to_pointer: BlockNumber = self.reader_disk.disk_read_struct(
                self.to_addr(inode.triply_indirect_block_pointers) + off * size_of::<BlockNumber>() as u32,
            );

            let off = dbg!(
                (((block_off - offset_start) % (blocknumber_per_block * blocknumber_per_block) as u32)
                    / blocknumber_per_block as u32) as u32
            );
            let pointer_to_pointer: BlockNumber = self
                .reader_disk
                .disk_read_struct(self.to_addr(pointer_to_pointer_to_pointer) + off * size_of::<BlockNumber>() as u32);

            let off = dbg!(
                (((block_off - offset_start) % (blocknumber_per_block * blocknumber_per_block) as u32)
                    % blocknumber_per_block as u32) as u32
            );
            let pointer: BlockNumber = self
                .reader_disk
                .disk_read_struct(self.to_addr(pointer_to_pointer) + off * size_of::<BlockNumber>() as u32);

            return Some(self.to_addr(pointer) + offset % self.block_size);
        }
        panic!("out of file bound");
    }

    pub fn read(&mut self, file: &mut File, buf: &mut [u8]) -> Result<usize, IoError> {
        // TODO: do indirect
        let file_curr_offset_start = file.curr_offset;
        if dbg!(file.curr_offset) > dbg!(file.inode.low_size) {
            return Err(IoError::FileOffsetOutOfFile);
        }
        if file.curr_offset == file.inode.low_size {
            return Ok(0);
        }

        let data_address = self.inode_data_address_at_offset(&file.inode, file.curr_offset).unwrap();
        let offset = min(
            (file.inode.low_size - file.curr_offset) as usize,
            min((self.block_size - file.curr_offset % self.block_size) as usize, buf.len()),
        );
        let data_read = self.reader_disk.disk_read_buffer(data_address, &mut buf[0..offset]);
        file.curr_offset += data_read as u32;
        if data_read < offset {
            return Ok((file.curr_offset - file_curr_offset_start) as usize);
        }

        for chunk in buf[offset..].chunks_mut(self.block_size as usize) {
            let data_address = self.inode_data_address_at_offset(&file.inode, file.curr_offset).unwrap();
            let offset = min((file.inode.low_size - file.curr_offset) as usize, chunk.len());
            let data_read = self.reader_disk.disk_read_buffer(data_address, &mut chunk[0..offset]);
            file.curr_offset += data_read as u32;
            if data_read < chunk.len() {
                return Ok((file.curr_offset - file_curr_offset_start) as usize);
            }
        }
        Ok((file.curr_offset - file_curr_offset_start) as usize)
    }

    pub fn write(&mut self, file: &mut File, buf: &[u8]) -> Result<usize, IoError> {
        // TODO: do indirect
        let file_curr_offset_start = file.curr_offset;
        if dbg!(file.curr_offset) > dbg!(file.inode.low_size) {
            return Err(IoError::FileOffsetOutOfFile);
        }
        if file.curr_offset == file.inode.low_size {
            return Ok(0);
        }

        let data_address = self
            .inode_data_address_at_offset(&file.inode, file.curr_offset)
            .ok_or(IoError::NoSpaceLeftOnDevice)
            .or_else(|_e| {
                self.alloc_inode_data_address_at_offset(&mut file.inode, file.inode_addr, file.curr_offset)
            })?;
        let offset = min((self.block_size - file.curr_offset % self.block_size) as usize, buf.len());
        let data_read = self.reader_disk.disk_write_buffer(data_address, &buf[0..offset]);
        file.curr_offset += data_read as u32;
        if data_read < offset {
            return Ok((file.curr_offset - file_curr_offset_start) as usize);
        }

        for chunk in buf[offset..].chunks(self.block_size as usize) {
            let data_address = self
                .inode_data_address_at_offset(&file.inode, file.curr_offset)
                .ok_or(IoError::NoSpaceLeftOnDevice)
                .or_else(|_e| {
                    self.alloc_inode_data_address_at_offset(&mut file.inode, file.inode_addr, file.curr_offset)
                })?;
            let data_read = self.reader_disk.disk_write_buffer(data_address, &chunk);
            file.curr_offset += data_read as u32;
            if file.inode.low_size < file.curr_offset {
                file.inode.low_size = file.curr_offset;
                self.reader_disk.disk_write_struct(file.inode_addr, &file.inode);
            }
            if data_read < chunk.len() {
                return Ok((file.curr_offset - file_curr_offset_start) as usize);
            }
        }
        Ok((file.curr_offset - file_curr_offset_start) as usize)
    }
}

#[allow(dead_code)]
fn find_string(path: &str, patern: &[u8]) {
    let data = std::fs::read(path).unwrap();
    for i in 0..data.len() - patern.len() {
        if &data[i..i + patern.len()] == patern {
            println!("match");
            // dbg!(i);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let f = OpenOptions::new().write(true).read(true).open(&args[1]).unwrap();
    let mut ext2 = Ext2Filesystem::new(f);
    dbg!(ext2.superblock);
    let (inode, _) = ext2.find_inode(2);
    dbg!(inode);
    let dir_entry = ext2.find_entry(&inode, 0);
    dbg!(dir_entry);
    for e in ext2.try_clone().unwrap().iter_entries(&inode).unwrap().skip(2) {
        dbg!(e.get_filename());
        let (inode, _) = ext2.find_inode(e.inode);
        println!("{:?}", inode);
        println!("inner");
        for e in ext2.iter_entries(&inode).unwrap().skip(2) {
            dbg!(e.get_filename());
            dbg!(e);
        }
        println!("end inner");
    }
    let mut file = ext2.open("dir/banane").unwrap();
    println!("{:#?}", file);

    println!("READ");
    let mut buf = [42; 10];
    let count = ext2.read(&mut file, &mut buf).unwrap();
    unsafe {
        println!("string: {}", core::str::from_utf8_unchecked(&buf[0..count]));
    }

    file.seek(SeekFrom::Start(0));
    println!("WRITE");
    let s = "123456789a".repeat(1000);
    ext2.write(&mut file, &s.as_bytes()).expect("write failed");

    file.seek(SeekFrom::Start(0));
    println!("READ");
    let mut buf = [42; 10000];
    let count = ext2.read(&mut file, &mut buf).unwrap();
    unsafe {
        println!("string: {}", core::str::from_utf8_unchecked(&buf[0..count]));
    }

    // let mut file = ext2.open("dir/indirect").unwrap();
    // println!("{:#?}", file);
    // let mut buf = [42; 1024];
    // let mut indirect_dump = StdFile::create("indirect_dump").unwrap();
    // while {
    //     let x = ext2.read(&mut file, &mut buf).unwrap();
    //     indirect_dump.write(&buf[0..x]).unwrap();
    //     x > 0
    // } {}
    // let mut file = ext2.open("dir/doubly_indirect").unwrap();
    // println!("{:#?}", file);
    // let mut buf = [42; 10];
    // let mut indirect_dump = StdFile::create("doubly_indirect_dump").unwrap();
    // while {
    //     let x = ext2.read(&mut file, &mut buf).unwrap();
    //     indirect_dump.write(&buf[0..x]).unwrap();
    //     x > 0
    // } {}
    // let mut file = ext2.open("dir/triply_indirect").unwrap();
    // println!("{:#?}", file);
    // let mut buf = [42; 1024];
    // let mut indirect_dump = StdFile::create("triply_indirect_dump").unwrap();
    // while {
    //     let x = ext2.read(&mut file, &mut buf).unwrap();
    //     indirect_dump.write(&buf[0..x]).unwrap();
    //     x > 0
    // } {}
    // while let Some(x) = ext2.alloc_block() {
    //     dbg!(x);
    // }
    // dbg!(count);

    // assert!(ext2.open("dir/artichaud").is_err());
    // find_string("simple_diskp1", "lescarotessontcuites".as_bytes());
}
