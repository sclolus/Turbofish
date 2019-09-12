use super::drivers::{ipc::FifoDriver, DefaultDriver, Driver, Ext2DriverFile, FileOperation};
use super::sync::SmartMutex;
use super::thread_group::Credentials;
use super::{IpcResult, SysResult};

use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::convert::TryInto;
use fallible_collections::{btree::BTreeMap, FallibleArc, FallibleBox, TryCollect};
use lazy_static::lazy_static;
use sync::DeadMutex;

use fallible_collections::TryClone;
use itertools::unfold;

mod tools;
use tools::KeyGenerator;

mod path;
mod posix_consts;
use posix_consts::{NAME_MAX, SYMLOOP_MAX};

pub use path::{Filename, Path};

mod direntry;
pub use direntry::{DirectoryEntry, DirectoryEntryBuilder, DirectoryEntryId};

mod dcache;

use dcache::Dcache;

mod inode;
pub use inode::InodeId;
use inode::{Inode, InodeData};
use libc_binding::OpenFlags;

use libc_binding::c_char;
use libc_binding::dirent;
use libc_binding::statfs;
use libc_binding::Errno::*;
use libc_binding::FileType;
use libc_binding::{gid_t, stat, uid_t, Errno};

pub mod init;
pub use init::{init, VFS};

mod filesystem;
use filesystem::{DeadFileSystem, FileSystem, FileSystemId};

pub struct VirtualFileSystem {
    mounted_filesystems: BTreeMap<FileSystemId, Arc<DeadMutex<dyn FileSystem>>>,

    // superblocks: Vec<Superblock>,
    inodes: BTreeMap<InodeId, Inode>,
    dcache: Dcache,
}

use core::fmt::{self, Debug};

impl Debug for VirtualFileSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "VirtualFileSystem")
    }
}

#[allow(unused)]
type Vfs = VirtualFileSystem;

impl VirtualFileSystem {
    pub fn new() -> SysResult<VirtualFileSystem> {
        let mut new = Self {
            mounted_filesystems: BTreeMap::new(),
            inodes: BTreeMap::new(),
            dcache: Dcache::new(),
        };

        let root_inode = Inode::root_inode()?;
        let root_inode_id = root_inode.id;

        new.inodes.try_insert(root_inode_id, root_inode)?;
        Ok(new)
    }
    fn add_inode(&mut self, inode: Inode) -> SysResult<()> {
        if self.inodes.contains_key(&inode.get_id()) {
            // if it is not from an hard link we panic
            if inode.link_number == 1 {
                panic!("inode already there {:?}", inode);
            } else {
                // else we already put the inode pointed by the hard link
                return Ok(());
            }
        }
        self.inodes.try_insert(inode.get_id(), inode)?;
        Ok(())
    }

    fn get_filesystem(&mut self, inode_id: InodeId) -> Option<&Arc<DeadMutex<dyn FileSystem>>> {
        self.mounted_filesystems.get(&inode_id.filesystem_id?)
    }

    fn get_filesystem_mut(
        &mut self,
        inode_id: InodeId,
    ) -> Option<&mut Arc<DeadMutex<dyn FileSystem>>> {
        self.mounted_filesystems.get_mut(&inode_id.filesystem_id?)
    }

    fn add_entry_from_filesystem(
        &mut self,
        fs: Arc<DeadMutex<dyn FileSystem>>,
        parent: Option<DirectoryEntryId>,
        (direntry, inode_data): (DirectoryEntry, InodeData),
    ) -> SysResult<DirectoryEntryId> {
        let direntry = self.dcache.add_entry(parent, direntry)?;

        let inode = Inode::new(
            fs,
            if inode_data.is_fifo() {
                Box::try_new(FifoDriver::try_new(inode_data.id)?)?
            } else {
                // TODO: handle others drivers
                Box::try_new(Ext2DriverFile::new(inode_data.id))?
            },
            inode_data,
        );
        self.add_inode(inode)?;
        Ok(direntry)
    }

    /// construct the files in directory `direntry_id` in ram form the filesystem
    fn lookup_directory(&mut self, direntry_id: DirectoryEntryId) -> SysResult<()> {
        // unimplemented!()
        let current_entry = self.dcache.get_entry(&direntry_id)?;
        // dbg!(&current_entry);
        let inode_id = current_entry.inode_id;
        let fs_cloned = self
            .get_filesystem(inode_id)
            .expect("no filesystem")
            .clone();
        let iter = self
            .get_filesystem(inode_id)
            .expect("no filesystem")
            .lock()
            .lookup_directory(inode_id.inode_number as u32)?;

        for fs_entry in iter {
            self.add_entry_from_filesystem(fs_cloned.clone(), Some(direntry_id), fs_entry)
                .expect("add entry from filesystem failed");
        }
        Ok(())
    }

    /// Construct a path from a DirectoryEntryId by follow up its
    /// parent
    pub fn dentry_path(&self, id: DirectoryEntryId) -> SysResult<Path> {
        let mut rev_path = self.dcache.dentry_path(id)?;
        rev_path.set_absolute(true)?;
        Ok(rev_path)
    }

    /// resolve the path `pathname` from root `root`, return the
    /// directory_entry_id associate with the file, used for lstat
    pub fn pathname_resolution_no_follow_last_symlink(
        &mut self,
        cwd: &Path,
        pathname: &Path,
    ) -> SysResult<DirectoryEntryId> {
        let root = if pathname.is_absolute() {
            self.dcache.root_id
        } else {
            debug_assert!(cwd.is_absolute());
            self._pathname_resolution(self.dcache.root_id, cwd, 0, true)?
        };
        self._pathname_resolution(root, pathname, 0, false)
    }

