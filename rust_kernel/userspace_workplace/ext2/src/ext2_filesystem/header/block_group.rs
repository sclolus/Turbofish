//! This file describe the block group descriptor model

use super::Block;

/// Common structure of a block groupe
#[derive(Debug, Copy, Clone)]
#[repr(packed)]
pub struct BlockGroupDescriptor {
    /// Block address of block usage bitmap
    /*0 	3 	4*/
    block_usage_bitmap: Block,
    /// Block address of inode usage bitmap
    /*4 	7 	4*/
    inode_usage_bitmap: Block,
    /// Starting block address of inode table
    /*8 	11 	4*/
    inode_table: Block,
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

impl BlockGroupDescriptor {
    /// Get the starting block of a inode table
    pub fn get_inode_table_address(&self) -> Block {
        self.inode_table
    }

    /// Get the number of unallocate blocks
    pub fn get_nbr_unallocated_blocks(&self) -> u16 {
        self.nbr_unallocated_blocks
    }

    /// Get the block address of the block bitmap usage
    pub fn get_block_usage_bitmap_address(&self) -> Block {
        self.block_usage_bitmap
    }

    /// Mark a new block as allocated
    pub fn allocate_block(&mut self) {
        self.nbr_unallocated_blocks -= 1;
    }
}
