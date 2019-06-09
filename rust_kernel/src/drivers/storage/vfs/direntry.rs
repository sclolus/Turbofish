// #![deny(missing_docs)]
use super::posix_consts::*;
use super::{VfsResult, VfsError};
use super::Filetype;
use core::convert::TryFrom;
use core::fmt;
use core::str::Split;
use core::mem;
use alloc;
use errno::Errno;

use alloc::vec;
use alloc::vec::Vec;

/// Newtype of filename
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Filename(pub [u8; NAME_MAX], pub usize);

impl TryFrom<&str> for Filename {
    type Error = Errno;
    fn try_from(s: &str) -> Result<Self, Errno> {
        let mut n = [0; NAME_MAX];
        if s.len() >= NAME_MAX {
            return Err(Errno::Enametoolong);
        } else {
            for (n, c) in n.iter_mut().zip(s.bytes()) {
                *n = c;
            }
            Ok(Self(n, s.len()))
        }
    }
}

impl Default for Filename {
    fn default() -> Self {
        Self([0; NAME_MAX], 0)
    }
}

impl PartialEq for Filename {
    fn eq(&self, other: &Self) -> bool {
        self.0[..self.1] == other.0[..self.1]
    }
}

/// Debug boilerplate of filename
impl fmt::Debug for Filename {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let slice: &[u8] = core::slice::from_raw_parts(&self.0 as *const u8, self.1);
            let s = core::str::from_utf8_unchecked(slice);
            write!(f, "{:?}", s)
        }
    }
}

#[derive(Debug)]
pub struct DirectoryEntryHeader {
    /// inode number of the entry.
    pub inode: u32,

    pub filename: Filename,
    pub name_size: usize,

    pub filetype: Filetype,
}

#[derive(Debug)]
pub struct DirectoryEntry {

    pub header: DirectoryEntryHeader,
    pub entry: Entry,
}

#[derive(Debug)]
pub enum Entry {
    RegularFile {

    },
    Directory {
        direntries: Vec<DirectoryEntry>,
    },
    CharacterDevice {
    },
    BlockDevice {
    },
    Fifo {
    },
    Socket {
    },
    SymbolicLink {
    },
}

impl DirectoryEntry {
    pub fn new(filename: &str, filetype: Filetype, inode: u32) -> VfsResult<Self> {
        let mut header: DirectoryEntryHeader = unsafe { mem::zeroed() };

        if filename.as_bytes().len() > NAME_MAX {
            unimplemented!();
        }

        header.inode = inode;
        header.filetype = filetype;
        // (header.filename.0)[0..filename.as_bytes().len()].copy_from_slice(filename.as_bytes());
        header.filename = TryFrom::try_from(filename)?;
        header.name_size = filename.len();
        use Filetype::*;

        let entry = match filetype {
            Directory => {
                Entry::Directory {
                    direntries: Vec::new()
                }
            }
            _ => unimplemented!()
        };
        Ok(Self {
            header,
            entry
        })
    }

    pub unsafe fn get_filename(&self) -> &str {
        let slice: &[u8] = core::slice::from_raw_parts(
            &self.header.filename.0 as *const u8,
            self.header.name_size,
        );
        core::str::from_utf8_unchecked(slice)
    }

    pub fn is_directory(&self) -> bool {
        self.header.filetype == Filetype::Directory
    }

    pub fn directory_entries(&self) -> VfsResult<core::slice::Iter<DirectoryEntry>> {
        use Entry::*;
        if self.is_directory() {
            if let Directory {
                direntries,
            } = &self.entry {
                Result::Ok(direntries.iter())
            } else {
                panic!("Impossible condition");
            }
        }
        else {
            Result::Err(VfsError::IsNotADirectory)
        }
    }

    pub fn directory_entries_mut(&mut self) -> VfsResult<core::slice::IterMut<DirectoryEntry>> {
        use Entry::*;
        if let Directory {
            direntries,
        } = &mut self.entry {
            Result::Ok(direntries.iter_mut())
        } else {
            Result::Err(VfsError::IsNotADirectory)
        }
    }

    pub fn add_direntry(&mut self, entry: DirectoryEntry) -> VfsResult<&mut DirectoryEntry> {
        if !self.is_directory() {
            return Result::Err(VfsError::IsNotADirectory);
        }

        if let Entry::Directory { direntries } = &mut self.entry {
            direntries.push(entry);
            let len = direntries.len();

            Result::Ok(&mut direntries[len - 1])
        } else {
            panic!("Impossible condition");
        }
    }

    pub fn walk_tree<E, F: FnMut(&DirectoryEntry) -> Result<(), E>>(&self, mut callback: &mut F) -> Result<(), E> {
        if !self.is_directory() {
            return Ok(())
        }

        for entry in self.directory_entries().unwrap() {
            callback(entry)?;
        }

        for entry in self.directory_entries().unwrap().filter(|x| x.is_directory()) {
            entry.walk_tree(callback)?;
        }
        Ok(())
    }

    pub fn walk_tree_mut<E, F: FnMut(&mut DirectoryEntry) -> Result<(), E>>(&mut self, mut callback: &mut F) -> Result<(), E> {
        if !self.is_directory() {
            return Ok(())
        }

        for entry in self.directory_entries_mut().unwrap().filter(|x| x.is_directory()) {
            if let Err(e) = entry.walk_tree_mut(callback) {
                return Err(e)
            }
        }

        for entry in self.directory_entries_mut().unwrap() {
            callback(entry)?;
        }
        Ok(())
    }
}