    /// resolve the path `pathname` from root `root`, return the
    /// directory_entry_id associate with the file
    pub fn pathname_resolution(
        &mut self,
        cwd: &Path,
        pathname: &Path,
    ) -> SysResult<DirectoryEntryId> {
        let root = if pathname.is_absolute() {
            self.dcache.root_id
        } else {
            debug_assert!(cwd.is_absolute());
            self._pathname_resolution(self.dcache.root_id, cwd, 0, true)?
        };
        self._pathname_resolution(root, pathname, 0, true)
    }

    fn _pathname_resolution(
        &mut self,
        mut root: DirectoryEntryId,
        pathname: &Path,
        recursion_level: usize,
        follow_last_symlink: bool,
    ) -> SysResult<DirectoryEntryId> {
        if recursion_level > SYMLOOP_MAX {
            return Err(Errno::ELOOP);
        }

        if pathname.is_absolute() {
            root = self.dcache.root_id;
        }

        if !self.dcache.contains_entry(&root) {
            return Err(ENOENT);
        }

        let mut current_dir_id = root;
        let mut components = pathname.components();
        let mut was_symlink = false;
        let mut current_entry = self.dcache.get_entry(&current_dir_id)?;

        // quick fix, this handle / mount point
        if current_entry.is_mounted()? {
            current_dir_id = current_entry
                .get_mountpoint_entry()
                .expect("mount point entry should be there");
            current_entry = self.dcache.get_entry(&current_dir_id)?;
        }
        for component in components.by_ref() {
            if current_entry.is_mounted()? {
                current_dir_id = current_entry
                    .get_mountpoint_entry()
                    .expect("mount point entry should be there");
                current_entry = self.dcache.get_entry(&current_dir_id)?;
            }
            let current_dir = current_entry.get_directory()?;

            if component == &"." {
                continue;
            } else if component == &".." {
                current_dir_id = current_entry.parent_id;
                current_entry = self.dcache.get_entry(&current_dir_id)?;
                continue;
            }
            if current_dir.entries().count() == 0 {
                self.lookup_directory(current_dir_id)?;
                current_entry = self.dcache.get_entry(&current_dir_id)?;
                let current_dir = current_entry.get_directory()?;
                //TODO:
                let next_entry_id = current_dir
                    .entries()
                    .find(|x| {
                        let filename = &self
                            .dcache
                            .get_entry(x)
                            .expect("Invalid entry id in a directory entry that is a directory")
                            .filename;
                        filename == component
                    })
                    .ok_or(ENOENT)?;

                current_entry = self.dcache.get_entry(next_entry_id)?;
                if current_entry.is_symlink() {
                    was_symlink = true;
                    break;
                }
                current_dir_id = *next_entry_id;
                continue;
            }
            let next_entry_id = current_dir
                .entries()
                .find(|x| {
                    let filename = &self
                        .dcache
                        .get_entry(x)
                        .expect("Invalid entry id in a directory entry that is a directory")
                        .filename;
                    filename == component
                })
                .ok_or(ENOENT)?;

            current_entry = self.dcache.get_entry(next_entry_id)?;
            if current_entry.is_symlink() {
                was_symlink = true;
                break;
            }
            current_dir_id = *next_entry_id;
        }
        if was_symlink {
            if components.len() == 0 && !follow_last_symlink {
                return Ok(current_entry.id);
            }
            let mut new_path = current_entry
                .get_symbolic_content()
                .expect("should be symlink")
                .clone();
            new_path.chain(components.try_into()?)?;

            self._pathname_resolution(
                current_dir_id,
                &new_path,
                recursion_level + 1,
                follow_last_symlink,
            )
        } else {
            Ok(self.dcache.get_entry(&current_dir_id).unwrap().id)
        }
    }

    /// Ici j'enregistre un filename associe a son driver (que je
    /// provide depuis l'ipc)
    /// constrainte: Prototype, filename et Arc<DeadMutex<dyn Driver>>
    /// en param
    /// Je pense pas qu'il soit oblige d'envoyer un
    /// Arc<DeadMutes<...>> ici, une simple Box<dyn ...> pourrait
    /// faire l'affaire
    /// L'arc ca peut apporter un avantage pour gerer les liens
    /// symboliques en interne, mais c'est tout relatif
    /// Je te passe l'ownership complet du 'Driver'
    pub fn new_driver(
        &mut self,
        cwd: &Path,
        creds: &Credentials,
        path: Path,
        mode: FileType,
        driver: Box<dyn Driver>,
    ) -> SysResult<()> {
        // la fonction driver.set_inode_id() doit etre appele lors de la creation. C'est pour joindre l'inode au cas ou
        // Je ne sais pas encore si ce sera completement indispensable. Il vaut mieux que ce soit un type primitif afin
        // qu'il n'y ait pas de reference croisees (j'ai mis usize dans l'exemple)

        // let entry_id;
        match self.pathname_resolution(cwd, &path) {
            Ok(_id) => return Err(EEXIST),
            Err(_e) => {
                //TODO: Option(FileSystemId)
                let new_id = self.get_available_id(None);

                let mut inode_data: InodeData = Default::default();
                inode_data
                    .set_id(new_id)
                    .set_access_mode(mode)
                    .set_uid(creds.uid)
                    .set_gid(creds.gid); // posix does not really like this.

                inode_data.link_number += 1;

                let new_inode = Inode::new(
                    Arc::try_new(DeadMutex::new(DeadFileSystem))?,
                    driver,
                    inode_data,
                );

                let mut new_direntry = DirectoryEntry::default();
                let parent_id = self.pathname_resolution(cwd, &path.parent()?)?;

                new_direntry
                    .set_filename(*path.filename().unwrap())
                    .set_inode_id(new_id);

                new_direntry.set_regular();

                self.add_inode(new_inode)?;
                self.dcache
                    .add_entry(Some(parent_id), new_direntry)
                    /*CLEANUP*/
                    .map_err(|e| {
                        self.inodes.remove(&new_id);
                        e
                    })?;
            }
        }

        // let entry = self.dcache.get_entry(&entry_id)?;
        // let entry_inode_id = entry.inode_id;
        // let entry_id = entry.id;
        // self.open_inode(current, entry_inode_id, entry_id, flags);
        Ok(())
    }

