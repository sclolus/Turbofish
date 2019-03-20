use core::mem::size_of;
use std::fs::File as StdFile;
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
struct BlockNumber(u32);

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
    optional_features_present: u32,
    /// Required features present (features that are required to be supported to read or write. see below)
    /*96   99   4 */
    required_features_present: u32,
    /// Features that if not supported, the volume must be mounted read-only see below)
    /*100  103  4 */
    feature_must_read_only: u32,
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
    type_and_permissions: u16,
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
    flags: u32,
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
    type_indicator: u8,
    /// N 	Name characters
    /*8 	8+N-1*/
    filename: Filename,
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

struct Ext2Filesystem {
    superblock: SuperBlock,
    nbr_block_grp: u32,
    block_size: u32,
    f: StdFile,
    buf: [u8; 4096],
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
        if self.cur_offset < self.inode.low_size {
            let d = self.filesystem.find_entry(&self.inode, self.cur_offset);
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
    curr_offset: u32,
}

#[derive(Debug, Copy, Clone)]
pub struct IoError;

impl Ext2Filesystem {
    pub fn new(mut f: StdFile) -> Self {
        let mut buf = [0; 4096];

        // the base superblock start at byte 1024
        f.seek(SeekFrom::Start(1024)).unwrap();
        f.read(&mut buf[0..size_of::<SuperBlock>()]).unwrap();
        let superblock: SuperBlock = unsafe { core::mem::transmute_copy(&buf) };

        // println!("{:#?}", superblock);

        unsafe {
            assert_eq!(superblock.ext2_signature, EXT2_SIGNATURE_MAGIC);
        }

        let nbr_block_grp = div_rounded_up(superblock.nbr_blocks, superblock.block_per_block_grp);
        let nbr_block_grp2 = div_rounded_up(superblock.nbr_inode, superblock.inodes_per_block_grp);
        // consistency check
        assert_eq!(nbr_block_grp, nbr_block_grp2);

        let block_size = 1024 << superblock.log2_block_size;
        // dbg!(block_size);

        Self { block_size, superblock, nbr_block_grp, f, buf }
    }

    pub fn try_clone(&self) -> std::io::Result<Self> {
        Ok(Self { f: self.f.try_clone()?, ..*self })
    }

    pub fn find_block_grp(&mut self, n: u32) -> BlockGroupDescriptor {
        // The table is located in the block immediately following the Superblock. So if the block size (determined from a field in the superblock) is 1024 bytes per block, the Block Group Descriptor Table will begin at block 2. For any other block size, it will begin at block 1. Remember that blocks are numbered starting at 0, and that block numbers don't usually correspond to physical block addresses.
        assert!(n <= self.nbr_block_grp);
        let offset = if self.block_size == 1024 { 2 * 1024 } else { self.block_size };

        let block_group_descriptr_addr = offset + (n - 1) * self.block_size;
        self.disk_read_struct(block_group_descriptr_addr)
    }

    pub fn to_addr(&self, block_number: BlockNumber) -> u32 {
        self.block_size * block_number.0
    }

    pub fn find_inode(&mut self, inode: u32) -> Inode {
        println!("ENTERING_FIND_INODE {}", inode);
        assert!(inode >= 1);
        let block_grp = (inode - 1) / self.superblock.inodes_per_block_grp + 1;
        let index = (inode - 1) % self.superblock.inodes_per_block_grp;
        let inode_offset = index * self.superblock.size_inode as u32;

        let block_grp_descriptor = self.find_block_grp(block_grp);
        // dbg!(block_grp_descriptor);
        let inode_addr = self.to_addr(block_grp_descriptor.inode_table) + inode_offset;
        // dbg!(inode_addr);

        self.disk_read_struct(inode_addr)
    }

