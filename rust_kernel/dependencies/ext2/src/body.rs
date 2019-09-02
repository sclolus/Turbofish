//! This module provide common structures and methods for EXT2 filesystems

use super::Block;

mod inode;
pub use inode::Inode;

mod directory_entry;
pub use directory_entry::{DirectoryEntry, DirectoryEntryHeader, DirectoryEntryType};
