//! this module contains a ext2 driver
//! see [osdev](https://wiki.osdev.org/Ext2)

#![cfg_attr(all(not(test), not(feature = "std-print")), no_std)]
// #![deny(missing_docs)]

mod disk;
use crate::disk::Disk;
pub use disk::DiskIo;

pub mod syscall;
use libc_binding::{Errno, FileType};

mod tools;
pub use tools::{
    align_next, align_prev, div_rounded_up, err_if_zero, u32_align_next, u32_align_prev, Block,
    IoResult,
};

use bit_field::BitField;
mod header;
use header::{BlockGroupDescriptor, SuperBlock};

mod body;
pub use body::{DirectoryEntry, DirectoryEntryType, Inode};

#[cfg(not(feature = "std-print"))]
#[allow(unused_imports)]
#[macro_use]
extern crate terminal;

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;
use bit_field::BitArray;

use core::mem::size_of;

/// Global structure of ext2Filesystem, such as disk partition.
#[derive(Debug)]
pub struct Ext2Filesystem {
    pub superblock: SuperBlock, // or remove pub and add get_superblock() method.
    superblock_addr: u64,
    disk: Disk,
    nbr_block_grp: u32,
    pub block_size: u32, // or remove pub and add get_block_size() method.
    cache: Cache<u64, Block>,
}

/// Used to help confirm the presence of Ext2 on a volume
const EXT2_SIGNATURE_MAGIC: u16 = 0xef53;

/// Magic iterator over the entire fileSytem
pub struct EntryIter<'a> {
    filesystem: &'a mut Ext2Filesystem,
    inode: (Inode, u64),
    curr_offset: u32,
}

impl<'a> Iterator for EntryIter<'a> {
    type Item = (DirectoryEntry, u32);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(d) = self
            .filesystem
            .find_entry((&mut self.inode.0, self.inode.1), self.curr_offset as u64)
        {
            let curr_offset = self.curr_offset;
            self.curr_offset += d.get_size() as u32;
            if d.get_inode() == 0 {
                self.next()
            } else {
                Some((d, curr_offset))
            }
        } else {
            None
        }
    }
}

/// The Kernel side FileDescriptor struct
#[derive(Debug, Copy, Clone)]
pub struct File {
    /// Inode number
    pub inode_nbr: u32,
    curr_offset: u64,
}

// impl File {
//     /// The future seek syscall ?
//     pub fn seek(&mut self, s: SeekFrom) {
//         match s {
//             SeekFrom::Start(n) => {
//                 self.curr_offset = n as u64;
//             }
//             _ => unimplemented!(),
//         }
//     }
// }

type OffsetDirEntry = u32;
type InodeAddr = u64;
type InodeNbr = u32;

impl Ext2Filesystem {
    /// Invocation of a new FileSystem instance: take a FD and his reader as parameter
    pub fn new(disk: Box<dyn DiskIo>) -> IoResult<Self> {
        let mut disk = Disk(disk);
        let superblock_addr = 1024;
        let superblock: SuperBlock = disk.read_struct(superblock_addr)?;

        let signature = superblock.get_ext2_signature();
        assert_eq!(signature, EXT2_SIGNATURE_MAGIC);

        // consistency check
        let nbr_block_grp = superblock.get_nbr_block_grp();
        assert_eq!(nbr_block_grp, superblock.get_inode_block_grp());

        let block_size = 1024 << superblock.get_log2_block_size();

        Ok(Self {
            block_size,
            superblock,
            superblock_addr,
            nbr_block_grp,
            disk,
            cache: Cache::new(block_size as usize / size_of::<Block>()),
        })
    }

    /// go through all filesystem to find the Parent Inode and the entry of path
    pub fn find_path(
        &mut self,
        mut path: &str,
    ) -> IoResult<(InodeNbr, (DirectoryEntry, OffsetDirEntry))> {
        let mut inode_nbr = 2;
        let mut parent_inode_nbr = 2;
        let mut entry = unsafe { core::mem::uninitialized() };
        path = path.trim();
        if path == "/" {
            path = "/.";
        }
        for p in path.split('/').filter(|x| x != &"") {
            parent_inode_nbr = inode_nbr;
            entry = self
                .iter_entries(inode_nbr)?
                .find(|(x, _)| unsafe { x.get_filename() } == p)
                .ok_or(Errno::ENOENT)?;

            inode_nbr = entry.0.get_inode();
        }
        Ok((parent_inode_nbr, entry))
    }

    pub fn find_entry_in_inode(
        &mut self,
        inode_nbr: u32,
        filename: &str,
    ) -> IoResult<(DirectoryEntry, OffsetDirEntry)> {
        Ok(self
            .iter_entries(inode_nbr)?
            .find(|(x, _)| unsafe { x.get_filename() } == filename)
            .ok_or(Errno::ENOENT)?)
    }