    #[allow(dead_code)]
    fn iter_directory_entries(
        &self,
        dir: DirectoryEntryId,
    ) -> SysResult<impl Iterator<Item = &DirectoryEntry>> {
        let dir = self.dcache.get_entry(&dir)?.get_directory()?;

        let mut entries = dir.entries();
        Ok(unfold((), move |_| {
            if let Some(entry_id) = entries.next() {
                let entry = self
                    .dcache
                    .get_entry(&entry_id)
                    .expect("Some entries from this directory are corrupted");
                Some(entry)
            } else {
                None
            }
        }))
    }

    /// Returns the FileType of the file pointed by the Path `path`.
    pub fn file_type(&mut self, cwd: &Path, path: Path) -> SysResult<FileType> {
        let direntry_id = self.pathname_resolution(cwd, &path)?;
        let inode_id = &self
            .dcache
            .get_entry(&direntry_id)
            .expect("Dcache is corrupted: Could not find expected direntry")
            .inode_id;
        Ok(self
            .inodes
            .get(inode_id)
            .expect("Vfs Inodes are corrupted: Could not find expected inode")
            .access_mode)
    }

    // fn recursive_build_subtree(
    //     // This should be refactored with recursive_creat.
    //     &mut self,
    //     current_dir_id: DirectoryEntryId,
    //     fs_id: FileSystemId,
    // ) -> SysResult<()> {
    //     let direntry = self.dcache.get_entry(&current_dir_id)?;

    //     // Inode unexpectedly does not exists...
    //     let inode = self.inodes.get(&direntry.inode_id).ok_or(ENOENT)?;

    //     if !inode.is_directory() {
    //         return Ok(());
    //     }
    //     let entries = inode
    //         .inode_operations
    //         .lookup_entries
    //         .expect("Directory does not have lookup_entries() method")(&inode);

    //     for mut entry in entries {
    //         let fs = self.mounted_filesystems.get(&fs_id).unwrap(); // remove this unwrap

    //         entry.inode_id.filesystem_id = fs_id;

    //         let mut new_inode = fs.load_inode(entry.inode_id.inode_number).unwrap(); // fix this unwrap
    //         new_inode.id.filesystem_id = fs_id;
    //         let inode_id = new_inode.id;

    //         // clean up in error case (if any)
    //         let entry_id = self.dcache.add_entry(Some(current_dir_id), entry)?;
    //         let is_dir = new_inode.is_directory();
    //         self.inodes.insert(inode_id, new_inode).unwrap(); // fix this unwrap.
    //         if is_dir {
    //             self.recursive_build_subtree(entry_id, fs_id)?
    //         }
    //     }
    //     Ok(())
    // }
    /// Mount the filesystem `filesystem` with filesystem id `fs_id`
    /// on mount dir `mount_dir_id`
    pub fn mount_filesystem(
        &mut self,
        filesystem: Arc<DeadMutex<dyn FileSystem>>,
        fs_id: FileSystemId,
        mount_dir_id: DirectoryEntryId,
    ) -> SysResult<()> {
        let mount_dir = self.dcache.get_entry_mut(&mount_dir_id)?;
        if !mount_dir.is_directory() {
            return Err(ENOTDIR);
        }

        if mount_dir.is_mounted()? {
            return Err(EBUSY);
        }
        let (mut root_dentry, mut root_inode_data) = filesystem.lock().root()?;

        root_inode_data.id.filesystem_id = Some(fs_id);
        root_dentry.inode_id = root_inode_data.id;

        let root_dentry_id = self.add_entry_from_filesystem(
            filesystem.clone(),
            Some(mount_dir_id),
            (root_dentry, root_inode_data),
        )?;

        let mount_dir = self
            .dcache
            .get_entry_mut(&mount_dir_id)
            .expect("WTF: mount_dir_id should be valid");
        mount_dir
            .set_mounted(root_dentry_id)
            .expect("WTF: and should be a directory");

        self.mounted_filesystems.try_insert(fs_id, filesystem)?;
        //TODO: cleanup root dentry_id

        Ok(())
    }

