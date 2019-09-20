use super::IpcResult;
use super::{
    DirectoryEntry as VfsDirectoryEntry, DirectoryEntryId, Driver, FileOperation, FileSystem,
    FileSystemId, SysResult,
};
use super::{
    DirectoryEntryBuilder, Filename, Inode as VfsInode, InodeData as VfsInodeData, InodeId, Path,
};

use super::dead::DeadFileSystem;
use super::{KeyGenerator, Mapper};
use crate::taskmaster::drivers::DefaultDriver;
use alloc::{boxed::Box, vec::Vec};
use core::convert::{TryFrom, TryInto};
use core::fmt::Debug;
use core::ops::{Deref, DerefMut};
use fallible_collections::{
    arc::FallibleArc,
    boxed::FallibleBox,
    btree::BTreeMap,
    vec::{FallibleVec, TryCollect},
    TryClone,
};
use libc_binding::{gid_t, statfs, uid_t, utimbuf, Errno, FileType};

use alloc::sync::Arc;
use core::default::Default;
use sync::DeadMutex;
mod procfs_driver;
use procfs_driver::ProcFsDriver;

mod version;
pub use version::VersionDriver;

#[derive(Debug)]
pub struct ProcFs {
    fs_id: FileSystemId,
    inodes: BTreeMap<InodeId, Inode>,
    direntries: BTreeMap<DirectoryEntryId, DirectoryEntry>,
}

impl KeyGenerator<InodeId> for ProcFs {}
impl KeyGenerator<DirectoryEntryId> for ProcFs {}

#[derive(Debug)]
struct DirectoryEntry(VfsDirectoryEntry);

impl Deref for DirectoryEntry {
    type Target = VfsDirectoryEntry;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DirectoryEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DirectoryEntry {
    fn root_entry() -> Self {
        Self(VfsDirectoryEntry::root_entry())
    }
}

#[derive(Debug)]
struct Inode(VfsInode, Box<dyn Driver>);

impl Deref for Inode {
    type Target = VfsInode;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Inode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Mapper<InodeId, Inode> for ProcFs {
    fn get_map(&mut self) -> &mut BTreeMap<InodeId, Inode> {
        &mut self.inodes
    }
}

impl Mapper<DirectoryEntryId, DirectoryEntry> for ProcFs {
    fn get_map(&mut self) -> &mut BTreeMap<DirectoryEntryId, DirectoryEntry> {
        &mut self.direntries
    }
}

impl ProcFs {
    pub fn new(fs_id: FileSystemId) -> SysResult<Self> {
        let mut new = Self {
            fs_id: fs_id,
            inodes: BTreeMap::new(),
            direntries: BTreeMap::new(),
        };

        let mut root_direntry = VfsDirectoryEntry::root_entry();
        let root_inode_id = new.new_inode_id(root_direntry.inode_id.inode_number);

        root_direntry
            .set_filename(Filename::try_from("ProcFsRoot").unwrap())
            .set_inode_id(root_inode_id);

        let mut inode = VfsInode::root_inode()?;
        let inode_id = new.new_inode_id(inode.id.inode_number);

        let root_direntry = DirectoryEntry(root_direntry);
        let root_dir_id = root_direntry.id;
        let driver = Box::try_new(DefaultDriver)?;

        let inode = Inode(inode, driver);

        new.inodes.try_insert(inode_id, inode)?;
        new.direntries.try_insert(root_dir_id, root_direntry)?;

        // Inserting divers basic procfs files.
        let driver = Box::try_new(DefaultDriver)?;
        let filesystem = Arc::try_new(DeadMutex::new(DeadFileSystem))?;

        let mut inode_id: InodeId = new.gen();
        inode_id.filesystem_id = Some(fs_id);
        let access_mode = FileType::REGULAR_FILE | FileType::from_bits(0777).unwrap();

        let vfs_inode_data = *VfsInodeData::default()
            .set_id(inode_id)
            .set_access_mode(access_mode)
            .set_uid(0)
            .set_gid(0);

        let version_inode = Inode(
            VfsInode::new(filesystem, driver, vfs_inode_data),
            Box::try_new(version::VersionDriver)?,
        );

        let version_dir_id: DirectoryEntryId = new.gen();
        let mut base_directory_entry = VfsDirectoryEntry::default();
        base_directory_entry
            .set_filename("version".try_into().unwrap())
            .set_inode_id(inode_id)
            .set_id(version_dir_id)
            .set_parent_id(root_dir_id)
            .set_regular();
        let version_direntry = DirectoryEntry(base_directory_entry);

        new.inodes
            .try_insert(inode_id, version_inode)
            .or(Err(Errno::ENOMEM))?;
        new.direntries
            .try_insert(version_dir_id, version_direntry)
            .or(Err(Errno::ENOMEM))?;

        new.direntries
            .get_mut(&root_dir_id)
            .unwrap()
            .add_entry(version_dir_id)?;
        //

        Ok(new)
    }

    pub fn new_inode_id(&self, inode_nbr: u32) -> InodeId {
        InodeId::new(inode_nbr, Some(self.fs_id))
    }
}

impl FileSystem for ProcFs {
    fn root(&self) -> SysResult<(VfsDirectoryEntry, VfsInodeData, Box<dyn Driver>)> {
        let direntry = self
            .direntries
            .get(&DirectoryEntryId::new(2))
            .expect("There should be a root direntry for procfs");
        let mut inode = self
            .inodes
            .get(&direntry.inode_id)
            .expect("There should be a root inode for procfs");

        let mut new_direntry = direntry.0.clone();

        new_direntry.get_directory_mut().unwrap().clear_entries();
        Ok((new_direntry, inode.inode_data, Box::try_new(DefaultDriver)?))
    }

    fn lookup_directory(
        &mut self,
        inode_nbr: u32,
    ) -> SysResult<Vec<(VfsDirectoryEntry, VfsInodeData, Box<dyn Driver>)>> {
        let inode_id = self.new_inode_id(inode_nbr);
        let get_inode = |inode_id| self.inodes.get(&inode_id);

        let inode = get_inode(inode_id).ok_or(Errno::ENOENT)?;

        if !inode.is_directory() {
            return Err(Errno::ENOTDIR);
        }

        // That's very dummy but hey, fuck this design.
        let direntry = self
            .direntries
            .iter()
            .map(|(_, dir)| dir)
            .find(|dir| dir.inode_id == inode_id)
            .expect("No corresponding directory for Inode");

        Ok(direntry
            .get_directory()
            .expect("Direntry was not a directory.")
            .entries()
            .filter_map(|direntry_id| {
                let direntry = self.direntries.get(direntry_id).expect("WTF");
                //remove this unwrap
                let driver: Box<dyn Driver> = Box::try_new(version::VersionDriver).unwrap();

                if let (ent, Some(inode)) = (direntry, get_inode(direntry.inode_id)) {
                    Some((ent.0.clone(), inode.inode_data.clone(), driver))
                } else {
                    None
                }
            })
            .try_collect()?)
    }
}
