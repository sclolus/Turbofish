//! This file describe the block group descriptor model

use super::Block;

/// Common structure of a block groupe
#[derive(Debug, Copy, Clone)]
#[repr(packed)]
pub struct BlockGroupDescriptor {
    /// Block address of block usage bitmap
    /*0 	3 	4*/
    pub block_usage_bitmap: Block,
    /// Block address of inode usage bitmap
    /*4 	7 	4*/
    pub inode_usage_bitmap: Block,
    /// Starting block address of inode table
    /*8 	11 	4*/
    pub inode_table: Block,
    /// Number of unallocated blocks in group
    /*12 	13 	2*/
    pub nbr_unallocated_blocks: u16,
    /// Number of unallocated inodes in group
    /*14 	15 	2*/
    pub nbr_unallocated_inodes: u16,
    /// Number of directories in group
    /*16 	17 	2*/
    nbr_directories: u16,
    pad: u16,
    reserved: [u8; 12],
}