    /// mount the source `source` on the target `target`
    pub fn mount(
        &mut self,
        cwd: &Path,
        creds: &Credentials,
        source: Path,
        target: Path,
    ) -> SysResult<()> {
        use crate::taskmaster::drivers::DiskWrapper;
        use ext2::Ext2Filesystem;
        use filesystem::Ext2fs;

        let flags = libc_binding::OpenFlags::O_RDWR;
        let mode = FileType::from_bits(0o777).expect("file permission creation failed");
        let file_operation = self
            .open(cwd, creds, source, flags, mode)
            .expect("open sda1 failed")
            .expect("disk driver open failed");

        let ext2_disk = DiskWrapper(file_operation);
        let ext2 =
            Ext2Filesystem::new(Box::try_new(ext2_disk)?).expect("ext2 filesystem new failed");
        let fs_id: FileSystemId = self.gen();

        // we handle only ext2 fs right now
        let filesystem = Ext2fs::new(ext2, fs_id);
        let mount_dir_id = self.pathname_resolution(cwd, &target)?;
        self.mount_filesystem(
            Arc::try_new(DeadMutex::new(filesystem))?,
            fs_id,
            mount_dir_id,
        )
    }

    pub fn opendir(
        &mut self,
        cwd: &Path,
        _creds: &Credentials,
        path: Path,
    ) -> SysResult<Vec<dirent>> {
        let entry_id = self.pathname_resolution(cwd, &path)?;
        let entry = self.dcache.get_entry(&entry_id)?;

        if entry.get_directory()?.entries().count() == 0 {
            self.lookup_directory(entry_id)?;
        }
        let direntry = self.dcache.get_entry(&entry_id)?;
        let dir = direntry.get_directory()?;
        Ok(dir
            .entries()
            .map(|e| {
                let child = self
                    .dcache
                    .get_entry(&e)
                    .expect("entry not found vfs is bullshit");
                child.dirent()
            })
            // recreate on the fly the . and .. file as it is not stocked
            // in the vfs
            .chain(Some(dirent {
                d_name: {
                    let mut name = [0; NAME_MAX + 1];
                    name[0] = '.' as c_char;
                    name
                },
                d_ino: direntry.inode_id.inode_number as u32,
            }))
            .chain(Some(dirent {
                d_name: {
                    let mut name = [0; NAME_MAX + 1];
                    name[0] = '.' as c_char;
                    name[1] = '.' as c_char;
                    name
                },
                d_ino: self
                    .dcache
                    .get_entry(&direntry.parent_id)
                    .unwrap()
                    .inode_id
                    .inode_number as u32,
            }))
            .try_collect()?)
    }

    pub fn unlink(&mut self, cwd: &Path, _creds: &Credentials, path: Path) -> SysResult<()> {
        let entry_id = self.pathname_resolution_no_follow_last_symlink(cwd, &path)?;
        let inode_id;
        let parent_id;

        {
            let entry = self.dcache.get_entry_mut(&entry_id)?;
            if entry.is_directory() {
                // unlink on directory not supported
                return Err(EISDIR);
            }
            inode_id = entry.inode_id;
            parent_id = entry.parent_id;
        }

        let corresponding_inode = self.inodes.get_mut(&inode_id).ok_or(ENOENT)?;
        self.dcache.remove_entry(entry_id)?;

        corresponding_inode.link_number -= 1;

        //TODO: VFS check that
        if corresponding_inode.link_number == 0 {
            self.inodes.remove(&inode_id).ok_or(ENOENT)?;
        }
        //TODO: check that
        let parent_inode_id = self.dcache.get_entry_mut(&parent_id)?.inode_id;
        let fs = self.get_filesystem(inode_id).expect("no filesystem");
        fs.lock().unlink(
            parent_inode_id.inode_number as u32,
            path.filename().expect("no filename").as_str(),
        )?;
        Ok(())
    }

    fn get_available_id(&self, filesystem_id: Option<FileSystemId>) -> InodeId {
        let mut current_id = InodeId::new(2, filesystem_id); // check this
        loop {
            if let None = self.inodes.get(&current_id) {
                return current_id;
            }

            // this is unchecked
            current_id = InodeId::new(current_id.inode_number + 1, filesystem_id);
        }
    }

    /// La fonction open() du vfs sera appelee par la fonction open()
    /// de l'ipc
    /// Elle doit logiquement renvoyer un FileOperation ou une erreur
    /// C'est le driver attache a l'inode qui se gere de retourner le
    /// bon FileOperation
    /// Open du driver doit etre appele
    /// constrainte: Prototype, filename en param et Arc<DeadMutex<dyn FileOperation>> en retour
    /// Ce sont les 'Driver' qui auront l'ownership des 'FileOperation'
    pub fn open(
        &mut self,
        cwd: &Path,
        _creds: &Credentials,
        path: Path,
        flags: OpenFlags,
        mode: FileType,
    ) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>> {
        let entry_id;
        match self.pathname_resolution(cwd, &path) {
            Ok(_id) if flags.contains(OpenFlags::O_CREAT | OpenFlags::O_EXCL) => {
                return Err(Errno::EEXIST)
            }
            Ok(id) => entry_id = id,
            Err(e) if !flags.contains(OpenFlags::O_CREAT) => return Err(e.into()),
            _ => {
                let parent_id = self.pathname_resolution(cwd, &path.parent()?)?;
                let parent_entry = self.dcache.get_entry(&parent_id)?;

                let inode_id = parent_entry.inode_id;
                let inode_number = inode_id.inode_number as u32;
                let fs = self.get_filesystem_mut(inode_id).expect("no filesystem");
                let fs_cloned = fs.clone();

                let fs_entry = fs.lock().create(
                    path.filename().expect("no filename").as_str(),
                    inode_number,
                    flags,
                    mode,
                )?;
                entry_id = self.add_entry_from_filesystem(fs_cloned, Some(parent_id), fs_entry)?;
            }
        }

        let entry = self.dcache.get_entry(&entry_id)?;
        let entry_inode_id = entry.inode_id;
        let entry_id = entry.id;
        if flags.contains(OpenFlags::O_DIRECTORY) && !entry.is_directory() {
            return Err(Errno::ENOTDIR);
        }
        self.open_inode(entry_inode_id, entry_id, flags)
    }

