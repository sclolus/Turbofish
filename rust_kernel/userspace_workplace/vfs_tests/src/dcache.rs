use std::cmp::{Eq, PartialEq, Ord, PartialOrd};
use std::collections::BTreeMap;
use std::str::FromStr;
use super::path::{Path, Filename};
use super::direntry::{DirectoryEntry, DirectoryEntryId};
use errno::Errno;
use std::convert::TryInto;

#[derive(Debug, Copy, Clone)]
pub enum DcacheError {
    FileAlreadyExists,
    NoSuchEntry,
    NotADirectory,
    NotASymlink,
    InvalidEntryIdInDirectory,
    RootDoesNotExists,
    NotEmpty,
    EntryNotConnected,
    NotEnoughArguments,
    Errno(Errno),
}

impl From<Errno> for DcacheError {
    fn from(errno: Errno) -> Self {
        DcacheError::Errno(errno)
    }
}

pub type DcacheResult<T> = Result<T, DcacheError>;

pub struct Dcache {
    pub root_id: DirectoryEntryId,
    pub d_entries: BTreeMap<DirectoryEntryId, DirectoryEntry>, // remove those pubs
    pub path_cache: BTreeMap<Path, DirectoryEntryId>,
}

use DcacheError::*;
impl Dcache {
    pub fn new() -> Self {
        let root_entry = DirectoryEntry::root_entry();

        let mut new = Self {
            root_id: root_entry.id,
            d_entries: BTreeMap::new(),
            path_cache: BTreeMap::new(),
        };

        new.add_entry(None, root_entry).expect("Could not add a root to the Dcache");
        new
    }

    pub fn get_entry(&self, id: &DirectoryEntryId) -> DcacheResult<&DirectoryEntry> {
        Ok(self.d_entries.get(&id).ok_or(NoSuchEntry)?)
    }

    pub fn get_entry_mut(&mut self, id: &DirectoryEntryId) -> DcacheResult<&mut DirectoryEntry> {
        Ok(self.d_entries.get_mut(&id).ok_or(NoSuchEntry)?)
    }

    pub fn contains_entry(&self, id: &DirectoryEntryId) -> bool {
        self.get_entry(id).is_ok()
    }

    pub fn add_entry(&mut self, parent: Option<DirectoryEntryId>, mut entry: DirectoryEntry) -> DcacheResult<DirectoryEntryId> {
        let id = self.get_available_id();

        entry.id = id;
        entry.parent_id = parent.unwrap_or(self.root_id); //eeeeeh yeah
        if self.d_entries.contains_key(&id) {
            return Err(FileAlreadyExists)
        }
        self.d_entries.insert(id, entry);

        if let Some(parent) = parent {
            let parent = match self.d_entries.get_mut(&parent) {
                None => return Err(NoSuchEntry),
                Some(parent) => parent,
            };

            parent.add_entry(id)?;
        }
        Ok(id)
    }

    pub fn remove_entry(&mut self, id: DirectoryEntryId) -> DcacheResult<DirectoryEntry> {
        let parent_id;
        {
            let entry = match self.d_entries.get(&id) {
                None => return Err(NoSuchEntry),
                Some(entry) => entry,
            };

            if entry.is_directory() && !entry.is_directory_empty()? {
                return Err(NotEmpty)
            }
            parent_id = entry.parent_id;
        }
        let parent_dir = self.d_entries.get_mut(&parent_id).ok_or(EntryNotConnected)?;

        parent_dir.remove_entry(id)?;
        Ok(match self.d_entries.remove(&id) {
            None => return Err(NoSuchEntry),
            Some(entry) => entry,
        })
    }

    pub fn dentry_path(&self, id: DirectoryEntryId) -> DcacheResult<Path> {
        let mut current_id = id;
        let mut rev_path = Path::new();
        loop {
            let entry = self.d_entries.get(&current_id).ok_or(NoSuchEntry)?;

            rev_path.push(entry.filename);
            if entry.id == entry.parent_id {
                break;
            }
            current_id = entry.parent_id;
        }
        let mut path = Path::new();

        while let Some(component) = rev_path.pop() {
            path.push(component);
        }
        Ok(path)
    }

