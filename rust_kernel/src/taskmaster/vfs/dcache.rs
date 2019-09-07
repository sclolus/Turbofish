use super::direntry::{DirectoryEntry, DirectoryEntryId};
use super::path::Path;
use super::tools::{DcacheError, DcacheResult};
use alloc::collections::BTreeMap;
use DcacheError::*;

pub struct Dcache {
    pub root_id: DirectoryEntryId,
    pub d_entries: BTreeMap<DirectoryEntryId, DirectoryEntry>, // remove those pubs
    pub path_cache: BTreeMap<Path, DirectoryEntryId>,
}

impl Dcache {
    pub fn new() -> Self {
        let root_entry = DirectoryEntry::root_entry();

        let mut new = Self {
            root_id: root_entry.id,
            d_entries: BTreeMap::new(),
            path_cache: BTreeMap::new(),
        };

        new.add_entry(None, root_entry)
            .expect("Could not add a root to the Dcache");
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

    pub fn add_entry(
        &mut self,
        parent: Option<DirectoryEntryId>,
        mut entry: DirectoryEntry,
    ) -> DcacheResult<DirectoryEntryId> {
        let id = self.get_available_id();

        entry.id = id;
        entry.parent_id = parent.unwrap_or(self.root_id); //eeeeeh yeah
        if self.d_entries.contains_key(&id) {
            return Err(FileAlreadyExists);
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

    pub fn _remove_entry(&mut self, id: DirectoryEntryId) -> DcacheResult<DirectoryEntry> {
        let parent_id;
        {
            let entry = match self.d_entries.get(&id) {
                None => return Err(NoSuchEntry),
                Some(entry) => entry,
            };

            if entry.is_directory() {
                if !entry.is_directory_empty()? {
                    return Err(NotEmpty);
                } else if entry.is_mounted()? {
                    return Err(DirectoryIsMounted)?;
                }
            }
            parent_id = entry.parent_id;
        }
        let parent_dir = self
            .d_entries
            .get_mut(&parent_id)
            .ok_or(EntryNotConnected)?;

        parent_dir.remove_entry(id)?;
        Ok(match self.d_entries.remove(&id) {
            None => return Err(NoSuchEntry),
            Some(entry) => entry,
        })
    }

    #[allow(dead_code)]
    pub fn dentry_path(&self, id: DirectoryEntryId) -> DcacheResult<Path> {
        let mut current_id = id;
        let mut rev_path = Path::new();
        let mut entry = self.d_entries.get(&current_id).ok_or(NoSuchEntry)?;
        loop {
            if entry.id == entry.parent_id {
                break;
            }
            rev_path.push(entry.filename)?;
            current_id = entry.parent_id;
            entry = self.d_entries.get(&current_id).ok_or(NoSuchEntry)?;
            if entry.is_mounted()? {
                rev_path.pop();
            }
        }
        let mut path = Path::new();

        while let Some(component) = rev_path.pop() {
            path.push(component)?;
        }
        Ok(path)
    }

    pub fn walk_tree<F: FnMut(&DirectoryEntry) -> DcacheResult<()>>(
        &self,
        root: &DirectoryEntry,
        callback: &mut F,
    ) -> DcacheResult<()> {
        let directory = root.get_directory()?;

        let mapping_closure = |entry_id| {
            self.d_entries
                .get(entry_id)
                .expect("Invalid entry_id in directory in dcache")
        };
        for entry in directory.entries().map(mapping_closure) {
            callback(entry)?;
        }

        for entry in directory
            .entries()
            .map(mapping_closure)
            .filter(|x| x.is_directory())
        {
            self.walk_tree(entry, callback)?;
        }
        Ok(())
    }

    pub fn _move_dentry(
        &mut self,
        id: DirectoryEntryId,
        new_parent: DirectoryEntryId,
    ) -> DcacheResult<()> {
        let parent_id;
        {
            let entry = self.d_entries.get(&id).ok_or(NoSuchEntry)?;
            parent_id = entry.parent_id;
        }
        let parent_dir = self
            .d_entries
            .get_mut(&parent_id)
            .ok_or(EntryNotConnected)?;

        parent_dir.remove_entry(id)?;
        let entry = self.d_entries.remove(&id).ok_or(NoSuchEntry)?;
        self.add_entry(Some(new_parent), entry)?;
        Ok(())
    }

    fn get_available_id(&self) -> DirectoryEntryId {
        let mut current_id = self.root_id; // check this
        loop {
            if let None = self.d_entries.get(&current_id) {
                return current_id;
            }
            current_id = current_id
                .next_id()
                .expect("No space left inside the dcache lool");
        }
    }
}

use core::fmt::{Display, Error, Formatter};

impl Display for Dcache {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let root = self
            .d_entries
            .get(&self.root_id)
            .expect("There is no root direntry for Dcache");
        self.walk_tree(root, &mut |entry: &DirectoryEntry| {
            writeln!(f, "-{}-", entry.filename).unwrap(); // take care of this if possible.
            Ok(())
        })
        .unwrap();

        Ok(())
    }
}