    fn open_inode(
        &mut self,
        inode_id: InodeId,
        _dentry_id: DirectoryEntryId,
        flags: OpenFlags,
    ) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>> {
        let inode = self.inodes.get_mut(&inode_id).ok_or(ENOENT)?;
        inode.driver.open(flags)
    }

    // pub fn creat(
    //     &mut self,
    //     current: &mut Current,
    //     path: Path,
    //     mode: FileType,
    // ) -> SysResult<Fd> {
    //     let mut flags = OpenFlags::O_WRONLY | OpenFlags::O_CREAT | OpenFlags::O_TRUNC;

    //     if mode.contains(FileType::S_IFDIR) {
    //         flags |= OpenFlags::O_DIRECTORY
    //     }

    //     // This effectively does not release fd.
    //     Ok(self.open(current, path, flags, mode)?)
    // }

    // pub fn recursive_creat(
    //     &mut self,
    //     current: &mut Current,
    //     path: Path,
    //     mode: FileType,
    // ) -> SysResult<Fd> {
    //     let mut ancestors = path.ancestors();

    //     let child = ancestors.next_back().ok_or(EINVAL)?;
    //     let ancestors = ancestors; //uncomment this
    //     for ancestor in ancestors {
    //         self.creat(current, ancestor, FileType::S_IFDIR)
    //             .unwrap(); // forget fd?
    //     }

    //     Ok(self.creat(current, child, mode)?)
    // }

    pub fn chmod(
        &mut self,
        cwd: &Path,
        _creds: &Credentials,
        path: Path,
        mut mode: FileType,
    ) -> SysResult<()> {
        let mask = FileType::SPECIAL_BITS | FileType::PERMISSIONS_MASK;
        mode &= mask;

        let entry_id = self.pathname_resolution(cwd, &path)?;
        let entry = self.dcache.get_entry(&entry_id)?;

        let inode_id = entry.inode_id;
        self.fchmod(inode_id, mode)
    }

    pub fn fchmod(&mut self, inode_id: InodeId, mut mode: FileType) -> SysResult<()> {
        let mask = FileType::SPECIAL_BITS | FileType::PERMISSIONS_MASK;
        mode &= mask;

        self.get_filesystem(inode_id)
            .expect("No corresponding filesystem")
            .lock()
            .chmod(inode_id.inode_number as u32, mode)?;

        let inode = self
            .inodes
            .get_mut(&inode_id)
            .expect("No corresponding inode for direntry");
        let mut new_mode = inode.access_mode;

        new_mode.remove(mask);
        new_mode.insert(mode);

        inode.set_access_mode(new_mode);
        Ok(())
    }

    pub fn chown(
        &mut self,
        cwd: &Path,
        _creds: &Credentials,
        path: Path,
        owner: uid_t,
        group: gid_t,
    ) -> SysResult<()> {
        let entry_id = self.pathname_resolution(cwd, &path)?;
        let entry = self.dcache.get_entry(&entry_id)?;

        let inode_id = entry.inode_id;
        let inode = self.inodes.get_mut(&entry.inode_id).ok_or(ENOENT)?;

        inode.set_uid(owner);
        inode.set_gid(group);

        let fs = self.get_filesystem(inode_id).expect("no filesystem");
        fs.lock()
            .chown(inode_id.inode_number as u32, owner, group)?;
        Ok(())
    }

    pub fn mkdir(
        &mut self,
        cwd: &Path,
        _creds: &Credentials,
        mut path: Path,
        mode: FileType,
    ) -> SysResult<()> {
        if let Ok(_) = self.pathname_resolution(cwd, &path) {
            return Err(EEXIST);
        }
        let filename = path.pop();
        let entry_id = self.pathname_resolution(cwd, &path)?;
        let entry = self.dcache.get_entry(&entry_id)?;
        if !entry.is_directory() {
            return Err(ENOTDIR);
        }
        let inode_id = entry.inode_id;

        let fs = self.get_filesystem(inode_id).expect("no filesystem");
        let fs_cloned = fs.clone();
        let fs_entry = fs.lock().create_dir(
            inode_id.inode_number,
            filename.expect("should have a filename").as_str(),
            mode,
        )?;
        self.add_entry_from_filesystem(fs_cloned, Some(entry_id), fs_entry)?;
        Ok(())
    }

    pub fn rmdir(&mut self, cwd: &Path, _creds: &Credentials, path: Path) -> SysResult<()> {
        let filename = path.filename().ok_or(EINVAL)?;
        if filename == &"." || filename == &".." {
            return Err(EINVAL);
        }

        let entry_id = self.pathname_resolution(cwd, &path)?;
        let entry = self.dcache.get_entry(&entry_id)?;

        if !entry.is_directory() {
            return Err(ENOTDIR);
        }
        if !entry.is_directory_empty()? {
            return Err(ENOTEMPTY);
        }
        let inode_id = entry.inode_id;
        let parent_id = entry.parent_id;

        self.dcache.remove_entry(entry_id)?;
        self.inodes.remove(&inode_id).expect("inode should be here");

        let parent_inode_id = self.dcache.get_entry_mut(&parent_id)?.inode_id;
        let fs = self.get_filesystem(inode_id).expect("no filesystem");
        fs.lock().rmdir(
            parent_inode_id.inode_number as u32,
            path.filename().expect("no filename").as_str(),
        )?;
        Ok(())
    }

