//! This module provide common structures and methods for EXT2 filesystems

use super::Block;

mod inode;
pub use inode::{Inode, TypeAndPerm};

mod directory_entry;
pub use directory_entry::{DirectoryEntry, DirectoryEntryType};
