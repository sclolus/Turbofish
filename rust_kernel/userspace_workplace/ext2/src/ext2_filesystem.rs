//! this module contains a ext2 driver
//! see [osdev](https://wiki.osdev.org/Ext2)

#![allow(dead_code)]

mod disk;
mod tools;
use disk::ReaderDisk;
pub use tools::{div_rounded_up, err_if_zero, Block, IoError, IoResult};

use bitflags::bitflags;
mod header;
use header::{BlockGroupDescriptor, SuperBlock};

mod body;
use body::{DirectoryEntry, DirectoryEntryType, Inode, TypeAndPerm};

use bit_field::BitArray;

use core::cmp::min;
use core::mem::size_of;

use std::fs::File as StdFile;
use std::io::SeekFrom;

/// Used to help confirm the presence of Ext2 on a volume
const EXT2_SIGNATURE_MAGIC: u16 = 0xef53;

/// Magic iterator over the entire fileSytem
pub struct EntryIter<'a> {
    filesystem: &'a mut Ext2Filesystem,
    inode: (Inode, u32),
    curr_offset: u32,
    cur_dir_index: u16,
}

impl<'a> Iterator for EntryIter<'a> {
    type Item = (DirectoryEntry, u32);
    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(d) = self
            .filesystem
            .find_entry((&mut self.inode.0, self.inode.1), self.curr_offset)
        {
            self.cur_dir_index += 1;
            let curr_offset = self.curr_offset;
            dbg!(curr_offset);
            dbg!(d.entry_size);
            self.curr_offset += d.entry_size as u32;
            if d.inode == 0 {
                self.next()
            } else {
                Some((d, curr_offset))
            }
        } else {
            None
        }
    }
}

