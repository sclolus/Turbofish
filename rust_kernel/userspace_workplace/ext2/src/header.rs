//! This module provide header structures and methods for EXT2 filesystems

use super::Block;

mod superblock;
pub use superblock::SuperBlock;

mod block_group;
pub use block_group::BlockGroupDescriptor;

// *** Header of EXT2 partition ***
// 0            1024         2048
// +------------+------------+-------------->
// |    1024    |  Super     | Block group
// |    free    |  Block     | Descriptor Table
// +-------------------------+-------------->
//
// *** Block group descriptor table ***
// 0              32             64             96
// +--------------+--------------+--------------+------------->
// | block group  | block group  | block group  | block group
// | descriptor 1 | descriptor 2 | descriptor 3 | descriptor N
// +--------------+--------------+--------------+------------->