    pub fn get_inode(&mut self, inode_id: InodeId) -> SysResult<&mut Inode> {
        self.inodes.get_mut(&inode_id).ok_or(ENOENT)
    }

    /// this implementation follow symbolic links
    pub fn link(
        &mut self,
        cwd: &Path,
        _creds: &Credentials,
        oldpath: Path,
        newpath: Path,
    ) -> SysResult<()> {
        let oldentry_id = self.pathname_resolution(cwd, &oldpath)?;
        let oldentry = self.dcache.get_entry(&oldentry_id)?;

        if oldentry.is_directory() {
            // link on directories not currently supported.
            return Err(EISDIR);
        }

        // works only on regular files
        if !oldentry.is_regular() {
            return Err(EINVAL);
        }

        if self.pathname_resolution(cwd, &newpath).is_ok() {
            return Err(EEXIST);
        }

        let parent_new_id = self.pathname_resolution(cwd, &newpath.parent()?)?;
        let parent_inode_id = self.dcache.get_entry_mut(&parent_new_id)?.inode_id;
        let parent_inode_number = parent_inode_id.inode_number;

        let oldentry = self.dcache.get_entry(&oldentry_id)?;

        let inode_id = oldentry.inode_id;
        let target_inode_number = inode_id.inode_number;

        let inode = self.inodes.get_mut(&oldentry.inode_id).ok_or(ENOENT)?;

        let filename = newpath.filename().ok_or(EINVAL)?;
        // let mut newentry = oldentry.clone();
        // newentry.filename = *newpath.filename().unwrap(); // remove this unwrap somehow.
        inode.link_number += 1;

        let fs = self.get_filesystem(inode_id).expect("no filesystem");

        let newentry =
            fs.lock()
                .link(parent_inode_number, target_inode_number, filename.as_str())?;
        // self.add_entry_from_filesystem(fs_cloned, Some(parent_new_id), fs_entry)?;
        self.dcache.add_entry(Some(parent_new_id), newentry)?;
        Ok(())
    }

    pub fn lstat(
        &mut self,
        cwd: &Path,
        _creds: &Credentials,
        path: Path,
        buf: &mut stat,
    ) -> SysResult<u32> {
        let entry_id = self.pathname_resolution_no_follow_last_symlink(cwd, &path)?;
        let entry = self.dcache.get_entry(&entry_id)?;
        let inode_id = entry.inode_id;
        let inode = self.get_inode(inode_id)?;
        inode.stat(buf)
    }

    pub fn readlink(
        &mut self,
        cwd: &Path,
        _creds: &Credentials,
        path: Path,
        buf: &mut [c_char],
    ) -> SysResult<u32> {
        let entry_id = self.pathname_resolution_no_follow_last_symlink(cwd, &path)?;
        let symbolic_content = self
            .dcache
            .get_entry(&entry_id)?
            .get_symbolic_content()
            .ok_or(EINVAL)?;

        let size = buf.len();
        let mut i = 0;
        for b in symbolic_content.iter_bytes() {
            // keep a place for the \0
            if i >= size - 1 {
                return Err(ERANGE);
            }
            buf[i] = *b;
            i += 1;
        }
        if i > 0 {
            buf[i - 1] = '\0' as c_char;
        }
        Ok(i as u32)
    }

    pub fn symlink(
        &mut self,
        cwd: &Path,
        _creds: &Credentials,
        target: &str,
        mut linkname: Path,
    ) -> SysResult<()> {
        if let Ok(_) = self.pathname_resolution(cwd, &linkname) {
            return Err(EEXIST);
        }
        let filename = linkname.pop().expect("no filename");
        let direntry_id = self.pathname_resolution(cwd, &linkname)?;
        let direntry = self.dcache.get_entry(&direntry_id)?;
        if !direntry.is_directory() {
            return Err(ENOENT);
        }

        let inode_id = direntry.inode_id;

        let parent_inode_id = self.dcache.get_entry_mut(&direntry_id)?.inode_id;
        let fs_cloned = self
            .get_filesystem(inode_id)
            .expect("no filesystem")
            .clone();
        let fs = self.get_filesystem(inode_id).expect("no filesystem");
        let fs_entry = fs.lock().symlink(
            parent_inode_id.inode_number as u32,
            target,
            filename.as_str(),
        )?;
        self.add_entry_from_filesystem(fs_cloned.clone(), Some(direntry_id), fs_entry)
            .expect("add entry from filesystem failed");
        Ok(())
    }

    pub fn resolve_path(&mut self, cwd: &Path, path: &Path) -> SysResult<Path> {
        let direntry_id = self.pathname_resolution(cwd, &path)?;
        self.dentry_path(direntry_id)
    }

