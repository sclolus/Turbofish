//! this module contains a ext2 driver
//! see [osdev](https://wiki.osdev.org/Ext2)

#![allow(dead_code)]

mod disk;
use disk::ReaderDisk;

mod header;
use header::{BlockGroupDescriptor, SuperBlock};

mod body;
use body::{DirectoryEntryHeader, Inode, TypeAndPerm};

use bit_field::BitArray;

use core::cmp::min;
use core::mem::size_of;
use core::ops::{Add, Mul};

use std::fs::File as StdFile;
use std::io::SeekFrom;

/// The Ext2 file system divides up disk space into logical blocks of contiguous space.
/// The size of blocks need not be the same size as the sector size of the disk the file system resides on.
/// The size of blocks can be determined by reading the field starting at byte 24 in the Superblock.
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(transparent)]
pub struct Block(u32);

/// Roundup style function
pub fn div_rounded_up(a: u32, b: u32) -> u32 {
    (a + b - 1) / b
}

/// Add boilerplate for Block
impl Add<Self> for Block {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Mul<u32> for Block {
    type Output = Self;
    fn mul(self, rhs: u32) -> Self::Output {
        Self(self.0 * rhs)
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

/// Magic iterator over the entire fileSytem
pub struct EntryIter<'a> {
    filesystem: &'a mut Ext2Filesystem,
    file: File,
    cur_dir_index: u16,
}

impl<'a> Iterator for EntryIter<'a> {
    type Item = DirectoryEntryHeader;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(d) = self.filesystem.find_entry(&mut self.file) {
            self.cur_dir_index += 1;
            self.file.curr_offset += d.entry_size as u32;
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

/// Local file structure
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

/// Global structure of ext2Filesystem
#[derive(Debug)]
pub struct Ext2Filesystem {
    superblock: SuperBlock,
    disk: ReaderDisk,
    nbr_block_grp: u32,
    block_size: u32,
}

impl Ext2Filesystem {
    /// Invocation of a new FileSystem instance: take a FD and his reader as parameter
    pub fn new(f: StdFile) -> Self {
        let mut disk = ReaderDisk(f);
        let superblock: SuperBlock = disk.read_struct(1024);

        let signature = superblock.get_ext2_signature();
        assert_eq!(signature, EXT2_SIGNATURE_MAGIC);

        // consistency check
        let nbr_block_grp = superblock.get_nbr_block_grp();
        assert_eq!(nbr_block_grp, superblock.get_inode_block_grp());

        let block_size = 1024 << superblock.get_log2_block_size();

        Self {
            block_size,
            superblock,
            nbr_block_grp,
            disk,
        }
    }

    /// Try to clone the Ext2Filesystem instance
    pub fn try_clone(&self) -> std::io::Result<Self> {
        Ok(Self {
            disk: self.disk.try_clone()?,
            ..*self
        })
    }

    /// Open a File
    pub fn open(&mut self, path: &str) -> IoResult<File> {
        let mut inode = self.find_inode(2);
        for p in path.split('/') {
            let entry = self
                .iter_entries(inode.0, inode.1)?
                .find(|x| x.get_filename() == p)
                .ok_or(IoError::NoSuchFileOrDirectory)?;
            // dbg!(entry.get_path());
            inode = self.find_inode(entry.inode);
        }
        Ok(File {
            inode: inode.0,
            inode_addr: inode.1,
            curr_offset: 0,
        })
    }

    pub fn to_addr(&self, block_number: Block) -> u32 {
        self.block_size * block_number.0
    }

    /// get inode nbr inode and return the Inode and it's address
    pub fn find_inode(&mut self, inode: u32) -> (Inode, u32) {
        println!("ENTERING_FIND_INODE {}", inode);
        assert!(inode >= 1);
        let block_grp = (inode - 1) / self.superblock.get_inode_per_block_grp();
        let index = (inode - 1) % self.superblock.get_inode_per_block_grp();
        let inode_offset = index * self.superblock.get_size_inode() as u32;

        let (block_grp_descriptor, _) = self.get_block_grp_descriptor(block_grp);
        // dbg!(block_grp_descriptor);
        let inode_addr =
            self.to_addr(block_grp_descriptor.get_inode_table_address()) + inode_offset;
        // dbg!(inode_addr);

        (self.disk.read_struct(inode_addr), inode_addr)
    }

    pub fn find_entry(&mut self, file: &mut File) -> Option<DirectoryEntryHeader> {
        let offset_entry = file.curr_offset;
        let inode = file.inode;
        if offset_entry >= inode.low_size {
            return None;
        }
        //TODO: remove unwrap
        let base_addr = self.inode_data(file).unwrap() as u32;
        let dir_header: DirectoryEntryHeader = self.disk.read_struct(base_addr);
        // let ptr = self.disk.disk_read_exact(base_addr + size_of::<DirectoryEntryHeader>() as u32, dir_header.name_length as u32);
        Some(dir_header)
    }

    pub fn iter_entries<'a>(
        &'a mut self,
        inode: Inode,
        inode_addr: u32,
    ) -> IoResult<EntryIter<'a>> {
        if unsafe { !inode.type_and_perm.contains(TypeAndPerm::DIRECTORY) } {
            return Err(IoError::NotADirectory);
        }
        Ok(EntryIter {
            filesystem: self,
            file: File {
                curr_offset: 0,
                inode,
                inode_addr,
            },
            cur_dir_index: 0,
        })
    }