    pub fn walk_tree<F: FnMut(&DirectoryEntry) -> DcacheResult<()>>(&self, root: &DirectoryEntry, mut callback: &mut F) -> DcacheResult<()>  {
        let directory = root.get_directory()?;

        let mapping_closure = |entry_id| self.d_entries.get(entry_id).expect("Invalid entry_id in directory in dcache");
        for entry in directory.entries().iter().map(mapping_closure) {
            callback(entry)?;
        }

        for entry in directory.entries().iter().map(mapping_closure).filter(|x| x.is_directory()) {
            self.walk_tree(entry, callback)?;
        }
        Ok(())
    }

    fn move_dentry(&mut self, id: DirectoryEntryId, new_parent: DirectoryEntryId) -> DcacheResult<()> {
        let parent_id;
        {
            let entry = self.d_entries.get(&id).ok_or(NoSuchEntry)?;
            parent_id = entry.parent_id;
        }
        let parent_dir = self.d_entries.get_mut(&parent_id).ok_or(EntryNotConnected)?;

        parent_dir.remove_entry(id)?;
        let entry = self.d_entries.remove(&id).ok_or(NoSuchEntry)?;
        self.add_entry(Some(new_parent), entry)?;
        Ok(())
    }

    pub fn rename_dentry(&mut self, cwd: DirectoryEntryId, id: DirectoryEntryId, new_pathname: Path) -> DcacheResult<()> {
        let new_filename = new_pathname.filename().unwrap(); // ?

        if new_filename == &"." || new_filename == &".." {
            return Err(Errno(Errno::Einval));
        }

        if let Ok(id) = self.pathname_resolution(cwd, new_pathname.clone()) {
            self.remove_entry(id)?;
        };

        let new_parent_id = self.pathname_resolution(cwd, new_pathname.parent())?;

        self.move_dentry(id, new_parent_id)?;

        let entry = self.d_entries.get_mut(&id).ok_or(NoSuchEntry)?;

        entry.set_filename(*new_filename);
        Ok(())
    }

    fn get_available_id(&self) -> DirectoryEntryId {
        let mut current_id = self.root_id; // check this
        loop {
            if let None = self.d_entries.get(&current_id) {
                return current_id
            }
            current_id = current_id.next_id().expect("No space left inside the dcache lool");
        }
    }
    fn _pathname_resolution(&self, mut root: DirectoryEntryId, pathname: Path, recursion_level: usize) -> DcacheResult<DirectoryEntryId> {
        use crate::posix_consts::SYMLOOP_MAX;
        if recursion_level > SYMLOOP_MAX {
            return Err(Errno(Errno::Eloop))
        }

        if pathname.is_absolute() {
            root = self.root_id;
        }

        if !self.contains_entry(&root) {
            return Err(RootDoesNotExists)
        }

        let mut current_dir_id = root;
        let mut components = pathname.components();
        let mut was_symlink = false;
        let mut current_entry = self.get_entry(&current_dir_id)?;
        let mut current_next_entry_id = root;
        for component in components.by_ref() {
            let current_dir = current_entry.get_directory()?;

            if component == &"." {
                continue ;
            } else if component == &".." {
                current_dir_id = current_entry.parent_id;
                current_entry = self.get_entry(&current_dir_id)?;
                continue ;
            }
            let next_entry_id = current_dir.entries().iter()
                .find(|x| {
                    let filename = &self.get_entry(x)
                        .expect("Invalid entry id in a directory entry that is a directory").filename;
                    filename == component
                }).ok_or(NoSuchEntry)?;

            current_next_entry_id = *next_entry_id;
            current_entry = self.get_entry(next_entry_id)?;
            if current_entry.is_symlink() {
                was_symlink = true;
                break ;
            }
            current_dir_id = *next_entry_id;
        }
        if was_symlink {
            let mut new_path = current_entry.get_symbolic_content()?.clone();
            new_path.chain(components.try_into()?)?;

            self._pathname_resolution(current_dir_id, new_path, recursion_level + 1)
        } else {
            Ok(self.get_entry(&current_dir_id).unwrap().id)
        }

    }

    pub fn pathname_resolution(&self, root: DirectoryEntryId, pathname: Path) -> DcacheResult<DirectoryEntryId> {
        self._pathname_resolution(root, pathname, 0)
    }

}
use core::fmt::{Display, Error, Formatter};

impl Display for Dcache {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let root = self.d_entries.get(&self.root_id).unwrap();
        self.walk_tree(root, &mut |entry: &DirectoryEntry| {writeln!(f, "-{}-", entry.filename ); Ok(())}).unwrap();

        Ok(())
    }
}