    pub fn rename(
        &mut self,
        cwd: &Path,
        creds: &Credentials,
        oldpath: Path,
        newpath: Path,
    ) -> SysResult<()> {
        // If either pathname argument refers to a path whose final
        // component is either dot or dot-dot, rename() shall fail.
        let old_filename = oldpath.filename().ok_or(EINVAL)?;

        if old_filename == &"." || old_filename == &".." {
            return Err(Errno::EINVAL);
        }
        let new_filename = newpath.filename().ok_or(EINVAL)?;

        if new_filename == &"." || new_filename == &".." {
            return Err(Errno::EINVAL);
        }

        let oldentry_id = self.pathname_resolution_no_follow_last_symlink(cwd, &oldpath)?;
        // The old pathname shall not name an ancestor directory of
        // the new pathname.
        let resolved_old_path = self.resolve_path(cwd, &oldpath)?;
        let mut resolved_new_path = self.resolve_path(cwd, &newpath.parent()?)?;
        resolved_new_path.push(*new_filename)?;
        if resolved_new_path
            .ancestors()
            .any(|p| p == resolved_old_path)
        {
            return Err(Errno::EINVAL);
        }

        match self.pathname_resolution_no_follow_last_symlink(cwd, &newpath) {
            Ok(new_entry_id) => {
                let new_entry = self.dcache.get_entry(&new_entry_id)?;

                let oldentry = self.dcache.get_entry(&oldentry_id)?;
                if oldentry.is_directory()
                    && (!new_entry.is_directory() || !new_entry.is_directory_empty()?)
                {
                    // If the old argument points to the pathname of a
                    // directory, the new argument shall not point to the
                    // pathname of a file that is not a directory, it
                    // shall be required to be an empty directory.
                    return Err(Errno::EEXIST);
                } else if !oldentry.is_directory() && new_entry.is_directory() {
                    // If the old argument points to the pathname of a
                    // file that is not a directory, the new argument
                    // shall not point to the pathname of a directory
                    return Err(Errno::EISDIR);
                }
                // If the old argument and the new argument resolve to
                // either the same existing directory entry or
                // different directory entries for the same existing
                // file, rename() shall return successfully and
                // perform no other action.
                if new_entry.inode_id.inode_number == oldentry.inode_id.inode_number {
                    return Ok(());
                }

                if new_entry.is_directory() {
                    self.rmdir(cwd, creds, newpath.try_clone()?)?;
                } else {
                    self.unlink(cwd, creds, newpath.try_clone()?)?;
                }
            }
            Err(_) => {}
        }
        // newpath does not exist in either case

        let oldentry = self.dcache.get_entry(&oldentry_id)?;
        let old_parent_id = oldentry.parent_id;
        let old_parent_inode_id = self.dcache.get_entry_mut(&old_parent_id)?.inode_id;
        let old_parent_inode_nbr = old_parent_inode_id.inode_number;

        let new_parent_id = self.pathname_resolution(cwd, &newpath.parent()?)?;
        let new_parent_inode_id = self.dcache.get_entry_mut(&new_parent_id)?.inode_id;
        let new_parent_inode_nbr = new_parent_inode_id.inode_number;

        let fs = self
            .get_filesystem(old_parent_inode_id)
            .expect("no filesystem");

        fs.lock().rename(
            old_parent_inode_nbr,
            old_filename.as_str(),
            new_parent_inode_nbr,
            new_filename.as_str(),
        )?;

        let oldentry_id = self.dcache.move_dentry(oldentry_id, new_parent_id)?;

        let entry = self
            .dcache
            .d_entries
            .get_mut(&oldentry_id)
            .expect("oldentry sould be there");

        entry.set_filename(*new_filename);
        Ok(())
    }

    pub fn statfs(
        &mut self,
        cwd: &Path,
        _creds: &Credentials,
        path: Path,
        buf: &mut statfs,
    ) -> SysResult<()> {
        let direntry_id = self
            .pathname_resolution(cwd, &path)
            .or(Err(Errno::ENOENT))?;
        let inode_id = {
            self.dcache
                .get_entry(&direntry_id)
                .expect("No corresponding inode for direntry")
                .inode_id
        };

        self.fstatfs(inode_id, buf)
    }

    pub fn fstatfs(&self, inode_id: InodeId, buf: &mut statfs) -> SysResult<()> {
        let fs_id = &inode_id.filesystem_id.ok_or(Errno::ENOSYS)?; // really not sure about that.
        let fs = self
            .mounted_filesystems
            .get(fs_id)
            .expect("No filesystem match the filesystem_id from an InodeId");

        fs.lock().statfs(buf)
    }
}

// pub type VfsHandler<T> = fn(VfsHandlerParams) -> SysResult<T>;

// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// pub enum VfsHandlerKind {
//     Open,
//     LookupInode,
//     LookupEntries,
//     Creat,
//     Rename,
//     Chmod,
//     Chown,
//     Lchown,
//     Truncate,
//     TestOpen,
// }
// // #[derive(Debug, Clone, Default)]
// #[derive(Default)]
// pub struct VfsHandlerParams<'a> {
//     inode: Option<&'a Inode>,
//     file: Option<&'a File>,
//     path: Option<&'a Path>,
// }

// impl<'a> VfsHandlerParams<'a> {
//     pub fn new() -> Self {
//         Self::default()
//     }

//     pub fn set_inode(mut self, inode: &'a Inode) -> Self {
//         self.inode = Some(inode);
//         self
//     }

//     pub fn set_file(mut self, file: &'a File) -> Self {
//         self.file = Some(file);
//         self
//     }

//     pub fn set_path(mut self, path: &'a Path) -> Self {
//         self.path = Some(path);
//         self
//     }

//     pub fn unset_inode(mut self) -> Self {
//         self.inode = None;
//         self
//     }

//     pub fn unset_file(mut self) -> Self {
//         self.file = None;
//         self
//     }

//     pub fn unset_path(mut self) -> Self {
//         self.path = None;
//         self
//     }
// }