    /// read the block group descriptor from the block group number starting at 0
    pub fn block_grp_addr(&mut self, n: u32) -> u32 {
        // The table is located in the block immediately following the Superblock. So if the block size (determined from a field in the superblock) is 1024 bytes per block, the Block Group Descriptor Table will begin at block 2. For any other block size, it will begin at block 1. Remember that blocks are numbered starting at 0, and that block numbers don't usually correspond to physical block addresses.
        assert!(n <= self.nbr_block_grp);
        let offset = if self.block_size == 1024 { 2 } else { 1 };

        self.to_addr(Block(offset)) + n * size_of::<BlockGroupDescriptor>() as u32
    }

    /// return the nth group descriptor with its address
    pub fn get_block_grp_descriptor(&mut self, n: u32) -> (BlockGroupDescriptor, u32) {
        let block_grp_addr = self.block_grp_addr(n);
        let block_grp: BlockGroupDescriptor = self.disk.read_struct(block_grp_addr);
        (block_grp, block_grp_addr)
    }

    /// try to allocate a new block on block grp number `n`
    fn alloc_block_on_grp(&mut self, n: u32) -> Option<Block> {
        let (mut block_dtr, block_dtr_addr) = self.get_block_grp_descriptor(n);
        if block_dtr.get_nbr_unallocated_blocks() == 0 {
            return None;
        }
        // TODO: dynamic alloc ?
        let bitmap_addr = self.to_addr(block_dtr.get_block_usage_bitmap_address());
        let mut bitmap: [u8; 1024] = self.disk.read_struct(bitmap_addr);
        for i in 0..self.superblock.get_block_per_block_grp().0 {
            if !bitmap.get_bit(i as usize) {
                bitmap.set_bit(i as usize, true);
                self.disk
                    .write_struct(bitmap_addr + i / 8, &bitmap[(i / 8) as usize]);
                block_dtr.allocate_block();
                self.disk.write_struct(block_dtr_addr, &block_dtr);
                // TODO: Check the + 1
                return Some(self.superblock.get_block_per_block_grp() * n + Block(i + 1));
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

    fn inode_data(&mut self, file: &mut File) -> Option<u32> {
        self.inode_data_alloc(file, false).ok()
    }
    fn alloc_pointer(&mut self, pointer_addr: u32, alloc: bool) -> Result<Block, IoError> {
        let err_if_zero = |x| {
            if x == Block(0) {
                Err(IoError::FileOffsetOutOfFile)
            } else {
                Ok(x)
            }
        };
        err_if_zero({
            let pointer = self.disk.read_struct(pointer_addr);
            if alloc && pointer == Block(0) {
                let new_block = self.alloc_block().ok_or(IoError::NoSpaceLeftOnDevice)?;
                self.disk.write_struct(pointer_addr, &new_block);
                new_block
            } else {
                pointer
            }
        })
    }
    /// Get the file location at offset 'offset'
    fn inode_data_alloc(&mut self, file: &mut File, alloc: bool) -> Result<u32, IoError> {
        let _inode_addr = file.inode_addr;
        let offset = file.curr_offset;
        let block_off = offset / self.block_size;
        let blocknumber_per_block = self.block_size as usize / size_of::<Block>();
        let err_if_zero = |x| {
            if x == Block(0) {
                Err(IoError::FileOffsetOutOfFile)
            } else {
                Ok(x)
            }
        };

        // let mut alloc_on_inode =
        //     |block_addr: &mut Block, field_offset: u32| -> Result<Block, IoError> {
        //         err_if_zero({
        //             if alloc && *block_addr == Block(0) {
        //                 *block_addr = self.alloc_block().ok_or(IoError::NoSpaceLeftOnDevice)?;
        //                 self.disk
        //                     .write_struct(inode_addr + field_offset, block_addr);
        //             }
        //             *block_addr
        //         })
        //     };

        /* SIMPLE ADDRESSING */
        let mut offset_start = 0;
        let mut offset_end = 12;
        if block_off >= offset_start && block_off < offset_end {
            if alloc && unsafe { file.inode.direct_block_pointers[block_off as usize] == Block(0) }
            {
                file.inode.direct_block_pointers[block_off as usize] =
                    self.alloc_block().ok_or(IoError::NoSpaceLeftOnDevice)?;
                self.disk.write_struct(file.inode_addr, &file.inode);
            }
            // dbg!(&file.inode);
            return Ok(self.to_addr(err_if_zero(
                file.inode.direct_block_pointers[block_off as usize],
            )?) + offset % self.block_size);
        }

        /* SINGLY INDIRECT ADDRESSING */
        // 12 * blocksize .. 12 * blocksize + (blocksize / 4) * blocksize
        offset_start = offset_end;
        offset_end += blocknumber_per_block as u32;
        if block_off >= offset_start && block_off < offset_end {
            // dbg!("singly indirect addressing");

            let off = block_off - offset_start;

            let singly_indirect = err_if_zero({
                if alloc && unsafe { file.inode.singly_indirect_block_pointers == Block(0) } {
                    file.inode.singly_indirect_block_pointers =
                        self.alloc_block().ok_or(IoError::NoSpaceLeftOnDevice)?;
                    self.disk.write_struct(file.inode_addr, &file.inode);
                }
                file.inode.singly_indirect_block_pointers
            })?;
            // let singly_indirect = {
            //     let off = unsafe {
            //         &file.inode.singly_indirect_block_pointers as *const _ as u32
            //             - &file.inode as *const _ as u32
            //     };
            //     alloc_on_inode(
            //         unsafe { &mut file.inode.singly_indirect_block_pointers },
            //         off,
            //     )?
            // };
            let pointer: Block = self.alloc_pointer(
                self.to_addr(singly_indirect) + off * size_of::<Block>() as u32,
                alloc,
            )?;
            // dbg!(pointer);

            return Ok(self.to_addr(pointer) + offset % self.block_size);
        }

        /* DOUBLY INDIRECT ADDRESSING */
        offset_start = offset_end;
        offset_end += (blocknumber_per_block * blocknumber_per_block) as u32;
        if block_off >= offset_start && block_off < offset_end {
            // dbg!("doubly indirect addressing");
            let off = (block_off - offset_start) / blocknumber_per_block as u32;
            let doubly_indirect = err_if_zero({
                if alloc && unsafe { file.inode.doubly_indirect_block_pointers == Block(0) } {
                    file.inode.doubly_indirect_block_pointers =
                        self.alloc_block().ok_or(IoError::NoSpaceLeftOnDevice)?;
                    self.disk.write_struct(file.inode_addr, &file.inode);
                }
                file.inode.doubly_indirect_block_pointers
            })?;
            let pointer_to_pointer: Block = self.alloc_pointer(
                self.to_addr(doubly_indirect) + off * size_of::<Block>() as u32,
                alloc,
            )?;
            let off = (block_off - offset_start) % blocknumber_per_block as u32;
            let pointer: Block = self.alloc_pointer(
                self.to_addr(pointer_to_pointer) + off * size_of::<Block>() as u32,
                alloc,
            )?;

            return Ok(self.to_addr(pointer) + offset % self.block_size);
        }

        // Triply Indirect Addressing
        offset_start = offset_end;
        offset_end +=
            (blocknumber_per_block * blocknumber_per_block * blocknumber_per_block) as u32;
        if block_off >= offset_start && block_off < offset_end {
            // dbg!("triply indirect addressing");
            let off =
                (block_off - offset_start) / (blocknumber_per_block * blocknumber_per_block) as u32;

            let tripply_indirect = err_if_zero({
                if alloc && unsafe { file.inode.triply_indirect_block_pointers == Block(0) } {
                    file.inode.triply_indirect_block_pointers =
                        self.alloc_block().ok_or(IoError::NoSpaceLeftOnDevice)?;
                    self.disk.write_struct(file.inode_addr, &file.inode);
                }
                file.inode.triply_indirect_block_pointers
            })?;
            let pointer_to_pointer_to_pointer: Block = self.alloc_pointer(
                self.to_addr(tripply_indirect) + off * size_of::<Block>() as u32,
                alloc,
            )?;

            let off = (((block_off - offset_start)
                % (blocknumber_per_block * blocknumber_per_block) as u32)
                / blocknumber_per_block as u32) as u32;
            let pointer_to_pointer: Block = self.alloc_pointer(
                self.to_addr(pointer_to_pointer_to_pointer) + off * size_of::<Block>() as u32,
                alloc,
            )?;

            let off = (((block_off - offset_start)
                % (blocknumber_per_block * blocknumber_per_block) as u32)
                % blocknumber_per_block as u32) as u32;
            let pointer: Block = self.alloc_pointer(
                self.to_addr(pointer_to_pointer) + off * size_of::<Block>() as u32,
                alloc,
            )?;

            return Ok(self.to_addr(pointer) + offset % self.block_size);
        }
        panic!("out of file bound");
    }

    pub fn read(&mut self, file: &mut File, buf: &mut [u8]) -> IoResult<usize> {
        // TODO: do indirect
        // for i in 0..self.nbr_block_grp {
        //     dbg!(self.get_block_grp_descriptor(i));
        // }
        let file_curr_offset_start = file.curr_offset;
        if file.curr_offset > file.inode.low_size {
            return Err(IoError::FileOffsetOutOfFile);
        }
        if file.curr_offset == file.inode.low_size {
            return Ok(0);
        }

        let data_address = self.inode_data(file).unwrap();
        let offset = min(
            (file.inode.low_size - file.curr_offset) as usize,
            min(
                (self.block_size - file.curr_offset % self.block_size) as usize,
                buf.len(),
            ),
        );
        let data_read = self
            .disk
            .disk_read_buffer(data_address, &mut buf[0..offset]);
        file.curr_offset += data_read as u32;
        if data_read < offset {
            return Ok((file.curr_offset - file_curr_offset_start) as usize);
        }

        for chunk in buf[offset..].chunks_mut(self.block_size as usize) {
            let data_address = self.inode_data(file).unwrap();
            let offset = min(
                (file.inode.low_size - file.curr_offset) as usize,
                chunk.len(),
            );
            let data_read = self
                .disk
                .disk_read_buffer(data_address, &mut chunk[0..offset]);
            file.curr_offset += data_read as u32;
            if data_read < chunk.len() {
                return Ok((file.curr_offset - file_curr_offset_start) as usize);
            }
        }
        Ok((file.curr_offset - file_curr_offset_start) as usize)
    }

    pub fn write(&mut self, file: &mut File, buf: &[u8]) -> IoResult<usize> {
        // for i in 0..self.nbr_block_grp {
        //     dbg!(self.get_block_grp_descriptor(i));
        // }
        let file_curr_offset_start = file.curr_offset;
        if file.curr_offset > file.inode.low_size {
            return Err(IoError::FileOffsetOutOfFile);
        }

        let data_address = self.inode_data_alloc(file, true)?;
        let offset = min(
            (self.block_size - file.curr_offset % self.block_size) as usize,
            buf.len(),
        );
        let data_write = self.disk.disk_write_buffer(data_address, &buf[0..offset]);
        file.curr_offset += data_write as u32;
        if file.inode.low_size < file.curr_offset {
            file.inode.low_size = file.curr_offset;
            file.inode.nbr_disk_sectors = div_rounded_up(file.inode.low_size, 512);
            self.disk.write_struct(file.inode_addr, &file.inode);
        }
        if data_write < offset {
            return Ok((file.curr_offset - file_curr_offset_start) as usize);
        }

        for chunk in buf[offset..].chunks(self.block_size as usize) {
            let data_address = self.inode_data_alloc(file, true)?;
            let data_write = self.disk.disk_write_buffer(data_address, &chunk);
            file.curr_offset += data_write as u32;
            if file.inode.low_size < file.curr_offset {
                file.inode.low_size = file.curr_offset;
                file.inode.nbr_disk_sectors = div_rounded_up(file.inode.low_size, 512);
                self.disk.write_struct(file.inode_addr, &file.inode);
            }
            if data_write < chunk.len() {
                return Ok((file.curr_offset - file_curr_offset_start) as usize);
            }
        }
        Ok((file.curr_offset - file_curr_offset_start) as usize)
    }
}