    pub fn find_entry(&mut self, inode: &Inode, offset_entry: u32) -> DirectoryEntryHeader {
        //TODO: handle the indirect block pointers too
        // assert!(offset_entry < self.block_size);
        let base_addr = self.to_addr(inode.direct_block_pointers[(offset_entry / self.block_size) as usize])
            + offset_entry % self.block_size;
        let dir_header: DirectoryEntryHeader = self.disk_read_struct(base_addr);
        // let ptr = self.disk_read_exact(base_addr + size_of::<DirectoryEntryHeader>() as u32, dir_header.name_length as u32);
        dir_header
    }

    pub fn iter_entries<'a>(&'a mut self, inode: &'a Inode) -> EntryIter<'a> {
        EntryIter { filesystem: self, inode, cur_dir_index: 0, cur_offset: 0 }
    }

    pub fn disk_read_struct<T: Copy>(&mut self, offset: u32) -> T {
        self.f.seek(SeekFrom::Start(offset as u64)).unwrap();
        self.f.read(&mut self.buf[0..core::mem::size_of::<T>()]).unwrap();
        unsafe { core::mem::transmute_copy(&self.buf) }
    }

    pub fn disk_read_exact(&mut self, offset: u32, length: u32) -> *const u8 {
        assert!((length as usize) < self.buf.len());
        self.f.seek(SeekFrom::Start(offset as u64)).unwrap();
        self.f.read(&mut self.buf[0..length as usize]).unwrap();
        &self.buf as *const u8
    }

    pub fn disk_read_buffer(&mut self, offset: u32, buf: &mut [u8]) -> usize {
        self.f.seek(SeekFrom::Start(offset as u64)).unwrap();
        self.f.read(buf).unwrap()
    }

    pub fn open(&mut self, filename: &str) -> Result<File, IoError> {
        let mut inode = self.find_inode(2);
        for p in filename.split('/') {
            let entry = self.iter_entries(&inode).find(|x| x.get_filename() == p).ok_or(IoError)?;
            dbg!(entry.get_filename());
            inode = self.find_inode(entry.inode);
        }
        Ok(File { inode, curr_offset: 0 })
    }

    pub fn read(&mut self, file: File, buf: &mut [u8]) -> Result<usize, IoError> {
        // TODO: do indirect
        let len = buf.len();
        Ok(self.disk_read_buffer(
            self.to_addr(file.inode.direct_block_pointers[(file.curr_offset / self.block_size) as usize])
                + file.curr_offset,
            &mut buf[0..core::cmp::min(len, (self.block_size - file.curr_offset % self.block_size) as usize)],
        ))
    }
}

fn find_string(path: &str, patern: &[u8]) {
    let data = std::fs::read(path).unwrap();
    for i in 0..data.len() - patern.len() {
        if &data[i..i + patern.len()] == patern {
            println!("match");
            dbg!(i);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let f = StdFile::open(&args[1]).unwrap();
    let mut ext2 = Ext2Filesystem::new(f);
    dbg!(ext2.superblock);
    let inode = ext2.find_inode(2);
    // dbg!(inode);
    // let dir_entry = ext2.find_entry(&inode, 0);
    // dbg!(dir_entry);
    for e in ext2.try_clone().unwrap().iter_entries(&inode).skip(2) {
        dbg!(e.get_filename());
        let inode = ext2.find_inode(e.inode);
        println!("{:?}", inode);
        println!("inner");
        for e in ext2.iter_entries(&inode).skip(2) {
            dbg!(e);
            dbg!(e.get_filename());
        }
        println!("end inner");
    }
    let file = ext2.open("dir/banane").unwrap();
    println!("{:#?}", file);
    let mut buf = [42; 1024];
    let count = ext2.read(file, &mut buf).unwrap();
    dbg!(count);

    unsafe {
        println!("string: {}", core::str::from_utf8_unchecked(&buf));
    }
    let file = ext2.open("dir/artichaud");
    println!("{:#?}", file);
    find_string("simple_diskp1", "lescarotessontcuites".as_bytes());
}
