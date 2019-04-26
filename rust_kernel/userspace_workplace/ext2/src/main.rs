//! this module contains a ext2 driver
//! see [osdev](https://wiki.osdev.org/Ext2)

use bit_field::BitArray;

use core::cmp::min;
use core::fmt;
use core::mem::size_of;
use core::ops::Add;

use std::fs::File as StdFile;
use std::fs::OpenOptions;
use std::io::SeekFrom;

mod reader_disk;
use reader_disk::ReaderDisk;

mod header;
use header::{BlockGroupDescriptor, SuperBlock};

mod inode;
use inode::{Inode, TypeAndPerm};

/// The Ext2 file system divides up disk space into logical blocks of contiguous space.
/// The size of blocks need not be the same size as the sector size of the disk the file system resides on.
/// The size of blocks can be determined by reading the field starting at byte 24 in the Superblock.
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct Block(u32);

/// Add boilerplate for Block
impl Add<Self> for Block {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

/// Local Result structure
pub type IoResult<T> = core::result::Result<T, IoError>;

/// Local Error structure
#[derive(Debug, Copy, Clone)]
pub enum IoError {
    NoSuchFileOrDirectory,
    FileOffsetOutOfFile,
    NotADirectory,
    NoSpaceLeftOnDevice,
}

/// Used to help confirm the presence of Ext2 on a volume
const EXT2_SIGNATURE_MAGIC: u16 = 0xef53;

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

struct Ext2Filesystem {
    superblock: SuperBlock,
    nbr_block_grp: u32,
    block_size: u32,
    reader_disk: ReaderDisk,
}

impl Ext2Filesystem {
    pub fn new(f: StdFile) -> Self {
        let mut reader_disk = ReaderDisk(f);
        let superblock: SuperBlock = reader_disk.disk_read_struct(1024);

        let signature = superblock.get_ext2_signature();
        assert_eq!(signature, EXT2_SIGNATURE_MAGIC);

        let nbr_block_grp = superblock.get_nbr_block_grp();
        let nbr_block_grp2 = superblock.get_inode_block_grp();

        // consistency check
        assert_eq!(nbr_block_grp, nbr_block_grp2);

        let block_size = 1024 << superblock.get_log2_block_size();

        Self { block_size, superblock, nbr_block_grp, reader_disk }
    }

    pub fn try_clone(&self) -> std::io::Result<Self> {
        Ok(Self { reader_disk: self.reader_disk.try_clone()?, ..*self })
    }

    /// read the block group descriptor from the block group number starting at 0
    pub fn block_grp_addr(&mut self, n: u32) -> Block {
        // The table is located in the block immediately following the Superblock. So if the block size (determined from a field in the superblock) is 1024 bytes per block, the Block Group Descriptor Table will begin at block 2. For any other block size, it will begin at block 1. Remember that blocks are numbered starting at 0, and that block numbers don't usually correspond to physical block addresses.
        assert!(n <= self.nbr_block_grp);
        let offset = if self.block_size == 1024 { 2 } else { 1 };

        Block(offset + n * self.superblock.get_block_per_block_grp())
    }
    pub fn find_block_grp(&mut self, n: u32) -> BlockGroupDescriptor {
        let block_grp_block_number = self.block_grp_addr(n);
        let block_grp_addr = self.to_addr(block_grp_block_number);
        self.reader_disk.disk_read_struct(block_grp_addr)
    }

    pub fn to_addr(&self, block_number: Block) -> u32 {
        self.block_size * block_number.0
    }