    /// go through all filesystem to find the Inode corresponding to path
    pub fn find_inode(&mut self, path: &str) -> IoResult<(Inode, InodeAddr)> {
        let (_parent_inode_nbr, entry) = self.find_path(path)?;
        let inode_nbr = entry.0.get_inode();
        self.get_inode(inode_nbr)
    }

    /// truncate inode to the size `new_size` deleting all data blocks above
    pub fn truncate_inode(
        &mut self,
        (inode, inode_addr): (&mut Inode, InodeAddr),
        new_size: u64,
    ) -> IoResult<()> {
        let size = inode.get_size();
        assert!(new_size <= size);
        if size == 0 {
            return Ok(());
        }
        let new_size_block = self.to_block_addr(new_size);
        // let curr_size = self.to_block_addr(align_next(size, self.block_size as u64));
        // size - 1 to get the previous block addr
        let curr_size = self.to_block_addr(size - 1);

        for block_off in (new_size_block.0..=curr_size.0).rev() {
            self.inode_free_block((inode, inode_addr), Block(block_off))
                .unwrap();
        }
        inode.update_size(new_size, self.block_size);
        self.disk.write_struct(inode_addr, inode)?;
        Ok(())
    }

    /// delete inode `inode_nbr`
    pub fn free_inode(
        &mut self,
        (inode, inode_addr): (&mut Inode, InodeAddr),
        inode_nbr: u32,
    ) -> IoResult<()> {
        assert!(inode_nbr >= 1);

        /* Delete Data Blocks */
        // don't truncate inode if it is a fast symbolic link
        if !(inode.type_and_perm.is_symlink()
            && inode.get_size() <= Inode::FAST_SYMLINK_SIZE_MAX as u64)
        {
            self.truncate_inode((inode, inode_addr), 0).unwrap();
        }

        /* Unset Inode bitmap */
        let block_grp = (inode_nbr - 1) / self.superblock.inodes_per_block_grp;
        let index = (inode_nbr as u64 - 1) % self.superblock.inodes_per_block_grp as u64;
        let (mut block_dtr, block_dtr_addr) = self.get_block_grp_descriptor(block_grp)?;
        let bitmap_addr = self.to_addr(block_dtr.inode_usage_bitmap);
        let mut bitmap: u8 = self.disk.read_struct(bitmap_addr + index / 8)?;
        assert!(bitmap.get_bit((index % 8) as usize));
        bitmap.set_bit((index % 8) as usize, false);
        self.disk.write_struct(bitmap_addr + index / 8, &bitmap)?;

        debug_assert!(self.get_inode(inode_nbr).is_err());
        // TODO: check that with fsck
        block_dtr.nbr_free_inodes += 1;
        self.superblock.nbr_free_inodes += 1;
        block_dtr.nbr_free_inodes;
        self.disk
            .write_struct(self.superblock_addr, &self.superblock)?;
        self.disk.write_struct(block_dtr_addr, &block_dtr)?;
        Ok(())
    }

    /// decrement link count of the inode and delete it if it becomes 0
    /// panic if the inode refers to a directory
    pub fn unlink_inode(&mut self, inode_nbr: u32, free_inode_data: bool) -> IoResult<()> {
        let (mut inode, inode_addr) = self.get_inode(inode_nbr)?;
        assert!(!inode.is_a_directory());
        if inode.nbr_hard_links <= 1 && free_inode_data {
            return self.free_inode((&mut inode, inode_addr), inode_nbr);
        }
        inode.nbr_hard_links -= 1;
        self.disk.write_struct(inode_addr, &inode)?;
        Ok(())
    }

    /// delete the entry at entry_off of the parent_inode nbr
    pub fn delete_entry(&mut self, parent_inode_nbr: u32, entry_off: u32) -> IoResult<()> {
        let (mut inode, inode_addr) = self.get_inode(parent_inode_nbr)?;
        let curr_offset = entry_off;
        let entry = self
            .find_entry((&mut inode, inode_addr), curr_offset as u64)
            .unwrap();

        let (mut previous, previous_offset) = self
            .iter_entries(parent_inode_nbr)
            .unwrap()
            .take_while(|(_, off)| *off < entry_off)
            .last()
            .unwrap();
        /* if it is the last entry */
        if self
            .find_entry(
                (&mut inode, inode_addr),
                curr_offset as u64 + entry.get_size() as u64,
            )
            .is_none()
        {
            self.set_as_last_entry((&mut inode, inode_addr), (&mut previous, previous_offset))
        }
        /* Else, we set previous next to current next.
        this creates a Hole which will be filled in push_entry */
        else {
            let next_entry_off = curr_offset as u64 + entry.get_size() as u64;
            let previous_entry_addr = self
                .inode_data_may_alloc((&mut inode, inode_addr), previous_offset as u64, false)
                .unwrap();
            previous.set_size((next_entry_off - previous_offset as u64) as u16);
            previous.write_on_disk(previous_entry_addr, &mut self.disk)?;
            Ok(())
        }
    }

