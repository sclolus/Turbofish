//! This file describe all the Directory Entry Header model

use core::fmt;

// Directories are inodes which contain some number of "entries" as their contents.
// These entries are nothing more than a name/inode pair. For instance the inode
// orresponding to the root directory might have an entry with the name of "etc" and an inode value of 50.
// A directory inode stores these entries in a linked-list fashion in its contents blocks.

// The root directory is Inode 2.

// The total size of a directory entry may be longer then the length of the name would imply
// (The name may not span to the end of the record), and records have to be aligned to 4-byte
// boundaries. Directory entries are also not allowed to span multiple blocks on the file-system,
// so there may be empty space in-between directory entries. Empty space is however not allowed
// in-between directory entries, so any possible empty space will be used as part of the preceding
// record by increasing its record length to include the empty space. Empty space may also be
// equivalently marked by a separate directory entry with an inode number of zero, indicating that directory entry should be skipped.

/// Directory Entry base structure
#[derive(Debug, Copy, Clone)]
#[repr(packed)]
pub struct DirectoryEntryHeader {
    /// Inode
    /*0 	3 	4*/
    pub inode: u32,
    /// Total size of this entry (Including all subfields)
    /*4 	5 	2*/
    pub entry_size: u16,
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

/// Type of file
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

/// Implementations of the Directory Entry
impl DirectoryEntryHeader {
    /// Get the file name
    pub fn get_filename(&self) -> &str {
        unsafe {
            let slice: &[u8] = core::slice::from_raw_parts(&self.filename.0 as *const u8, self.name_length as usize);
            core::str::from_utf8_unchecked(slice)
        }
    }
}

/// Newtype of filename
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Filename(pub [u8; 256]);

/// Debug boilerplate of filename
impl fmt::Debug for Filename {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", "Filename")
    }
}
