use super::IpcResult;
use super::{
    DirectoryEntry, DirectoryEntryBuilder, DirectoryEntryId, Driver, FileOperation, FileSystem,
    FileSystemId, SysResult,
};
use super::{Filename, Inode as VfsInode, InodeData as VfsInodeData, InodeId};

use crate::taskmaster::SCHEDULER;

use super::dead::DeadFileSystem;
use super::{KeyGenerator, Mapper};
use crate::taskmaster::drivers::DefaultDriver;
use crate::taskmaster::vfs::Dcache;
use alloc::{boxed::Box, vec::Vec};
use core::convert::{TryFrom, TryInto};
use core::ops::{Deref, DerefMut};
use fallible_collections::{
    arc::FallibleArc, boxed::FallibleBox, btree::BTreeMap, vec::TryCollect,
};
use libc_binding::{Errno, FileType};

use alloc::sync::Arc;
use core::default::Default;
use sync::DeadMutex;
mod procfs_driver;

mod version;
pub use version::VersionDriver;

mod filesystems;
pub use filesystems::FilesystemsDriver;

mod stat;
pub use stat::StatDriver;

unsafe impl Send for ProcFs {}

#[derive(Debug)]
pub struct ProcFs {
    fs_id: FileSystemId,
    inodes: BTreeMap<InodeId, Inode>,
    root_direntry_id: DirectoryEntryId,
    root_inode_id: InodeId,
    dcache: Dcache,
}

impl KeyGenerator<InodeId> for ProcFs {
    fn gen_filter(&self, mut key: InodeId) -> bool {
        key.filesystem_id = Some(self.fs_id);
        !self.inodes.contains_key(&key)
    }
}
impl KeyGenerator<DirectoryEntryId> for ProcFs {
    fn gen_filter(&self, key: DirectoryEntryId) -> bool {
        !self.dcache.contains_entry(&key)
    }
}

// #[derive(Debug)]
// struct DirectoryEntry(DirectoryEntry);

// impl Deref for DirectoryEntry {
//     type Target = DirectoryEntry;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl DerefMut for DirectoryEntry {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

// impl DirectoryEntry {
//     fn root_entry() -> Self {
//         Self(DirectoryEntry::root_entry())
//     }
// }

// #[derive(Debug)]
struct Inode(VfsInode, Box<FnMut() -> Box<dyn Driver>>);

impl core::fmt::Debug for Inode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        Ok(write!(f, "{:?}", self.0)?)
    }
}

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
        &mut self.dcache.d_entries
    }
}

impl ProcFs {
    pub fn register_file(
        &mut self,
        parent: DirectoryEntryId,
        name: Filename,
        gen_driver: Box<FnMut() -> Box<dyn Driver>>,
    ) -> SysResult<()> {
        let driver = Box::new(DefaultDriver);
        let filesystem = Arc::try_new(DeadMutex::new(DeadFileSystem))?;

        let mut inode_id: InodeId = self.gen();
        inode_id.filesystem_id = Some(self.fs_id);
        let access_mode = FileType::REGULAR_FILE | FileType::from_bits(0o777).unwrap();

        let vfs_inode_data = *VfsInodeData::default()
            .set_id(inode_id)
            .set_access_mode(access_mode)
            .set_uid(0)
            .set_gid(0);

        let inode = Inode(
            VfsInode::new(filesystem, driver, vfs_inode_data),
            gen_driver,
        );

        let mut direntry = DirectoryEntryBuilder::new();
        direntry
            .set_filename(name)
            .set_inode_id(inode_id)
            .set_parent_id(parent)
            .set_regular();

        let mut direntry = direntry.build();
        let dir_id = self
            .dcache
            .add_entry(Some(parent), direntry)
            .or(Err(Errno::ENOMEM))?;

        assert!(self
            .inodes
            .try_insert(inode_id, inode)
            .or(Err(Errno::ENOMEM))?
            .is_none());
        Ok(())
    }

    pub fn new(fs_id: FileSystemId) -> SysResult<Self> {
        let mut new = Self {
            fs_id: fs_id,
            inodes: BTreeMap::new(),
            dcache: Dcache::new(),
            root_direntry_id: DirectoryEntryId::new(0),
            root_inode_id: InodeId::new(0, Some(fs_id)),
        };

        new.root_direntry_id = new.dcache.root_id;

        let root_dir_id = new.root_direntry_id;
        let root_direntry = new.dcache.get_entry(&root_dir_id)?;
        let root_inode_id = new.new_inode_id(root_direntry.inode_id.inode_number);
        let root_direntry = new.dcache.get_entry_mut(&root_dir_id)?;
        root_direntry
            .set_filename(Filename::try_from("ProcFsRoot").unwrap())
            .set_inode_id(root_inode_id);

        let inode = VfsInode::root_inode()?;

        new.root_inode_id = root_inode_id;
        let driver = Box::new(DefaultDriver);

        let inode = Inode(inode, Box::new(|| Box::new(DefaultDriver)));

        log::warn!("root_inode_id: {:?}", root_inode_id);
        log::warn!("root_dir_id: {:?}", root_dir_id);
        new.inodes.try_insert(root_inode_id, inode)?;

        let version_filename = Filename::try_from("version").unwrap();
        let filesystems_filename = Filename::try_from("filesystems").unwrap();
        new.register_file(
            root_dir_id,
            filesystems_filename,
            Box::new(|| Box::new(filesystems::FilesystemsDriver)),
        )?;
        new.register_file(
            root_dir_id,
            version_filename,
            Box::new(|| Box::new(version::VersionDriver)),
        )?;

        // Inserting divers basic procfs files.

        Ok(new)
    }

