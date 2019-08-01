use std::str::FromStr;
use super::{DcacheResult, DcacheError};
use crate::path::{Path, Filename};
use super::inode::InodeId;
use DcacheError::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct DirectoryEntryId(usize);

impl DirectoryEntryId {
    pub fn new(id: usize) -> DirectoryEntryId {
        Self(id)
    }

    pub fn next_id(&self) -> Option<DirectoryEntryId> {
        let id = self.0.checked_add(1)?;
        Some(Self::new(id))
    }
}

use std::fmt::{Display, Formatter, Error};
use std::convert::TryFrom;
impl Display for DirectoryEntryId {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        Ok(write!(f, "D #{}", self.0)?)
    }
}

#[derive(Debug, Clone)]
pub struct EntryDirectory {
    entries: Vec<DirectoryEntryId>,
}

impl EntryDirectory {
    pub fn is_directory_empty(&self) -> bool {
        self.entries.len() == 0
    }

    pub fn entries(&self) -> &Vec<DirectoryEntryId> {
        &self.entries
    }
}

impl Default for EntryDirectory {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DirectoryEntryInner {
    Regular,
    Directory(EntryDirectory),
    Symlink(Path),
}

use DirectoryEntryInner::*;
macro_rules!    is_variant {
    ($pat: pat = $it: tt) => {
        if let $pat = $it {
            true
        } else {
            false
        }
    };
}

impl DirectoryEntryInner {
    pub fn is_directory(&self) -> bool {
        is_variant!(Directory(_) = self)
    }

    pub fn is_symlink(&self) -> bool {
        is_variant!(Symlink(_) = self)
    }

    pub fn is_regular(&self) -> bool {
        is_variant!(Regular = self)
    }

    pub fn is_directory_empty(&self) -> DcacheResult<bool> {
        Ok(self.get_directory()?.is_directory_empty())
    }

    pub fn get_directory(&self) -> DcacheResult<&EntryDirectory> {
        use DirectoryEntryInner::*;
        Ok(match self {
            Directory(ref directory) => directory,
            _ => return Err(NotADirectory),
        })
    }

    pub fn get_directory_mut(&mut self) -> DcacheResult<&mut EntryDirectory> {
        use DirectoryEntryInner::*;
        Ok(match self {
            Directory(ref mut directory) => directory,
            _ => return Err(NotADirectory),
        })
    }

    pub fn get_symbolic_content(&self) -> DcacheResult<&Path> {
        use DirectoryEntryInner::*;
        Ok(match self {
            Symlink(ref path) => path,
            _ => return Err(NotASymlink),
        })
    }
}

#[derive(Debug, Clone)]
pub struct DirectoryEntry {
    pub filename: Filename,
    inner: DirectoryEntryInner,
    pub id: DirectoryEntryId,
    pub parent_id: DirectoryEntryId,
    pub inode_id: InodeId,
}

impl DirectoryEntry {
    // ---------- BUILDER PATTERN ------------
    pub fn set_filename(&mut self, filename: Filename) -> &mut Self {
        self.filename = filename;
        self
    }

    pub fn set_id(&mut self, id: DirectoryEntryId) -> &mut Self {
        self.id = id;
        self
    }

    pub fn set_parent_id(&mut self, parent_id: DirectoryEntryId) -> &mut Self {
        self.parent_id = parent_id;
        self
    }

    pub fn set_inode_id(&mut self, inode_id: InodeId) -> &mut Self {
        self.inode_id = inode_id;
        self
    }

    pub fn set_directory(&mut self) -> &mut Self {
        self.inner = DirectoryEntryInner::Directory(EntryDirectory::default());
        self
    }

    pub fn set_regular(&mut self) -> &mut Self {
        self.inner = DirectoryEntryInner::Regular;
        self
    }

    pub fn set_symlink(&mut self, path: Path) -> &mut Self {
        self.inner = DirectoryEntryInner::Symlink(path);
        self
    }

    pub fn root_entry() -> Self {
        let mut root_entry = DirectoryEntry::default();
        root_entry
            .set_filename(Filename::try_from("root").unwrap())
            .set_id(DirectoryEntryId::new(2))
            .set_directory();

        root_entry
    }
    // ---------- BUILDER PATTERN END ------------

    pub fn is_directory(&self) -> bool {
        self.inner.is_directory()
    }

    pub fn is_symlink(&self) -> bool {
        self.inner.is_symlink()
    }

    pub fn is_regular(&self) -> bool {
        self.inner.is_regular()
    }

    pub fn get_symbolic_content(&self) -> DcacheResult<&Path> {
        self.inner.get_symbolic_content()
    }

    pub fn get_directory(& self) -> DcacheResult<&EntryDirectory> {
        self.inner.get_directory()
    }

    pub fn get_directory_mut(&mut self) -> DcacheResult<&mut EntryDirectory> {
        self.inner.get_directory_mut()
    }

    pub fn is_directory_empty(&self) -> DcacheResult<bool> {
        self.inner.is_directory_empty()
    }

    pub fn add_entry(&mut self, entry: DirectoryEntryId) -> DcacheResult<()> {
        let directory = self.inner.get_directory_mut()?;

        directory.entries.push(entry);
        Ok(())
    }

    pub fn remove_entry(&mut self, entry: DirectoryEntryId) -> DcacheResult<()> {
        let directory = self.inner.get_directory_mut()?;

        let index = match directory.entries.iter().position(|&x| x == entry) {
            Some(index) => index,
            None => return Err(NoSuchEntry),
        };
        directory.entries.swap_remove(index);
        Ok(())
    }
}

impl Default for DirectoryEntry {
    fn default() -> Self {
        Self {
            filename: Filename::try_from("DefaultFilenameChangeThisLol").unwrap(), // remove this unwrap somehow
            inner: DirectoryEntryInner::Regular,
            id: DirectoryEntryId::new(0),
            parent_id: DirectoryEntryId::new(0),
            inode_id: InodeId::new(0),
        }
    }
}