    /// convert a block to an address
    pub fn to_addr(&self, block_number: Block) -> u64 {
        self.block_size as u64 * block_number.0 as u64
    }

    /// convert an address to a number of block
    pub fn to_block_addr(&self, size: u64) -> Block {
        Block((size / self.block_size as u64) as u32)
    }

    /// convert an address to a number of block
    pub fn to_block(&self, size: u64) -> Block {
        Block(
            (size / self.block_size as u64 + ((size % self.block_size as u64 != 0) as u64)) as u32,
        )
    }

    /// get inode nbr inode and return the Inode and it's address
    pub fn get_inode(&mut self, inode: u32) -> IoResult<(Inode, InodeAddr)> {
        assert!(inode >= 1);
        let block_grp = (inode - 1) / self.superblock.inodes_per_block_grp;
        let index = (inode as u64 - 1) % self.superblock.inodes_per_block_grp as u64;
        let inode_offset = index as u64 * self.superblock.get_size_inode() as u64;

        let (block_dtr, _) = self.get_block_grp_descriptor(block_grp)?;
        let bitmap_addr = self.to_addr(block_dtr.inode_usage_bitmap);
        let bitmap: u8 = self.disk.read_struct(bitmap_addr + index / 8)?;
        if !bitmap.get_bit((index % 8) as usize) {
            return Err(Errno::ENOENT);
        }

        let inode_addr = self.to_addr(block_dtr.inode_table) + inode_offset;

        Ok((self.disk.read_struct(inode_addr)?, inode_addr))
    }

    //TODO: better handle disk error
    /// try to allocate a new inode on block group n and return the inode number
    fn alloc_inode_on_grp(&mut self, n: u32) -> Option<InodeNbr> {
        let (mut block_dtr, block_dtr_addr) = self.get_block_grp_descriptor(n).ok()?;
        if block_dtr.nbr_free_inodes == 0 {
            return None;
        }

        // TODO: dynamic alloc ?
        let bitmap_addr = self.to_addr(block_dtr.inode_usage_bitmap);
        let mut bitmap: [u8; 1024] = self.disk.read_struct(bitmap_addr).ok()?;
        for i in 0..self.superblock.inodes_per_block_grp {
            if !bitmap.get_bit(i as usize) {
                bitmap.set_bit(i as usize, true);
                self.disk
                    .write_struct(bitmap_addr + i as u64 / 8, &bitmap[(i / 8) as usize])
                    .ok()?;
                block_dtr.nbr_free_inodes -= 1;
                self.superblock.nbr_free_inodes -= 1;
                block_dtr.nbr_free_inodes;
                self.disk
                    .write_struct(self.superblock_addr, &self.superblock)
                    .ok()?;
                self.disk.write_struct(block_dtr_addr, &block_dtr).ok()?;
                // TODO: Check the + 1
                return Some(self.superblock.inodes_per_block_grp * n + i + 1);
            }
        }
        None
    }

    /// try to allocate a new inode anywhere on the filesystem and return the inode number
    fn alloc_inode(&mut self) -> Option<InodeNbr> {
        for n in 0..self.nbr_block_grp {
            if let Some(n) = self.alloc_inode_on_grp(n) {
                return Some(n);
            }
        }
        None
    }

    /// the the entry at offset entry_offset the last entry of the directory
    pub fn set_as_last_entry(
        &mut self,
        (inode, inode_addr): (&mut Inode, InodeAddr),
        (entry, entry_offset): (&mut DirectoryEntry, OffsetDirEntry),
    ) -> IoResult<()> {
        let entry_addr = self.inode_data_alloc((inode, inode_addr), entry_offset as u64)?;

        // =(the offset to the next block)
        entry.set_size((u32_align_next(entry_offset + 1, self.block_size) - entry_offset) as u16);
        entry.write_on_disk(entry_addr, &mut self.disk)?;
        /* Update inode size */

        let new_size = entry_offset as u64 + entry.get_size() as u64;
        if new_size < inode.get_size() {
            self.truncate_inode((inode, inode_addr), new_size)?;
        } else {
            inode.update_size(new_size, self.block_size);
            self.disk.write_struct(inode_addr, inode)?;
        }
        Ok(())
    }