    pub fn new_inode_id(&self, inode_nbr: u32) -> InodeId {
        InodeId::new(inode_nbr, Some(self.fs_id))
    }

    pub fn root_ids(&self) -> (DirectoryEntryId, InodeId) {
        (self.root_direntry_id, self.root_inode_id)
    }

    fn mkdir(
        &mut self,
        parent: DirectoryEntryId,
        filename: Filename,
        mode: FileType,
    ) -> SysResult<DirectoryEntryId> {
        let parent_dir = self.dcache.get_entry_mut(&parent)?;
        let driver = Box::new(DefaultDriver);
        let filesystem = Arc::try_new(DeadMutex::new(DeadFileSystem))?;

        let parent_directory = parent_dir
            .get_directory_mut()
            .expect("Parent in Procfs::mkdir() should be a directory");

        let inode_id: InodeId = self.gen();
        let inode_id = self.new_inode_id(inode_id.inode_number);
        let vfs_inode_data = *VfsInodeData::default()
            .set_id(inode_id)
            .set_access_mode(mode)
            .set_uid(0) //TODO change this.
            .set_gid(0);

        let inode = Inode(
            VfsInode::new(filesystem, driver, vfs_inode_data),
            Box::new(|| Box::new(DefaultDriver)),
        );

        let mut direntry = DirectoryEntryBuilder::new();
        direntry
            .set_filename(filename)
            .set_parent_id(parent)
            .set_inode_id(inode_id)
            .set_directory();

        let direntry = direntry.build();

        self.inodes.try_insert(inode_id, inode)?;
        match self.dcache.add_entry(Some(parent), direntry) {
            Ok(id) => Ok(id),
            Err(e) => {
                self.inodes.remove(&inode_id);
                Err(e)
            }
        }
    }
}

impl FileSystem for ProcFs {
    fn root(&self) -> SysResult<(DirectoryEntry, VfsInodeData, Box<dyn Driver>)> {
        let (root_dir_id, root_inode_id) = self.root_ids();

        let direntry = self
            .dcache
            .get_entry(&root_dir_id)
            .expect("There should be a root direntry for procfs");
        let inode = self
            .inodes
            .get(&root_inode_id)
            .expect("There should be a root inode for procfs");

        let mut new_direntry = direntry.clone();

        new_direntry.get_directory_mut().unwrap().clear_entries();
        Ok((new_direntry, inode.inode_data, Box::try_new(DefaultDriver)?))
    }

    fn lookup_directory(
        &mut self,
        inode_nbr: u32,
    ) -> SysResult<Vec<(DirectoryEntry, VfsInodeData, Box<dyn Driver>)>> {
        let inode_id = self.new_inode_id(inode_nbr);

        let inode = self.inodes.get_mut(&inode_id).ok_or(Errno::ENOENT)?;

        if !inode.is_directory() {
            return Err(Errno::ENOTDIR);
        }

        // That's very dummy but hey, fuck this design.
        let (root_dir_id, _root_inode_id) = self.root_ids();

        log::info!("Trying to lock scheduler");
        SCHEDULER.force_unlock();
        let scheduler = SCHEDULER.lock();

        let mut thread_groups = scheduler.iter_thread_groups();

        for thread_group in thread_groups {
            let pid = thread_group.pgid; // Is that so ?
            let pid_filename = format!("{}", pid); // unfaillible context
            let pid_filename = Filename::try_from(pid_filename.as_str())
                .expect("A Pid should a perfectly acceptable FileName");
            let mode = FileType::DIRECTORY
                | FileType::USER_READ_PERMISSION
                | FileType::USER_EXECUTE_PERMISSION
                | FileType::GROUP_READ_PERMISSION
                | FileType::GROUP_EXECUTE_PERMISSION
                | FileType::OTHER_READ_PERMISSION
                | FileType::OTHER_EXECUTE_PERMISSION;

            let dir_id = self.mkdir(root_dir_id, pid_filename, mode)?;

            let stat_filename = Filename::try_from("stat").expect("This filename should be valid");
            self.register_file(
                dir_id,
                stat_filename,
                Box::new(move || Box::new(StatDriver::new(pid))),
            )?;
        }

        let direntry = self
            .dcache
            .iter()
            // .map(|(_, dir)| dir)
            .find(|dir| dir.inode_id == inode_id)
            .expect("No corresponding directory for Inode");

        eprintln!("Trying to access childs of {:?}", direntry.id);

        let inodes = &mut self.inodes;
        let dcache = &self.dcache;

        log::info!("Succesfully locked scheduler");
        Ok(direntry
            .get_directory()
            .expect("Direntry was not a directory.")
            .entries()
            .filter_map(|direntry_id| {
                // eprintln!("Trying to access child: {:?}", direntry_id);
                let direntry = dcache.get_entry(direntry_id).expect("WTF");
                //remove this unwrap
                let inode = inodes.get_mut(&direntry.inode_id);
                if let (ent, Some(inode)) = (direntry, inode) {
                    let mut entry = ent.clone();

                    if entry.is_directory() {
                        entry.get_directory_mut().unwrap().clear_entries();
                    }
                    Some((entry, inode.inode_data.clone(), inode.1()))
                } else {
                    None
                }
            })
            .try_collect()?)
    }
}