/// Local file structure
#[derive(Debug, Copy, Clone)]
pub struct File {
    inode_nbr: u32,
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

bitflags! {
    pub struct OpenFlags : u32 {
        const Append = 1 << 0;
        const ReadOnly = 1 << 1;
        const ReadWrite = 1 << 2;
        const Creat = 1 << 3;
    }
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
    pub fn open(&mut self, path: &str, flags: OpenFlags) -> IoResult<File> {
        let mut inode_nbr = 2;
        let mut inode = self.find_inode(2);
        let mut iter_path = path.split('/').peekable();
        while let Some(p) = iter_path.next() {
            dbg!(p);
            let entry = dbg!(self
                .iter_entries(inode)?
                .find(|(x, _)| unsafe { dbg!(x.get_filename()) } == p)
                .ok_or(IoError::NoSuchFileOrDirectory));
            // dbg!(entry?.0.get_filename());
            if entry.is_err() && iter_path.peek().is_none() && flags.contains(OpenFlags::Creat) {
                inode_nbr = dbg!(self.create_file(p, inode, flags)?);
            } else {
                inode_nbr = entry?.0.inode;
                inode = dbg!(self.find_inode(entry?.0.inode));
            }
        }
        Ok(File {
            inode_nbr,
            curr_offset: 0,
        })
    }

    pub fn to_addr(&self, block_number: Block) -> u32 {
        self.block_size * block_number.0
    }

    pub fn to_block(&self, size: u32) -> Block {
        Block(size / self.block_size + ((size % self.block_size != 0) as u32))
    }

    /// get inode nbr inode and return the Inode and it's address
    pub fn find_inode(&mut self, inode: u32) -> (Inode, u32) {
        println!("ENTERING_FIND_INODE {}", inode);
        assert!(inode >= 1);
        let block_grp = (inode - 1) / self.superblock.inodes_per_block_grp;
        let index = (inode - 1) % self.superblock.inodes_per_block_grp;
        let inode_offset = index * self.superblock.get_size_inode() as u32;

        let (block_grp_descriptor, _) = self.get_block_grp_descriptor(block_grp);
        // dbg!(block_grp_descriptor);
        let inode_addr = self.to_addr(block_grp_descriptor.inode_table) + inode_offset;
        // dbg!(inode_addr);

        (self.disk.read_struct(inode_addr), inode_addr)
    }

    /// try to allocate a new inode on block group n and return the inode number
    fn alloc_inode_on_grp(&mut self, n: u32) -> Option<u32> {
        let (mut block_dtr, block_dtr_addr) = self.get_block_grp_descriptor(n);
        if block_dtr.nbr_unallocated_inodes == 0 {
            return None;
        }
        // TODO: dynamic alloc ?
        let bitmap_addr = self.to_addr(block_dtr.inode_usage_bitmap);
        let mut bitmap: [u8; 1024] = self.disk.read_struct(bitmap_addr);
        for i in 0..self.superblock.inodes_per_block_grp {
            if !bitmap.get_bit(i as usize) {
                bitmap.set_bit(i as usize, true);
                self.disk
                    .write_struct(bitmap_addr + i / 8, &bitmap[(i / 8) as usize]);
                block_dtr.nbr_unallocated_inodes -= 1;
                self.disk.write_struct(block_dtr_addr, &block_dtr);
                // TODO: Check the + 1
                return Some(self.superblock.inodes_per_block_grp * n + i + 1);
            }
        }
        None
    }

    /// try to allocate a new inode anywhere on the filesystem and return the inode number
    fn alloc_inode(&mut self) -> Option<u32> {
        for n in 0..self.nbr_block_grp {
            if let Some(n) = self.alloc_inode_on_grp(n) {
                return Some(n);
            }
        }
        None
    }

    fn create_file(
        &mut self,
        filename: &str,
        (mut inode, inode_addr): (Inode, u32),
        flags: OpenFlags,
    ) -> IoResult<u32> {
        let inode_nbr = dbg!(self.alloc_inode().ok_or(IoError::NoSpaceLeftOnDevice)?);
        let (mut entry, offset) = self
            .iter_entries((inode, inode_addr))?
            .last()
            .expect("directory contains no entries");
        let entry_addr = self.inode_data((&mut inode, inode_addr), offset).unwrap();

        assert_eq!(self.disk.read_struct::<DirectoryEntry>(entry_addr), entry);

        dbg!(entry.entry_size);
        let (new_directory_entry_addr, new_offset) =
        // if we do not cross a Block
            if self.to_block(offset + entry.entry_size as u32) == self.to_block(offset + entry.entry_size as u32 + size_of::<DirectoryEntry>() as u32)
        // and the block is already allocated
            && self.to_block(offset + entry.entry_size as u32) == self.to_block(offset)
        {
            let offset = offset + entry.entry_size as u32;
            (
                dbg!(self
                    .inode_data((&mut inode, inode_addr), offset)
                    .unwrap()),
                offset,
            )
        } else {
            let offset =
                dbg!(self.to_addr(self.to_block(offset + entry.entry_size as u32)));
                (self.inode_data_alloc((&mut inode, inode_addr), offset)?, offset)
        };
        dbg!((entry_addr, offset));
        dbg!((new_directory_entry_addr, new_offset));
        let new_directory_entry =
            DirectoryEntry::new(filename, DirectoryEntryType::RegularFile, inode_nbr)?;
        //TODO: Change entry.entry_size to the new offset
        // entry.entry_size = dbg!(new_directory_entry_addr - entry_addr) as u16;
        // self.disk.write_struct(entry_addr, &entry);

        inode.low_size = dbg!(new_offset + new_directory_entry.entry_size as u32);
        self.disk.write_struct(inode_addr, &inode);
        // panic!("au secour");
        self.disk
            .write_struct(new_directory_entry_addr, &new_directory_entry);
        let (_, inode_addr) = self.find_inode(inode_nbr);
        let inode = Inode::new(TypeAndPerm::from_bits_truncate(0o644) | TypeAndPerm::REGULAR_FILE);
        self.disk.write_struct(inode_addr, &inode);
        Ok(inode_nbr)
    }

    /// find the directory entry a offset file.curr_offset
    pub fn find_entry(
        &mut self,
        inode: (&mut Inode, u32),
        offset: u32,
    ) -> IoResult<DirectoryEntry> {
        if offset >= inode.0.low_size {
            dbg!("none");
            return Err(IoError::EndOfFile);
        }
        let base_addr = dbg!(self.inode_data(inode, offset))? as u32;
        let dir_header: DirectoryEntry = self.disk.read_struct(base_addr);
        Ok(dir_header)
    }

    pub fn iter_entries<'a>(
        &'a mut self,
        (inode, inode_addr): (Inode, u32),
    ) -> IoResult<EntryIter<'a>> {
        if unsafe { !inode.type_and_perm.contains(TypeAndPerm::DIRECTORY) } {
            return Err(IoError::NotADirectory);
        }
        Ok(EntryIter {
            filesystem: self,
            inode: (inode, inode_addr),
            curr_offset: 0,
            cur_dir_index: 0,
        })
    }

    /// read the block group descriptor address from the block group number starting at 0
    pub fn block_grp_descriptor_addr(&mut self, n: u32) -> u32 {
        // The table is located in the block immediately following the Superblock. So if the block size (determined from a field in the superblock) is 1024 bytes per block, the Block Group Descriptor Table will begin at block 2. For any other block size, it will begin at block 1. Remember that blocks are numbered starting at 0, and that block numbers don't usually correspond to physical block addresses.
        assert!(n <= self.nbr_block_grp);
        let offset = if self.block_size == 1024 { 2 } else { 1 };

        self.to_addr(Block(offset)) + n * size_of::<BlockGroupDescriptor>() as u32
    }

    /// read the block group descriptor from the block group number starting at 0
    pub fn get_block_grp_descriptor(&mut self, n: u32) -> (BlockGroupDescriptor, u32) {
        let block_grp_addr = self.block_grp_descriptor_addr(n);
        let block_grp: BlockGroupDescriptor = self.disk.read_struct(block_grp_addr);
        (block_grp, block_grp_addr)
    }

    /// try to allocate a new block on block grp number `n`
    fn alloc_block_on_grp(&mut self, n: u32) -> Option<Block> {
        let (mut block_dtr, block_dtr_addr) = self.get_block_grp_descriptor(n);
        if block_dtr.nbr_unallocated_blocks == 0 {
            return None;
        }
        // TODO: dynamic alloc ?
        let bitmap_addr = self.to_addr(block_dtr.block_usage_bitmap);
        let mut bitmap: [u8; 1024] = self.disk.read_struct(bitmap_addr);
        for i in 0..self.superblock.get_block_per_block_grp().0 {
            if !bitmap.get_bit(i as usize) {
                bitmap.set_bit(i as usize, true);
                self.disk
                    .write_struct(bitmap_addr + i / 8, &bitmap[(i / 8) as usize]);
                block_dtr.nbr_unallocated_blocks -= 1;
                self.disk.write_struct(block_dtr_addr, &block_dtr);
                // TODO: Check the + 1
                return Some(self.superblock.get_block_per_block_grp() * n + Block(i + 1));
            }
        }
        None
    }

    /// try to allocate a new block anywhere on the filesystem
    fn alloc_block(&mut self) -> Option<Block> {
        for n in 0..self.nbr_block_grp {
            if let Some(addr) = self.alloc_block_on_grp(n) {
                return Some(addr);
            }
        }
        None
    }

    /// get the data of an inode at the offset file.curr_offset
    fn inode_data(&mut self, inode: (&mut Inode, u32), offset: u32) -> Result<u32, IoError> {
        self.inode_data_may_alloc(inode, offset, false)
    }
    fn inode_data_alloc(&mut self, inode: (&mut Inode, u32), offset: u32) -> Result<u32, IoError> {
        self.inode_data_may_alloc(inode, offset, true)
    }

    /// alloc a pointer (used by the function inode_data_alloc)
    fn alloc_pointer(&mut self, pointer_addr: u32, alloc: bool) -> Result<Block, IoError> {
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
    fn inode_data_may_alloc(
        &mut self,
        (inode, inode_addr): (&mut Inode, u32),
        offset: u32,
        alloc: bool,
    ) -> Result<u32, IoError> {
        let block_off = offset / self.block_size;
        let blocknumber_per_block = self.block_size as usize / size_of::<Block>();

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
            if alloc && unsafe { inode.direct_block_pointers[block_off as usize] == Block(0) } {
                inode.direct_block_pointers[block_off as usize] =
                    self.alloc_block().ok_or(IoError::NoSpaceLeftOnDevice)?;
                self.disk.write_struct(inode_addr, inode);
                dbg!(block_off);
                dbg!(inode_addr);
                dbg!(inode.direct_block_pointers[block_off as usize]);
            }
            // dbg!(&inode);
            dbg!(block_off);
            dbg!(inode.direct_block_pointers[block_off as usize]);
            return Ok(self.to_addr(err_if_zero(
                inode.direct_block_pointers[block_off as usize],
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
                if alloc && unsafe { inode.singly_indirect_block_pointers == Block(0) } {
                    inode.singly_indirect_block_pointers =
                        self.alloc_block().ok_or(IoError::NoSpaceLeftOnDevice)?;
                    self.disk.write_struct(inode_addr, inode);
                }
                inode.singly_indirect_block_pointers
            })?;
            // let singly_indirect = {
            //     let off = unsafe {
            //         &inode.singly_indirect_block_pointers as *const _ as u32
            //             - &inode as *const _ as u32
            //     };
            //     alloc_on_inode(
            //         unsafe { &mut inode.singly_indirect_block_pointers },
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
                if alloc && unsafe { inode.doubly_indirect_block_pointers == Block(0) } {
                    inode.doubly_indirect_block_pointers =
                        self.alloc_block().ok_or(IoError::NoSpaceLeftOnDevice)?;
                    self.disk.write_struct(inode_addr, inode);
                }
                inode.doubly_indirect_block_pointers
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
                if alloc && unsafe { inode.triply_indirect_block_pointers == Block(0) } {
                    inode.triply_indirect_block_pointers =
                        self.alloc_block().ok_or(IoError::NoSpaceLeftOnDevice)?;
                    self.disk.write_struct(inode_addr, inode);
                }
                inode.triply_indirect_block_pointers
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
        // for i in 0..self.nbr_block_grp {
        //     dbg!(self.get_block_grp_descriptor(i));
        // }
        let (mut inode, inode_addr) = self.find_inode(file.inode_nbr);
        let file_curr_offset_start = file.curr_offset;
        if file.curr_offset > inode.low_size {
            return Err(IoError::FileOffsetOutOfFile);
        }
        if file.curr_offset == inode.low_size {
            return Ok(0);
        }

        let data_address = self
            .inode_data((&mut inode, inode_addr), file.curr_offset)
            .unwrap();
        let offset = min(
            (inode.low_size - file.curr_offset) as usize,
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
            let data_address = self
                .inode_data((&mut inode, inode_addr), file.curr_offset)
                .unwrap();
            let offset = min((inode.low_size - file.curr_offset) as usize, chunk.len());
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
        let (mut inode, inode_addr) = self.find_inode(file.inode_nbr);
        let file_curr_offset_start = file.curr_offset;
        if file.curr_offset > inode.low_size {
            return Err(IoError::FileOffsetOutOfFile);
        }

        let data_address = self.inode_data_alloc((&mut inode, inode_addr), file.curr_offset)?;
        let offset = min(
            (self.block_size - file.curr_offset % self.block_size) as usize,
            buf.len(),
        );
        let data_write = self.disk.disk_write_buffer(data_address, &buf[0..offset]);
        file.curr_offset += data_write as u32;
        if inode.low_size < file.curr_offset {
            inode.low_size = file.curr_offset;
            inode.nbr_disk_sectors = div_rounded_up(inode.low_size, 512);
            self.disk.write_struct(inode_addr, &inode);
        }
        if data_write < offset {
            return Ok((file.curr_offset - file_curr_offset_start) as usize);
        }

        for chunk in buf[offset..].chunks(self.block_size as usize) {
            let data_address = self.inode_data_alloc((&mut inode, inode_addr), file.curr_offset)?;
            let data_write = self.disk.disk_write_buffer(data_address, &chunk);
            file.curr_offset += data_write as u32;
            if inode.low_size < file.curr_offset {
                inode.low_size = file.curr_offset;
                inode.nbr_disk_sectors = div_rounded_up(inode.low_size, 512);
                self.disk.write_struct(inode_addr, &inode);
            }
            if data_write < chunk.len() {
                return Ok((file.curr_offset - file_curr_offset_start) as usize);
            }
        }
        Ok((file.curr_offset - file_curr_offset_start) as usize)
    }
}