    /// push a directory entry on the Directory inode: `parent_inode_nbr`
    fn push_entry(
        &mut self,
        parent_inode_nbr: u32,
        new_entry: &mut DirectoryEntry,
    ) -> IoResult<()> {
        let (mut inode, inode_addr) = self.get_inode(parent_inode_nbr)?;
        // Get the last entry of the Directory
        match self.iter_entries(parent_inode_nbr)?.last() {
            Some((mut entry, offset)) => {
                let offset = offset as u64;

                let entry_addr = self
                    .inode_data_may_alloc((&mut inode, inode_addr), offset, false)
                    .unwrap();
                // debug_assert_eq!(self.disk.read_struct::<DirectoryEntry>(entry_addr), entry)?;
                let entry_size = entry.size() as u64;

                let new_offset = {
                    let new_offset = align_next(offset + entry_size, 4);
                    // if we do not cross a Block
                    if self.to_block(new_offset) == self.to_block(new_offset + new_entry.size() as u64)
                    // and the block is already allocated
                        && self.inode_data_may_alloc((&mut inode, inode_addr), new_offset, true).is_ok()
                    //self.to_block( as u32) == self.to_block(offset)
                    {
                        new_offset
                    } else {
                        align_next(offset + entry_size, self.block_size as u64)
                    }
                };
                /* Update previous entry size */
                entry.set_size((new_offset - offset) as u16);
                entry.write_on_disk(entry_addr, &mut self.disk)?;

                self.set_as_last_entry((&mut inode, inode_addr), (new_entry, new_offset as u32))
            }
            None => self.set_as_last_entry((&mut inode, inode_addr), (new_entry, 0 as u32)),
        }
    }

    /// find the directory entry a offset file.curr_offset
    pub fn find_entry(&mut self, inode: (&mut Inode, u64), offset: u64) -> Option<DirectoryEntry> {
        if offset >= inode.0.get_size() {
            return None;
        }
        let base_addr = self.inode_data_may_alloc(inode, offset, false).ok()? as u64;
        let dir_header: DirectoryEntry = self.disk.read_struct(base_addr).ok()?;
        Some(dir_header)
    }