    pub fn find_inode(&mut self, inode: u32) -> (Inode, u32) {
        println!("ENTERING_FIND_INODE {}", inode);
        assert!(inode >= 1);
        let block_grp = (inode - 1) / self.superblock.get_inode_per_block_grp();
        let index = (inode - 1) % self.superblock.get_inode_per_block_grp();
        let inode_offset = index * self.superblock.get_size_inode() as u32;

        let block_grp_descriptor = self.find_block_grp(block_grp);
        // dbg!(block_grp_descriptor);
        let inode_addr = self.to_addr(block_grp_descriptor.get_inode_table_address()) + inode_offset;
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

    pub fn iter_entries<'a>(&'a mut self, inode: &'a Inode) -> IoResult<EntryIter<'a>> {
        if unsafe { !inode.type_and_perm.contains(TypeAndPerm::DIRECTORY) } {
            return Err(IoError::NotADirectory);
        }
        Ok(EntryIter { filesystem: self, inode, cur_dir_index: 0, cur_offset: 0 })
    }

    pub fn open(&mut self, path: &str) -> IoResult<File> {
        let mut inode = self.find_inode(2);
        for p in path.split('/') {
            let entry =
                self.iter_entries(&inode.0)?.find(|x| x.get_filename() == p).ok_or(IoError::NoSuchFileOrDirectory)?;
            // dbg!(entry.get_path());
            inode = self.find_inode(entry.inode);
        }
        Ok(File { inode: inode.0, inode_addr: inode.1, curr_offset: 0 })
    }

    fn alloc_block_on_grp(&mut self, n: u32) -> Option<Block> {
        let block_grp_addr = self.block_grp_addr(n);
        let mut block_grp = self.find_block_grp(n);
        if block_grp.get_nbr_unallocated_blocks() == 0 {
            return None;
        }
        // TODO: dynamic alloc ?
        let bitmap_addr = self.to_addr(block_grp.get_block_usage_bitmap_address());
        let mut bitmap: [u8; 1024] = self.reader_disk.disk_read_struct(bitmap_addr);
        for i in 0..self.superblock.get_block_per_block_grp() {
            if bitmap.get_bit(i as usize) {
                bitmap.set_bit(i as usize, true);
                self.reader_disk.disk_write_struct(bitmap_addr + i / 8, &bitmap[(i / 8) as usize]);
                block_grp.allocate_block();
                self.reader_disk.disk_write_struct(self.to_addr(block_grp_addr), &block_grp);
                return Some(block_grp_addr + Block(i));
            }
        }
        None
    }

    fn alloc_block(&mut self) -> Option<Block> {
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
    ) -> IoResult<u32> {
        let block_off = offset / self.block_size;
        let _blocknumber_per_block = self.block_size as usize / size_of::<Block>();
        // let none_if_zero = |x| if x == 0 { None } else { Some(x) };
        dbg!(offset);

        // Simple Addressing
        let offset_start = 0;
        let offset_end = 12;
        if block_off >= offset_start && block_off < offset_end {
            if unsafe { inode.direct_block_pointers[block_off as usize] == Block(0) } {
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
        let blocknumber_per_block = self.block_size as usize / size_of::<Block>();
        let none_if_zero = |x| if x == Block(0) { None } else { Some(x) };
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
            // let pointer_table = vec![Block(0); blocknumber_per_block];
            let pointer: Block = self
                .reader_disk
                .disk_read_struct(self.to_addr(inode.singly_indirect_block_pointers) + off * size_of::<Block>() as u32);
            dbg!(pointer);

            return Some(self.to_addr(pointer) + offset % self.block_size);
        }

        // Doubly Indirect Addressing
        offset_start = offset_end;
        offset_end += (blocknumber_per_block * blocknumber_per_block) as u32;
        if block_off >= offset_start && block_off < offset_end {
            dbg!("doubly indirect addressing");
            let off = (block_off - offset_start) / blocknumber_per_block as u32;
            let pointer_to_pointer: Block = self
                .reader_disk
                .disk_read_struct(self.to_addr(inode.doubly_indirect_block_pointers) + off * size_of::<Block>() as u32);

            let off = (block_off - offset_start) % blocknumber_per_block as u32;
            let pointer: Block =
                self.reader_disk.disk_read_struct(self.to_addr(pointer_to_pointer) + off * size_of::<Block>() as u32);

            return Some(self.to_addr(pointer) + offset % self.block_size);
        }

        // Triply Indirect Addressing
        offset_start = offset_end;
        offset_end += (blocknumber_per_block * blocknumber_per_block * blocknumber_per_block) as u32;
        if block_off >= offset_start && block_off < offset_end {
            dbg!("triply indirect addressing");
            let off = dbg!((block_off - offset_start) / (blocknumber_per_block * blocknumber_per_block) as u32);
            let pointer_to_pointer_to_pointer: Block = self
                .reader_disk
                .disk_read_struct(self.to_addr(inode.triply_indirect_block_pointers) + off * size_of::<Block>() as u32);

            let off = dbg!(
                (((block_off - offset_start) % (blocknumber_per_block * blocknumber_per_block) as u32)
                    / blocknumber_per_block as u32) as u32
            );
            let pointer_to_pointer: Block = self
                .reader_disk
                .disk_read_struct(self.to_addr(pointer_to_pointer_to_pointer) + off * size_of::<Block>() as u32);

            let off = dbg!(
                (((block_off - offset_start) % (blocknumber_per_block * blocknumber_per_block) as u32)
                    % blocknumber_per_block as u32) as u32
            );
            let pointer: Block =
                self.reader_disk.disk_read_struct(self.to_addr(pointer_to_pointer) + off * size_of::<Block>() as u32);

            return Some(self.to_addr(pointer) + offset % self.block_size);
        }
        panic!("out of file bound");
    }

    pub fn read(&mut self, file: &mut File, buf: &mut [u8]) -> IoResult<usize> {
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

    pub fn write(&mut self, file: &mut File, buf: &[u8]) -> IoResult<usize> {
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