impl KeyGenerator<FileSystemId> for VirtualFileSystem {
    fn gen_filter(&self, id: FileSystemId) -> bool {
        !self.mounted_filesystems.contains_key(&id)
    }
}

// impl Mapper<FileSystemId, Arc<DeadMutex<dyn FileSystem>>> for VirtualFileSystem {
//     fn get_map(&mut self) -> &mut BTreeMap<FileSystemId, Arc<DeadMutex<dyn FileSystem>>> {
//         &mut self.mounted_filesystems
//     }
// }

#[cfg(test)]
mod vfs {

    use super::*;
    // rename this
    macro_rules! make_test {
        ($body: expr, $name: ident) => {
            #[test]
            fn $name() {
                $body
            }
        };
        (failing, $body: expr, $name: ident) => {
            #[test]
            #[should_panic]
            fn $name() {
                $body
            }
        };
    }

    macro_rules! vfs_test {
        ($body: block, $name: ident) => {
            make_test! {$body, $name}
        };
        (failing, $body: block, $name: ident) => {
            make_test! {failing, $body, $name}
        };
    }

    // macro_rules! vfs_file_exists_test {
    //     ($body: block, $path: expr, $name: ident) => {
    //         make_test! {{
    //             let mut vfs = Vfs::new().unwrap();
    //             let mut current = default_current();
    //             let path: &str = $path;
    //             let path: Path = std::convert::TryInto::try_into(path).unwrap();

    //             if path != std::convert::TryInto::try_into("/").unwrap() {
    //                 vfs.recursive_creat(&mut current, path.clone(), FileType::S_IRWXU).unwrap();
    //             }
    //             assert!(vfs.file_exists(&current, path).unwrap())
    //         }, $name}
    //     };
    //     (failing, $body: block, $path: expr, $name: ident) => {
    //         make_test! {failing, {
    //             let mut vfs = Vfs::new().unwrap();
    //             let mut current = default_current();
    //             let path: &str = $path;
    //             let path: Path = std::convert::TryInto::try_into(path).unwrap();

    //             if path != std::convert::TryInto::try_into("/").unwrap() {
    //                 vfs.recursive_creat(&mut current, path.clone(), FileType::S_IRWXU).unwrap();
    //             }
    //             assert!(vfs.file_exists(&current, path).unwrap())
    //         }, $name}
    //     };
    // }

    // vfs_file_exists_test! {{}, "/", file_exists_root}
    // vfs_file_exists_test! {failing, {}, "", file_exists_null}
    // vfs_file_exists_test! {{
    // }, "a", file_exists_basic_a}
    // vfs_file_exists_test! {{
    // }, "/a", file_exists_basic_root_a}

    // vfs_file_exists_test! {{
    // }, "a/b", file_exists_basic_a_b}
    // vfs_file_exists_test! {{
    // }, "a/b/c", file_exists_basic_a_b_c}
    // vfs_file_exists_test! {{
    // }, "a/b/c/d", file_exists_basic_a_b_c_d}
    // vfs_file_exists_test! {{
    // }, "a/b/c/d/e/f", file_exists_basic_a_b_c_d_e_f}

    // vfs_file_exists_test! {{
    // }, "/a/b", file_exists_basic_root_a_b}
    // vfs_file_exists_test! {{
    // }, "/a/b/c", file_exists_basic_root_a_b_c}
    // vfs_file_exists_test! {{
    // }, "/a/b/c/d", file_exists_basic_root_a_b_c_d}
    // vfs_file_exists_test! {{
    // }, "/a/b/c/d/e/f", file_exists_basic_root_a_b_c_d_e_f}

    // macro_rules! vfs_recursive_creat_test {
    //     ($path: expr, $name: ident) => {
    //         make_test! {{
    //             let mut vfs = Vfs::new().unwrap();
    //             let mut current = default_current();
    //             let path: &str = $path;
    //             let path: Path = std::convert::TryInto::try_into(path).unwrap();

    //             vfs.recursive_creat(&mut current
    //                                 , path.clone()
    //                                 , FileType::S_IRWXU).unwrap();
    //             assert!(vfs.file_exists(&current, path).unwrap())
    //         }, $name}
    //     };
    //     (failing, $path: expr, $name: ident) => {
    //         make_test! {failing, {
    //             let mut vfs = Vfs::new().unwrap();
    //             let mut current = default_current();
    //             let path: &str = $path;
    //             let path: Path = path.try_into().unwrap();

    //             vfs.recursive_creat(&mut current
    //                                 , path.clone()
    //                                 , FileType::S_IRWXU).unwrap();
    //             for ancestors in path.ancestors() {
    //                 assert!(vfs.file_exists(&current, ancestor).unwrap())
    //             }
    //         }, $name}
    //     };
    // }

    // vfs_recursive_creat_test! {"a/b/c/d/e/f/g", recursive_creat_a_b_c_d_e_f_g}
    // vfs_recursive_creat_test! {"a/b/c/d/e/f  ", recursive_creat_a_b_c_d_e_f}
    // vfs_recursive_creat_test! {"a/b/c/d/e    ", recursive_creat_a_b_c_d_e}
    // vfs_recursive_creat_test! {"a/b/c/d      ", recursive_creat_a_b_c_d}
    // vfs_recursive_creat_test! {"a/b/c        ", recursive_creat_a_b_c}
    // vfs_recursive_creat_test! {"a/b          ", recursive_creat_a_b} // infinite loop
    // vfs_recursive_creat_test! {"a            ", recursive_creat_a}
}