    /// iter of the entries of inodes if inode is a directory
    pub fn iter_entries<'a>(&'a mut self, inode: InodeNbr) -> IoResult<EntryIter<'a>> {
        let (inode, inode_addr) = self.get_inode(inode)?;
        if !inode.type_and_perm.contains(FileType::DIRECTORY) {
            return Err(Errno::ENOTDIR);
        }
        Ok(EntryIter {
            filesystem: self,
            inode: (inode, inode_addr),
            curr_offset: 0,
        })
    }

    /// read the block group descriptor address from the block group number starting at 0
    pub fn block_grp_descriptor_addr(&self, n: u32) -> u64 {
        // The table is located in the block immediately following the
        // Superblock. So if the block size (determined from a field
        // in the superblock) is 1024 bytes per block, the Block Group
        // Descriptor Table will begin at block 2. For any other block
        // size, it will begin at block 1. Remember that blocks are
        // numbered starting at 0, and that block numbers don't
        // usually correspond to physical block addresses.
        assert!(n <= self.nbr_block_grp);
        let offset = if self.block_size == 1024 { 2 } else { 1 };

        self.to_addr(Block(offset)) + n as u64 * size_of::<BlockGroupDescriptor>() as u64
    }

    /// read the block group descriptor from the block group number starting at 0
    pub fn get_block_grp_descriptor(&mut self, n: u32) -> IoResult<(BlockGroupDescriptor, u64)> {
        let block_grp_addr = self.block_grp_descriptor_addr(n);
        let block_grp: BlockGroupDescriptor = self.disk.read_struct(block_grp_addr)?;
        Ok((block_grp, block_grp_addr))
    }

    /// try to allocate a new block on block grp number `n`
    fn alloc_block_on_grp(&mut self, n: u32) -> Option<Block> {
        let (mut block_dtr, block_dtr_addr) = self.get_block_grp_descriptor(n).ok()?;
        if block_dtr.nbr_free_blocks == 0 {
            return None;
        }
        // TODO: dynamic alloc ?
        let bitmap_addr = self.to_addr(block_dtr.block_usage_bitmap);
        let mut bitmap: [u8; 1024] = self.disk.read_struct(bitmap_addr).ok()?;
        for i in 0..self.superblock.get_block_per_block_grp().0 {
            if !bitmap.get_bit(i as usize) {
                bitmap.set_bit(i as usize, true);
                self.disk
                    .write_struct(bitmap_addr + i as u64 / 8, &bitmap[(i / 8) as usize])
                    .ok()?;

                block_dtr.nbr_free_blocks -= 1;
                self.disk.write_struct(block_dtr_addr, &block_dtr).ok()?;
                self.superblock.nbr_free_blocks -= 1;
                self.disk
                    .write_struct(self.superblock_addr, &self.superblock)
                    .ok()?;
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

    /// try to free the block block_nbr
    fn free_block(&mut self, block_nbr: Block) -> IoResult<()> {
        let block_grp = (block_nbr.0 - 1) / self.superblock.get_block_per_block_grp().0;
        let index = (block_nbr.0 as u64 - 1) % self.superblock.get_block_per_block_grp().0 as u64;

        let (mut block_dtr, block_dtr_addr) = self.get_block_grp_descriptor(block_grp)?;
        let bitmap_addr = self.to_addr(block_dtr.block_usage_bitmap);
        let mut bitmap: u8 = self.disk.read_struct(bitmap_addr + index / 8)?;
        assert!(bitmap.get_bit((index % 8) as usize));
        bitmap.set_bit((index % 8) as usize, false);
        self.disk.write_struct(bitmap_addr + index / 8, &bitmap)?;
        block_dtr.nbr_free_blocks += 1;
        self.disk.write_struct(block_dtr_addr, &block_dtr)?;
        self.superblock.nbr_free_blocks += 1;
        self.disk
            .write_struct(self.superblock_addr, &self.superblock)?;
        Ok(())
    }

    /// get the data of inode at offset `offset`, and allocate the data block if necessary
    fn inode_data_alloc(&mut self, inode: (&mut Inode, u64), offset: u64) -> IoResult<u64> {
        self.inode_data_may_alloc(inode, offset, true)
    }

    /// alloc a pointer (used by the function inode_data_alloc)
    fn alloc_pointer(&mut self, pointer_addr: u64, alloc: bool) -> IoResult<Block> {
        err_if_zero({
            //            println!("reading at {:#X?}", pointer_addr);
            let pointer = self.disk.read_struct(pointer_addr)?;
            if alloc && pointer == Block(0) {
                let new_block = self.alloc_block().ok_or(Errno::ENOSPC)?;
                self.disk.write_struct(pointer_addr, &new_block)?;
                new_block
            } else {
                pointer
            }
        })
    }

    /// free a pointer (used by the function inode_data_alloc)
    fn free_pointer(&mut self, pointer_addr: u64) -> IoResult<()> {
        let pointer = self.disk.read_struct(pointer_addr)?;
        if pointer == Block(0) {
            panic!("free pointer null");
        } else {
            self.disk.write_struct(pointer_addr, &Block(0))?;
            self.free_block(pointer)
        }
    }

    /// Get the file location at offset 'offset'
    fn inode_free_block(
        &mut self,
        (inode, inode_addr): (&mut Inode, InodeAddr),
        block_off: Block,
    ) -> IoResult<()> {
        let blocknumber_per_block = (self.block_size as usize / size_of::<Block>()) as u32;
        let block_off = block_off.0 as u64;

        /* SIMPLE ADDRESSING */
        let mut offset_start = 0;
        let mut offset_end = 12;
        if block_off >= offset_start && block_off < offset_end {
            let pointer = err_if_zero(inode.direct_block_pointers[block_off as usize])?;
            self.free_block(pointer)?;
            inode.direct_block_pointers[block_off as usize] = Block(0);
            self.disk.write_struct(inode_addr, inode)?;
            return Ok(());
        }

        /* SINGLY INDIRECT ADDRESSING */
        // 12 * blocksize .. 12 * blocksize + (blocksize / 4) * blocksize
        offset_start = offset_end;
        offset_end += blocknumber_per_block as u64;
        if block_off >= offset_start && block_off < offset_end {
            let off = (block_off - offset_start) as u64;
            let pointer = err_if_zero(inode.singly_indirect_block_pointers)?;

            self.free_pointer(self.to_addr(pointer) + off * size_of::<Block>() as u64)?;

            if block_off == offset_start {
                let pointer = err_if_zero(inode.singly_indirect_block_pointers)?;
                self.free_block(pointer)?;
                inode.singly_indirect_block_pointers = Block(0);
                self.disk.write_struct(inode_addr, inode)?;
            }
            return Ok(());
        }

        /* DOUBLY INDIRECT ADDRESSING */
        offset_start = offset_end;
        offset_end += (blocknumber_per_block * blocknumber_per_block) as u64;
        if block_off >= offset_start && block_off < offset_end {
            // dbg!("doubly indirect addressing");
            let doubly_indirect = err_if_zero(inode.doubly_indirect_block_pointers)?;

            let off_doubly = (block_off - offset_start) / blocknumber_per_block as u64;
            let addr_pointer_to_pointer =
                self.to_addr(doubly_indirect) + off_doubly * size_of::<Block>() as u64;

            let pointer_to_pointer: Block =
                err_if_zero(self.disk.read_struct(addr_pointer_to_pointer)?)?;
            let off = (block_off - offset_start) % blocknumber_per_block as u64;

            self.free_pointer(self.to_addr(pointer_to_pointer) + off * size_of::<Block>() as u64)?;

            if off == 0 {
                self.free_pointer(addr_pointer_to_pointer)?;
            }

            if block_off == offset_start {
                let pointer = err_if_zero(inode.doubly_indirect_block_pointers)?;
                self.free_block(pointer)?;
                inode.doubly_indirect_block_pointers = Block(0);
                self.disk.write_struct(inode_addr, inode)?;
            }
            return Ok(());
        }

        /* TRIPLY INDIRECT ADDRESSING */
        offset_start = offset_end;
        offset_end +=
            (blocknumber_per_block * blocknumber_per_block * blocknumber_per_block) as u64;
        if block_off >= offset_start && block_off < offset_end {
            // dbg!("triply indirect addressing");
            let off_triply =
                (block_off - offset_start) / (blocknumber_per_block * blocknumber_per_block) as u64;

            let tripply_indirect = err_if_zero(inode.triply_indirect_block_pointers)?;

            let addr_pointer_to_pointer_to_pointer =
                self.to_addr(tripply_indirect) + off_triply * size_of::<Block>() as u64;
            let pointer_to_pointer_to_pointer: Block =
                err_if_zero(self.disk.read_struct(addr_pointer_to_pointer_to_pointer)?)?;

            let off_doubly = (((block_off - offset_start)
                % (blocknumber_per_block * blocknumber_per_block) as u64)
                / blocknumber_per_block as u64) as u64;

            let addr_pointer_to_pointer = self.to_addr(pointer_to_pointer_to_pointer)
                + off_doubly * size_of::<Block>() as u64;

            let pointer_to_pointer: Block =
                err_if_zero(self.disk.read_struct(addr_pointer_to_pointer)?)?;

            let off = (((block_off - offset_start)
                % (blocknumber_per_block * blocknumber_per_block) as u64)
                % blocknumber_per_block as u64) as u64;

            self.free_pointer(self.to_addr(pointer_to_pointer) + off * size_of::<Block>() as u64)?;

            if off == 0 {
                self.free_pointer(addr_pointer_to_pointer)?;
            }

            if off_doubly == 0 {
                self.free_pointer(addr_pointer_to_pointer_to_pointer)?;
            }

            if block_off == offset_start {
                let pointer = err_if_zero(inode.triply_indirect_block_pointers)?;
                self.free_block(pointer)?;
                inode.triply_indirect_block_pointers = Block(0);
                self.disk.write_struct(inode_addr, inode)?;
            }
            return Ok(());
        }
        Err(Errno::EFBIG)
    }

    /// Get the file location at offset 'offset'
    fn inode_data_may_alloc(
        &mut self,
        (inode, inode_addr): (&mut Inode, InodeAddr),
        offset: u64,
        alloc: bool,
    ) -> IoResult<u64> {
        let block_off = offset / self.block_size as u64;
        let blocknumber_per_block = self.block_size as usize / size_of::<Block>();

        /*
         * SIMPLE ADDRESSING
         */
        let mut offset_start = 0;
        let mut offset_end = 12;
        if block_off >= offset_start && block_off < offset_end {
            if alloc && inode.direct_block_pointers[block_off as usize] == Block(0) {
                inode.direct_block_pointers[block_off as usize] =
                    self.alloc_block().ok_or(Errno::ENOSPC)?;
                self.disk.write_struct(inode_addr, inode)?;
            }
            return Ok(self.to_addr(err_if_zero(
                inode.direct_block_pointers[block_off as usize],
            )?) + offset % self.block_size as u64);
        }

        /*
         * SINGLY INDIRECT ADDRESSING
         * 12 * blocksize .. 12 * blocksize + (blocksize / 4) * blocksize
         */
        offset_start = offset_end;
        offset_end += blocknumber_per_block as u64;
        if block_off >= offset_start && block_off < offset_end {
            let off = block_off - offset_start;

            let singly_indirect = err_if_zero({
                if alloc && inode.singly_indirect_block_pointers == Block(0) {
                    inode.singly_indirect_block_pointers =
                        self.alloc_block().ok_or(Errno::ENOSPC)?;
                    self.disk.write_struct(inode_addr, inode)?;
                }
                inode.singly_indirect_block_pointers
            })?;

            let pointer: Block = self.alloc_pointer(
                self.to_addr(singly_indirect) + off * size_of::<Block>() as u64,
                alloc,
            )?;

            return Ok(self.to_addr(pointer) + offset % self.block_size as u64);
        }

        /*
         * DOUBLY INDIRECT ADDRESSING
         */
        offset_start = offset_end;
        offset_end += (blocknumber_per_block * blocknumber_per_block) as u64;
        if block_off >= offset_start && block_off < offset_end {
            let off = (block_off - offset_start) / blocknumber_per_block as u64;
            let doubly_indirect = err_if_zero({
                if alloc && inode.doubly_indirect_block_pointers == Block(0) {
                    inode.doubly_indirect_block_pointers =
                        self.alloc_block().ok_or(Errno::ENOSPC)?;
                    self.disk.write_struct(inode_addr, inode)?;
                }
                inode.doubly_indirect_block_pointers
            })?;
            let pointer_to_pointer: Block = self.alloc_pointer(
                self.to_addr(doubly_indirect) + off * size_of::<Block>() as u64,
                alloc,
            )?;
            let off = (block_off - offset_start) % blocknumber_per_block as u64;
            let pointer: Block = self.alloc_pointer(
                self.to_addr(pointer_to_pointer) + off * size_of::<Block>() as u64,
                alloc,
            )?;
            return Ok(self.to_addr(pointer) + offset % self.block_size as u64);
        }

        /*
         * TRIPLY INDIRECT ADDRESSING
         */
        offset_start = offset_end;
        offset_end +=
            (blocknumber_per_block * blocknumber_per_block * blocknumber_per_block) as u64;
        if block_off >= offset_start && block_off < offset_end {
            let off =
                (block_off - offset_start) / (blocknumber_per_block * blocknumber_per_block) as u64;

            let tripply_indirect = err_if_zero({
                if alloc && inode.triply_indirect_block_pointers == Block(0) {
                    inode.triply_indirect_block_pointers =
                        self.alloc_block().ok_or(Errno::ENOSPC)?;
                    self.disk.write_struct(inode_addr, inode)?;
                }
                inode.triply_indirect_block_pointers
            })?;
            let pointer_to_pointer_to_pointer: Block = self.alloc_pointer(
                self.to_addr(tripply_indirect) + off * size_of::<Block>() as u64,
                alloc,
            )?;

            let off = (((block_off - offset_start)
                % (blocknumber_per_block * blocknumber_per_block) as u64)
                / blocknumber_per_block as u64) as u64;
            let pointer_to_pointer: Block = self.alloc_pointer(
                self.to_addr(pointer_to_pointer_to_pointer) + off * size_of::<Block>() as u64,
                alloc,
            )?;

            let off = (((block_off - offset_start)
                % (blocknumber_per_block * blocknumber_per_block) as u64)
                % blocknumber_per_block as u64) as u64;
            let pointer: Block = self.alloc_pointer(
                self.to_addr(pointer_to_pointer) + off * size_of::<Block>() as u64,
                alloc,
            )?;

            return Ok(self.to_addr(pointer) + offset % self.block_size as u64);
        }
        Err(Errno::EFBIG)
    }

    /// Get the file location at offset 'offset'
    /// Return which block store the file data at offset T
    /// Simple Read
    fn inode_data(&mut self, inode: &Inode, offset: u64) -> IoResult<u64> {
        let block_off = offset / self.block_size as u64;
        let blocknumber_per_block = self.block_size as usize / size_of::<Block>();

        /*
         * SIMPLE ADDRESSING
         */
        let mut offset_start = 0;
        let mut offset_end = 12;
        if block_off >= offset_start && block_off < offset_end {
            return Ok(self.to_addr(err_if_zero(
                inode.direct_block_pointers[block_off as usize],
            )?) + offset % self.block_size as u64);
        }

        /*
         * SINGLY INDIRECT ADDRESSING
         * 12 * blocksize .. 12 * blocksize + (blocksize / 4) * blocksize
         */
        offset_start = offset_end;
        offset_end += blocknumber_per_block as u64;
        if block_off >= offset_start && block_off < offset_end {
            let off = block_off - offset_start;
            let singly_indirect = err_if_zero({ inode.singly_indirect_block_pointers })?;

            let addr = self.to_addr(singly_indirect);
            let pointer = self.get_pointer(addr, off, Level::L1)?;
            return Ok(self.to_addr(pointer) + offset % self.block_size as u64);
        }

        /*
         * DOUBLY INDIRECT ADDRESSING
         */
        offset_start = offset_end;
        offset_end += (blocknumber_per_block * blocknumber_per_block) as u64;
        if block_off >= offset_start && block_off < offset_end {
            let off = (block_off - offset_start) / blocknumber_per_block as u64;
            let doubly_indirect = err_if_zero({ inode.doubly_indirect_block_pointers })?;

            let addr = self.to_addr(doubly_indirect);
            let pointer_to_pointer = self.get_pointer(addr, off, Level::L1)?;

            let off = (block_off - offset_start) % blocknumber_per_block as u64;

            let addr = self.to_addr(pointer_to_pointer);
            let pointer = self.get_pointer(addr, off, Level::L2)?;

            return Ok(self.to_addr(pointer) + offset % self.block_size as u64);
        }

        /*
         * TRIPLY INDIRECT ADDRESSING
         */
        offset_start = offset_end;
        offset_end +=
            (blocknumber_per_block * blocknumber_per_block * blocknumber_per_block) as u64;
        if block_off >= offset_start && block_off < offset_end {
            let off =
                (block_off - offset_start) / (blocknumber_per_block * blocknumber_per_block) as u64;
            let tripply_indirect = err_if_zero({ inode.triply_indirect_block_pointers })?;

            let addr = self.to_addr(tripply_indirect);
            let pointer_to_pointer_to_pointer = self.get_pointer(addr, off, Level::L1)?;

            let off = (((block_off - offset_start)
                % (blocknumber_per_block * blocknumber_per_block) as u64)
                / blocknumber_per_block as u64) as u64;

            let addr = self.to_addr(pointer_to_pointer_to_pointer);
            let pointer_to_pointer = self.get_pointer(addr, off, Level::L2)?;

            let off = (((block_off - offset_start)
                % (blocknumber_per_block * blocknumber_per_block) as u64)
                % blocknumber_per_block as u64) as u64;

            let addr = self.to_addr(pointer_to_pointer);
            let pointer = self.get_pointer(addr, off, Level::L3)?;

            return Ok(self.to_addr(pointer) + offset % self.block_size as u64);
        }
        Err(Errno::EFBIG)
    }

    /// Get a inode pointer
    #[inline(always)]
    fn get_pointer(&mut self, addr: u64, off: u64, level: Level) -> IoResult<Block> {
        Ok(*match self.cache.get(addr, off as usize, level) {
            Some(p) => p,
            None => {
                let v = self.cache.update_layer(addr, level);
                unsafe {
                    self.disk.read_buffer(
                        addr,
                        core::slice::from_raw_parts_mut(
                            v.as_mut_ptr() as *mut u8,
                            self.block_size as usize,
                        ),
                    )?
                };
                self.cache
                    .get(addr, off as usize, level)
                    .expect("Must be founded !")
            }
        })
    }
}

const NB_LAYERS: usize = 3;

/// Multi layer cache
#[derive(Debug)]
struct Cache<K, T> {
    entries: [CacheEntry<K, T>; NB_LAYERS],
}

impl<K: Eq + PartialEq + Copy, T: Clone + Default> Cache<K, T> {
    /// Create a new multi layer cache
    fn new(nb_elems: usize) -> Self {
        let mut entries: [CacheEntry<K, T>; NB_LAYERS] = unsafe { core::mem::uninitialized() };
        for entry in entries.iter_mut() {
            *entry = CacheEntry::new(nb_elems);
        }
        Self { entries }
    }

    /// Invalidate all the layers
    fn invalidate(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.invalidate();
        }
    }

    /// Try to get a layer borrowed data
    fn get(&self, idx: K, offset: usize, level: Level) -> Option<&T> {
        self.entries[get_index(level)].get(idx).map(|v| &v[offset])
    }

    /// Update a layer and get a mutable reference to his content
    fn update_layer(&mut self, idx: K, level: Level) -> &mut Vec<T> {
        self.entries[get_index(level)].update_layer(idx)
    }
}

fn get_index(level: Level) -> usize {
    use Level::*;
    match level {
        L1 => NB_LAYERS - 3,
        L2 => NB_LAYERS - 2,
        L3 => NB_LAYERS - 1,
    }
}

#[derive(Debug, Copy, Clone)]
enum Level {
    L1,
    L2,
    L3,
}

#[derive(Debug)]
struct CacheEntry<K, T> {
    idx: Option<K>,
    data: Vec<T>,
}

impl<K: Eq + PartialEq + Copy, T: Clone + Default> CacheEntry<K, T> {
    fn new(nb_elems: usize) -> Self {
        Self {
            idx: None,
            data: vec![Default::default(); nb_elems],
        }
    }

    fn invalidate(&mut self) {
        self.idx = None;
    }

    fn get(&self, idx: K) -> Option<&Vec<T>> {
        if let Some(stored_idx) = self.idx {
            if stored_idx == idx {
                return Some(&self.data);
            }
        }
        None
    }

    fn update_layer(&mut self, idx: K) -> &mut Vec<T> {
        self.idx = Some(idx);
        &mut self.data
    }
}
