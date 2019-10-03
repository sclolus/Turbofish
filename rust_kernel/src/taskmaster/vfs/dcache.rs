use super::direntry::{DirectoryEntry, DirectoryEntryId};
use super::path::Path;
use super::SysResult;
use fallible_collections::btree::BTreeMap;
use itertools::unfold;
use libc_binding::Errno::*;

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

    pub fn get_entry(&self, id: &DirectoryEntryId) -> SysResult<&DirectoryEntry> {
        Ok(self.d_entries.get(&id).ok_or(ENOENT)?)
    }

    pub fn get_entry_mut(&mut self, id: &DirectoryEntryId) -> SysResult<&mut DirectoryEntry> {
        Ok(self.d_entries.get_mut(&id).ok_or(ENOENT)?)
    }

    pub fn contains_entry(&self, id: &DirectoryEntryId) -> bool {
        self.get_entry(id).is_ok()
    }

    pub fn add_entry(
        &mut self,
        parent: Option<DirectoryEntryId>,
        mut entry: DirectoryEntry,
    ) -> SysResult<DirectoryEntryId> {
        let id = self.get_available_id();

        entry.id = id;
        entry.parent_id = parent.unwrap_or(self.root_id); //eeeeeh yeah
        if self.d_entries.contains_key(&id) {
            return Err(EEXIST);
        }
        self.d_entries.try_insert(id, entry)?;

        if let Some(parent) = parent {
            let parent = match self.d_entries.get_mut(&parent) {
                None => return Err(ENOENT),
                Some(parent) => parent,
            };

            parent.add_entry(id)?;
        }
        Ok(id)
    }

    pub fn remove_entry(&mut self, id: DirectoryEntryId) -> SysResult<DirectoryEntry> {
        let parent_id;
        {
            let entry = match self.d_entries.get(&id) {
                None => return Err(ENOENT),
                Some(entry) => entry,
            };

            if entry.is_directory() {
                if !entry.is_directory_empty()? {
                    return Err(ENOTEMPTY);
                } else if entry.is_mounted()? {
                    return Err(EBUSY)?;
                }
            }
            parent_id = entry.parent_id;
        }
        if let Some(parent_dir) = self.d_entries.get_mut(&parent_id) {
            parent_dir.remove_entry(id)?;
        }
        Ok(match self.d_entries.remove(&id) {
            None => return Err(ENOENT),
            Some(entry) => entry,
        })
    }

    #[allow(dead_code)]
    pub fn dentry_path(&self, id: DirectoryEntryId) -> SysResult<Path> {
        let mut current_id = id;
        let mut rev_path = Path::new();
        let mut entry = self.d_entries.get(&current_id).ok_or(ENOENT)?;
        loop {
            if entry.id == entry.parent_id {
                break;
            }
            rev_path.push(entry.filename)?;
            current_id = entry.parent_id;
            entry = self.d_entries.get(&current_id).ok_or(ENOENT)?;
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

    pub fn walk_tree<F: FnMut(&DirectoryEntry) -> SysResult<()>>(
        &self,
        root: &DirectoryEntry,
        callback: &mut F,
    ) -> SysResult<()> {
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

    pub fn children(
        &self,
        dir_id: DirectoryEntryId,
    ) -> SysResult<impl Iterator<Item = (&DirectoryEntry)>> {
        let dcache = self;
        let mut children_iter = self.get_entry(&dir_id)?.get_directory()?.entries();

        Ok(unfold((), move |_| match children_iter.next() {
            None => None,
            Some(id) => dcache.get_entry(&id).ok(),
        }))
    }

    pub fn move_dentry(
        &mut self,
        id: DirectoryEntryId,
        new_parent: DirectoryEntryId,
    ) -> SysResult<DirectoryEntryId> {
        let parent_id;
        {
            let entry = self.d_entries.get(&id).ok_or(ENOENT)?;
            parent_id = entry.parent_id;
        }
        if let Some(parent_dir) = self.d_entries.get_mut(&parent_id) {
            parent_dir.remove_entry(id)?;
        }
        let entry = self.d_entries.remove(&id).ok_or(ENOENT)?;
        self.add_entry(Some(new_parent), entry)
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

    pub fn iter(&self) -> impl Iterator<Item = &DirectoryEntry> {
        self.d_entries.iter().map(|(_, entry)| entry)
    }
}

use core::fmt::{Debug, Display, Error, Formatter};

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

impl Debug for Dcache {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        <Self as Display>::fmt(self, f)
    }
}
