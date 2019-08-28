#![feature(underscore_const_names)]
#![feature(type_ascription)]
#![feature(try_trait)]
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use core::cmp::{Eq, Ord, PartialEq, PartialOrd};
use core::str::FromStr;

#[macro_use]
extern crate itertools;

use itertools::unfold;

mod path;
mod posix_consts;
use path::{Filename, Path};

mod direntry;
use direntry::{DirectoryEntry, DirectoryEntryId};

mod dcache;

use dcache::{Dcache, DcacheError, DcacheResult};

mod inode;
use inode::{File, Inode, InodeId, InodeNumber, Offset, OpenFlags, SeekType};

mod user;
use user::{Current, GroupId, UserId};

mod fildes;

use libc_binding::Errno;
use Errno::*;

mod permissions;
use permissions::FilePermissions;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VfsError {
    FileAlreadyExists,
    NoSuchEntry,
    NotADirectory,
    NotASymlink,
    InvalidEntryIdInDirectory,
    RootDoesNotExists,
    NotEmpty,
    EntryNotConnected,
    NotEnoughArguments,
    DirectoryNotMounted,
    DirectoryIsMounted,
    UndefinedHandler,

    MountError,
    NoSuchInode,
    InodeAlreadyExists,
    Errno(Errno),
}

use VfsError::*;

pub type VfsResult<T> = Result<T, VfsError>;

impl From<DcacheError> for VfsError {
    fn from(value: DcacheError) -> Self {
        match value {
            DcacheError::FileAlreadyExists => VfsError::FileAlreadyExists,
            DcacheError::NoSuchEntry => VfsError::NoSuchEntry,
            DcacheError::NotADirectory => VfsError::NotADirectory,
            DcacheError::NotASymlink => VfsError::NotASymlink,
            DcacheError::InvalidEntryIdInDirectory => VfsError::InvalidEntryIdInDirectory,
            DcacheError::RootDoesNotExists => VfsError::RootDoesNotExists,
            DcacheError::NotEmpty => VfsError::NotEmpty,
            DcacheError::EntryNotConnected => VfsError::EntryNotConnected,
            DcacheError::NotEnoughArguments => VfsError::NotEnoughArguments,
            DcacheError::DirectoryNotMounted => VfsError::DirectoryNotMounted,
            DcacheError::DirectoryIsMounted => VfsError::DirectoryIsMounted,
            DcacheError::Errno(errno) => VfsError::Errno(errno),
        }
    }
}

impl From<Errno> for VfsError {
    fn from(value: Errno) -> Self {
        VfsError::Errno(value)
    }
}

impl From<VfsError> for core::option::NoneError {
    fn from(_value: VfsError) -> Self {
        core::option::NoneError
    }
}

pub struct SuperblockOperations {
    lookup: Option<fn(&mut Superblock)>,
    create: Option<fn(&mut Superblock)>,
    unlink: Option<fn(&mut Superblock)>,
    link: Option<fn(&mut Superblock)>,
    symlink: Option<fn(&mut Superblock)>,
    statfs: Option<fn(&mut Superblock)>,
    mkdir: Option<fn(&mut Superblock)>,
    rmdir: Option<fn(&mut Superblock)>,
}

pub struct Superblock {
    // filesystem_type: FileSystemType,
    operations: SuperblockOperations,
}

pub struct StandardFileSystem {}

impl StandardFileSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl FileSystem for StandardFileSystem {
    fn name(&self) -> &str {
        "StandardFileSystem"
    }

    fn get_superblock(&self) -> Superblock {
        let operations = SuperblockOperations {
            lookup: None,
            create: None,
            unlink: None,
            link: None,
            symlink: None,
            statfs: None,
            mkdir: None,
            rmdir: None,
        };

        Superblock { operations }
    }

    fn root_dentry(&self) -> DirectoryEntry {
        unimplemented!()
    }

    fn root_inode(&self) -> Inode {
        unimplemented!()
    }

    fn load_inode(&self, inode_number: InodeNumber) -> VfsResult<Inode> {
        unimplemented!()
    }
}

pub trait FileSystem {
    fn name(&self) -> &str;
    fn get_superblock(&self) -> Superblock;
    fn root_dentry(&self) -> DirectoryEntry;
    fn root_inode(&self) -> Inode;
    fn load_inode(&self, inode_number: InodeNumber) -> VfsResult<Inode>;
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Default, Eq, PartialEq)]
pub struct FileSystemId(usize);

impl FileSystemId {
    fn new(id: usize) -> Self {
        Self(id)
    }
}

impl core::ops::Add<usize> for FileSystemId {
    type Output = Self;
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

pub type VfsHandler<T> = fn(VfsHandlerParams) -> VfsResult<T>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum VfsHandlerKind {
    Open,
    LookupInode,
    LookupEntries,
    Creat,
    Rename,
    Chmod,
    Chown,
    Lchown,
    Truncate,
    TestOpen,
}

// #[derive(Debug, Clone, Default)]
#[derive(Default)]
pub struct VfsHandlerParams<'a> {
    inode: Option<&'a Inode>,
    file: Option<&'a File>,
    path: Option<&'a Path>,
}

impl<'a> VfsHandlerParams<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_inode(mut self, inode: &'a Inode) -> Self {
        self.inode = Some(inode);
        self
    }

    pub fn set_file(mut self, file: &'a File) -> Self {
        self.file = Some(file);
        self
    }

    pub fn set_path(mut self, path: &'a Path) -> Self {
        self.path = Some(path);
        self
    }

    pub fn unset_inode(mut self) -> Self {
        self.inode = None;
        self
    }

    pub fn unset_file(mut self) -> Self {
        self.file = None;
        self
    }

    pub fn unset_path(mut self) -> Self {
        self.path = None;
        self
    }
}

pub struct VirtualFileSystem {
    mounted_filesystems: BTreeMap<FileSystemId, Box<FileSystem>>,

    // superblocks: Vec<Superblock>,
    inodes: BTreeMap<InodeId, Inode>,
    dcache: Dcache,
    open_file_descriptions: BTreeMap<OFDId, File>,
}

impl KeyGenerator<FileSystemId> for VirtualFileSystem {
    fn gen_filter(&self, id: FileSystemId) -> bool {
        !self.mounted_filesystems.contains_key(&id)
    }
}

impl Mapper<FileSystemId, Box<FileSystem>> for VirtualFileSystem {
    fn get_map(&mut self) -> &mut BTreeMap<FileSystemId, Box<FileSystem>> {
        &mut self.mounted_filesystems
    }
}

use fildes::{Fd, Fildes, KeyGenerator, Mapper, MapperResult, OFDId};
impl KeyGenerator<OFDId> for VirtualFileSystem {
    fn gen_filter(&self, fd: Fd) -> bool {
        !self.open_file_descriptions.contains_key(&fd)
    }
}

impl Mapper<OFDId, File> for VirtualFileSystem {
    fn get_map(&mut self) -> &mut BTreeMap<OFDId, File> {
        &mut self.open_file_descriptions
    }
}

type Vfs = VirtualFileSystem;

impl VirtualFileSystem {
    pub fn new() -> VfsResult<VirtualFileSystem> {
        let mut new = Self {
            mounted_filesystems: BTreeMap::new(),
            inodes: BTreeMap::new(),
            dcache: Dcache::new(),
            open_file_descriptions: BTreeMap::new(),
        };

        let root_inode = Inode::root_inode();
        let root_inode_id = root_inode.id;

        new.inodes.insert(root_inode_id, root_inode);
        Ok(new)
    }

    fn iter_directory_entries(
        &self,
        dir: DirectoryEntryId,
    ) -> VfsResult<impl Iterator<Item = &DirectoryEntry>> {
        let dir = self.dcache.get_entry(&dir)?.get_directory()?;

        let mut entries = dir.entries().iter();
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

    fn recursive_build_subtree(
        &mut self,
        current_dir_id: DirectoryEntryId,
        fs_id: FileSystemId,
    ) -> VfsResult<()> {
        let direntry = self.dcache.get_entry(&current_dir_id)?;

        // Inode unexpectedly does not exists...
        let inode = self.inodes.get(&direntry.inode_id).ok_or(NoSuchInode)?;

        if !inode.is_directory() {
            return Ok(());
        }
        let entries = inode
            .inode_operations
            .lookup_entries
            .expect("Directory does not have lookup_entries() method")(&inode);

        for mut entry in entries {
            let fs = self.mounted_filesystems.get(&fs_id).unwrap(); // remove this unwrap

            entry.inode_id.filesystem_id = fs_id;

            let mut new_inode = fs.load_inode(entry.inode_id.inode_number).unwrap(); // fix this unwrap
            new_inode.id.filesystem_id = fs_id;
            let inode_id = new_inode.id;

            // clean up in error case (if any)
            let entry_id = self.dcache.add_entry(Some(current_dir_id), entry)?;
            let is_dir = new_inode.is_directory();
            self.inodes.insert(inode_id, new_inode).unwrap(); // fix this unwrap.
            if is_dir {
                self.recursive_build_subtree(entry_id, fs_id)?
            }
        }
        Ok(())
    }

    pub fn mount_filesystem(
        &mut self,
        // current: &mut Current,
        mount_dir_id: DirectoryEntryId,
        filesystem: Box<FileSystem>,
    ) -> VfsResult<FileSystemId> {
        let mount_dir = self.dcache.get_entry_mut(&mount_dir_id)?;
        if !mount_dir.is_directory() {
            return Err(NotADirectory);
        }

        if mount_dir.is_mounted()? {
            return Err(DirectoryIsMounted);
        }
        let mut root_dentry = filesystem.root_dentry();
        let mut root_inode = filesystem.root_inode();

        // So much to undo if any of this fails...
        let fs_id = self.add_entry(filesystem).unwrap(); // this

        root_inode.id.filesystem_id = fs_id;

        root_dentry.inode_id = root_inode.id;

        let root_dentry_id = self.dcache.add_entry(Some(mount_dir_id), root_dentry)?;
        let mount_dir = self.dcache.get_entry_mut(&mount_dir_id)?;
        mount_dir.set_mounted(root_dentry_id);

        let root_inode_id = root_inode.id;
        self.inodes.insert(root_inode_id, root_inode);

        self.recursive_build_subtree(root_dentry_id, fs_id)?;
        Ok(fs_id)
    }

    pub fn unlink(&mut self, current: &mut Current, path: Path) -> VfsResult<()> {
        use VfsError::*;
        let entry_id = self.dcache.pathname_resolution(current.cwd, path)?;
        let inode_id;

        {
            let entry = self.dcache.get_entry_mut(&entry_id)?;
            inode_id = entry.inode_id;
        }

        let corresponding_inode = self.inodes.get_mut(&inode_id).ok_or(NoSuchInode)?;
        self.dcache.remove_entry(entry_id)?;

        corresponding_inode.link_number -= 1;

        if corresponding_inode.link_number == 0 && !corresponding_inode.is_opened() {
            self.inodes.remove(&inode_id).ok_or(NoSuchInode)?;
        }
        Ok(())
    }

    fn get_available_id(&self, filesystem_id: FileSystemId) -> InodeId {
        let mut current_id = InodeId::new(2, filesystem_id); // check this
        loop {
            if let None = self.inodes.get(&current_id) {
                return current_id;
            }

            // this is unchecked
            current_id = InodeId::new(current_id.inode_number + 1, filesystem_id);
        }
    }

    pub fn open(
        &mut self,
        current: &mut Current,
        path: Path,
        flags: OpenFlags,
        mode: FilePermissions,
    ) -> VfsResult<Fd> {
        let entry_id;
        match self.dcache.pathname_resolution(current.cwd, path.clone()) {
            Ok(id) if flags.contains(OpenFlags::O_CREAT | OpenFlags::O_EXCL) => {
                return Err(Errno(Errno::Eexist))
            }
            Ok(id) => entry_id = id,
            Err(e) if !flags.contains(OpenFlags::O_CREAT) => return Err(e.into()),
            _ => {
                let mut new_inode = Inode::default();
                let new_id = self.get_available_id(FileSystemId::new(0)); // THIS IS FALSE

                new_inode
                    .set_id(new_id)
                    .set_access_mode(mode)
                    .set_uid(current.uid)
                    .set_gid(current.gid); // posix does not really like this.

                new_inode.link_number += 1;
                assert!(self.inodes.insert(new_id, new_inode).is_none());
                let mut new_direntry = DirectoryEntry::default();
                let parent_id = self
                    .dcache
                    .pathname_resolution(current.cwd, path.parent())?;

                new_direntry
                    .set_filename(*path.filename().unwrap())
                    .set_inode_id(new_id);

                if flags.contains(OpenFlags::O_DIRECTORY) {
                    new_direntry.set_directory();
                } else {
                    new_direntry.set_regular();
                }

                entry_id = self.dcache.add_entry(Some(parent_id), new_direntry)?;
            }
        }

        let entry = self.dcache.get_entry(&entry_id)?;
        if flags.contains(OpenFlags::O_DIRECTORY) && !entry.is_directory() {
            return Err(NotADirectory);
        }
        self.open_inode(current, entry.inode_id, entry.id, flags)
    }

    fn open_inode(
        &mut self,
        current: &mut Current,
        inode_id: InodeId,
        dentry_id: DirectoryEntryId,
        flags: OpenFlags,
    ) -> VfsResult<Fd> {
        let inode = self.inodes.get_mut(&inode_id).ok_or(NoSuchInode)?;
        let offset = if flags.contains(OpenFlags::O_APPEND) {
            inode.size
        } else {
            0
        };

        inode.open();
        let ofd = File {
            id: inode_id,
            dentry_id,
            offset,
            flags,
        };

        let ofd_id = self.add_entry(ofd).unwrap(); // remove this unwrap

        let fildes = Fildes::new(ofd_id);
        Ok(current.add_fd(fildes).unwrap()) // remove this_unwrap
    }

    pub fn creat(
        &mut self,
        current: &mut Current,
        path: Path,
        mode: FilePermissions,
    ) -> VfsResult<Fd> {
        let mut flags = OpenFlags::O_WRONLY | OpenFlags::O_CREAT | OpenFlags::O_TRUNC;

        if mode.contains(FilePermissions::S_IFDIR) {
            flags |= OpenFlags::O_DIRECTORY
        }

        // This effectively does not release fd.
        Ok(self.open(current, path, flags, mode)?)
    }

    pub fn recursive_creat(
        &mut self,
        current: &mut Current,
        path: Path,
        mode: FilePermissions,
    ) -> VfsResult<Fd> {
        let mut ancestors = path.ancestors();

        let child = ancestors.next_back().ok_or(Errno(Einval))?;
        let mut ancestors = ancestors; //uncomment this
        for ancestor in ancestors {
            self.creat(current, ancestor, FilePermissions::S_IFDIR)
                .unwrap(); // forget fd?
        }

        Ok(self.creat(current, child, mode)?)
    }

    pub fn chmod(
        &mut self,
        current: &mut Current,
        path: Path,
        mode: FilePermissions,
    ) -> VfsResult<()> {
        let entry_id = self.dcache.pathname_resolution(current.cwd, path)?;

        let entry = self.dcache.get_entry(&entry_id)?;

        let inode = self.inodes.get_mut(&entry.inode_id).ok_or(NoSuchInode)?;

        inode.set_access_mode(mode);
        Ok(())
    }

    pub fn chown(
        &mut self,
        current: &mut Current,
        path: Path,
        owner: UserId,
        group: GroupId,
    ) -> VfsResult<()> {
        let entry_id = self.dcache.pathname_resolution(current.cwd, path)?;

        let entry = self.dcache.get_entry(&entry_id)?;

        let inode = self.inodes.get_mut(&entry.inode_id).ok_or(NoSuchInode)?;

        inode.set_uid(owner);
        inode.set_gid(group);
        Ok(())
    }

    pub fn mkdir(
        &mut self,
        current: &mut Current,
        path: Path,
        mode: FilePermissions,
    ) -> VfsResult<()> {
        let flags = OpenFlags::O_DIRECTORY | OpenFlags::O_CREAT;

        self.open(current, path, flags, mode)?;
        Ok(())
    }

    pub fn rmdir(&mut self, current: &mut Current, path: Path) -> VfsResult<()> {
        let filename = path.filename().ok_or(Errno(Einval))?;
        if filename == &"." || filename == &".." {
            return Err(Errno(Einval));
        }

        let entry_id = self.dcache.pathname_resolution(current.cwd, path.clone())?;
        let entry = self.dcache.get_entry(&entry_id)?;

        if !entry.is_directory() {
            return Err(NotADirectory);
        }
        self.unlink(current, path)
    }

    pub fn link(&mut self, current: &mut Current, oldpath: Path, newpath: Path) -> VfsResult<()> {
        let oldentry_id = self.dcache.pathname_resolution(current.cwd, oldpath)?;
        let oldentry = self.dcache.get_entry(&oldentry_id)?;

        if oldentry.is_directory() {
            //link on directories not currently supported.
            return Err(Errno(Eisdir));
        }

        if self
            .dcache
            .pathname_resolution(current.cwd, newpath.clone())
            .is_ok()
        {
            return Err(Errno(Eexist));
        }

        let parent_new = self
            .dcache
            .pathname_resolution(current.cwd, newpath.parent())?;

        let inode = self.inodes.get_mut(&oldentry.inode_id).ok_or(NoSuchInode)?;

        let mut newentry = oldentry.clone();

        newentry.filename = *newpath.filename().unwrap(); // remove this unwrap somehow.
        self.dcache.add_entry(Some(parent_new), newentry)?;
        inode.link_number += 1;
        Ok(())
    }

    pub fn rename(&mut self, current: &mut Current, oldpath: Path, newpath: Path) -> VfsResult<()> {
        let oldentry_id = self.dcache.pathname_resolution(current.cwd, oldpath)?;

        self.dcache
            .rename_dentry(current.cwd, oldentry_id, newpath)?;
        Ok(())
    }

    pub fn read(&mut self, current: &mut Current, fd: Fd, buf: &mut [u8]) -> VfsResult<usize> {
        Ok(0)
    }

    pub fn write(&mut self, current: &mut Current, fd: Fd, buf: &mut [u8]) -> VfsResult<usize> {
        Ok(0)
    }

    pub fn lseek(
        &mut self,
        current: &mut Current,
        fd: Fd,
        offset: Offset,
        seek: SeekType,
    ) -> VfsResult<Offset> {
        Ok(0)
    }

    pub fn file_exists(&self, current: &Current, path: Path) -> VfsResult<bool> {
        self.dcache.pathname_resolution(current.cwd, path).unwrap();
        Ok(true)
    }
}

use std::convert::{TryFrom, TryInto};
use std::fs::{read_dir, DirEntry, FileType};
use std::os::unix::fs::PermissionsExt;
use std::path::Path as StdPath;
use walkdir::WalkDir;
// fn main() {
//     use std::env;
//     let mut vfs = Vfs::new().unwrap();

//     let mut args = env::args().skip(1);
//     let mut current =
//         Current { cwd: DirectoryEntryId::new(2), uid: 0, euid: 0, gid: 0, egid: 0, open_fds: BTreeMap::new() };

//     fn construct_tree(vfs: &mut Vfs, current: &mut Current, root: &StdPath, current_path: Path) {
//         let mut iter = read_dir(root).unwrap().filter_map(|e| e.ok());

//         for entry in iter {
//             let filename = Filename::try_from(entry.file_name().to_str().unwrap()).unwrap();
//             let mut path = current_path.clone();

//             path.push(filename).unwrap();
//             // let mut new = DirectoryEntry::default();

//             // new.set_filename();
//             // new.set_id(get_available_directory_entry_id());
//             let filetype = entry.file_type().unwrap();
//             let mode = unsafe { FilePermissions::from_u32(entry.metadata().unwrap().permissions().mode()) };

//             let mut flags = OpenFlags::O_CREAT;

//             if filetype.is_dir() {
//                 flags |= OpenFlags::O_DIRECTORY;
//             } else if filetype.is_symlink() {
//                 // let std_path = std::fs::read_link(entry.path()).unwrap();
//                 // let path = std_path.as_os_str().to_str().unwrap().try_into().unwrap();
//                 // new.set_symlink(path);
//             }

//             // println!("{}", path);
//             vfs.open(current, path.clone(), flags, mode).unwrap();
//             if entry.file_type().unwrap().is_dir() {
//                 construct_tree(vfs, current, &entry.path(), path);
//             }
//         }
//     }

//     let path = args.next().unwrap();

//     construct_tree(&mut vfs, &mut current, &StdPath::new(&path), "/".try_into().unwrap());

//     let mut line = String::new();
//     let mut stdin = stdin();
//     use std::io::stdin;

//     // let mut callbacks: Vec<Box<ReplClosures>> = Vec::new();

//     // let ls_closure = |fs: &mut Vfs, current: &mut Current, args: Vec<&str>| -> DcacheResult<()> {
//     //     let arg = args.get(0);
//     //     let path;
//     //     let entry;
//     //     let entry_id;

//     //     match arg {
//     //         Some(&arg) => {
//     //             path = Path::try_from(arg)?;
//     //             entry_id = dc.pathname_resolution(current.cwd, path)?;
//     //             entry = dc.d_entries.get(&entry_id).ok_or(NoSuchEntry)?;

//     //         },
//     //         None => {
//     //             entry_id = current.cwd;
//     //             entry = dc.d_entries.get(cwd).ok_or(NoSuchEntry)?;
//     //         }
//     //     }

//     //     if entry.is_directory() {
//     //         let directory = entry.get_directory()?;

//     //         println!("(DIRECTORY {}):", entry.filename);
//     //         for entry_id in directory.entries() {
//     //             let entry = dc.d_entries.get(entry_id).ok_or(NoSuchEntry)?;

//     //             let postfix: Option<String>;
//     //             let prefix;
//     //             if entry.is_directory() {
//     //                 postfix = None;
//     //                 prefix = "d---------";
//     //             } else if entry.is_symlink() {
//     //                 postfix = Some(format!("-> {}", entry.get_symbolic_content()?));
//     //                 prefix = "l---------";
//     //             } else {
//     //                 postfix = None;
//     //                 prefix = "----------";
//     //             }
//     //             println!("+={} {} {}", prefix, entry.filename, &postfix.unwrap_or("".to_string()));
//     //         }
//     //     } else {
//     //         println!("-> {}", dc.dentry_path(entry_id)?);
//     //     }
//     //     Ok(())
//     // };
//     // // let cd_closure = |dcache: &mut Dcache, cwd: &mut DirectoryEntryId, args: Vec<&str>| -> DcacheResult<()> {
//     // //     let path = *args.get(0).ok_or(NotEnoughArguments)?;
//     // //     let path = Path::try_from(path)?;
//     // //     let search_root;
//     // //     search_root = *cwd;

//     // //     let entry_id = dcache.pathname_resolution(search_root, path)?;
//     // //     let entry = dcache.d_entries.get(&entry_id).ok_or(NoSuchEntry)?;
//     // //     if entry.is_directory() {
//     // //         *cwd = entry_id;
//     // //     } else {
//     // //         return Err(NotADirectory)
//     // //     }
//     // //     Ok(())
//     // // };
//     // // let unlink_closure = |dc: &mut Dcache, cwd: &mut DirectoryEntryId, args: Vec<&str>| -> DcacheResult<()> {
//     // //     let path = *args.get(0).ok_or(NotEnoughArguments)?;
//     // //     let path = Path::try_from(path)?;

//     // //     let search_root;
//     // //     search_root = *cwd;

//     // //     let entry_id = dc.pathname_resolution(search_root, path)?;
//     // //     if entry_id == *cwd {
//     // //         *cwd = dc.d_entries.get(&entry_id).ok_or(EntryNotConnected)?.parent_id;
//     // //     }
//     // //     dc.remove_entry(entry_id)?;
//     // //     Ok(())
//     // // };

//     // // let rename_closure = |dc: &mut Dcache, cwd: &mut DirectoryEntryId, args: Vec<&str>| -> DcacheResult<()> {
//     // //     let path = *args.get(0).ok_or(NotEnoughArguments)?;
//     // //     let new_pathname: Path = args.get(1).ok_or(NotEnoughArguments).map(|x| *x)?.try_into()?;
//     // //     let path = Path::try_from(path)?;

//     // //     let search_root;
//     // //         search_root = *cwd;

//     // //     let entry_id = dc.pathname_resolution(search_root, path)?;
//     // //     dc.rename_dentry(*cwd, entry_id, new_pathname)?;
//     // //     Ok(())
//     // // };

//     // // let symlink_closure = |dc: &mut Dcache, cwd: &mut DirectoryEntryId, args: Vec<&str>| -> DcacheResult<()> {
//     // //     let path = *args.get(0).ok_or(NotEnoughArguments)?;
//     // //     let new_symlink_pathname = args.get(1).ok_or(NotEnoughArguments)?;
//     // //     let path = Path::try_from(path)?;
//     // //     let new_symlink_path = Path::try_from(*new_symlink_pathname)?;

//     // //     let search_root;
//     // //         search_root = *cwd;

//     // //     let parent_path = new_symlink_path.parent();
//     // //     let filename = new_symlink_path.filename().unwrap(); //remove this unwrap
//     // //     let parent_id = dc.pathname_resolution(search_root, parent_path)?;
//     // //     let mut new_symlink_entry = DirectoryEntry::default();

//     // //     println!("Created symlink {} with path: {}", new_symlink_path, path);

//     // //     new_symlink_entry
//     // //         .set_filename(*filename)
//     // //         .set_id(get_available_directory_entry_id())
//     // //         .set_symlink(path);

//     // //     dc.add_entry(Some(parent_id), new_symlink_entry)?;
//     // //     Ok(())
//     // // };

//     // let no_such_command_closure = |dcache: &mut Dcache, cwd: &mut DirectoryEntryId, args: Vec<&str>| -> DcacheResult<()> {
//     //     println!("No such command");
//     //     Ok(())
//     // };
//     // let callbacks_strings = ["ls"// , "cd", "unlink", "rename", "symlink"
//     //                          , "help", ""];

//     // let help = |_dcache: &mut Dcache, _cwd: &mut DirectoryEntryId, _args: Vec<&str>| -> DcacheResult<()> {
//     //     let command_strings = ["ls"// , "cd", "unlink", "rename", "symlink"
//     //                            , "help", ""];

//     //     println!("Available commands:");
//     //     for command in command_strings.iter() {
//     //         println!("- {}", command);
//     //     }
//     //     Ok(())
//     // };

//     // let print_prompt_closure = |dcache: &Dcache, cwd: &DirectoryEntryId| {
//     //     let entry = dcache.d_entries.get(cwd).unwrap();
//     //     print!("{}> ", entry.filename);
//     //     use std::io::{stdout, Write};

//     //     stdout().flush()
//     // };

//     // type ReplClosures = dyn Fn(&mut Vfs, &mut Current, Vec<&str>) -> DcacheResult<()>;
//     // callbacks.push(Box::new(ls_closure));
//     // // callbacks.push(Box::new(cd_closure));
//     // // callbacks.push(Box::new(unlink_closure));
//     // // callbacks.push(Box::new(rename_closure));
//     // // callbacks.push(Box::new(symlink_closure));
//     // callbacks.push(Box::new(help));
//     // callbacks.push(Box::new(no_such_command_closure));
//     // let mut cwd_id = dcache.root_id;

//     // loop {
//     //     line.clear();
//     //     print_prompt_closure(&dcache, &cwd_id);
//     //     match stdin.read_line(&mut line) {
//     //         Ok(_) => {
//     //             println!("-> {}", line);
//     //         },
//     //         Err(e) => {
//     //             println!("(ERROR) -> {}", e);
//     //         }
//     //     }
//     //     let fields = line.split_ascii_whitespace().collect::<Vec<&str>>();
//     //     if fields.len() == 0 {
//     //         continue
//     //     }

//     //     let callback = callbacks_strings.iter().zip(callbacks.iter()).find(|(&x, _)| x == fields[0] || x == "")
//     //         .map(|(_, callback)| callback).unwrap();

//     //     if let Err(e) = (callback)(&mut dcache, &mut cwd_id, fields[1..].to_vec()) {
//     //         println!("Error(e) => {:?}", e);
//     //     }
//     // }
// }

#[cfg(test)]
mod vfs {

    fn default_current() -> Current {
        Current {
            cwd: DirectoryEntryId::new(2),
            uid: 0,
            euid: 0,
            gid: 0,
            egid: 0,
            open_fds: BTreeMap::new(),
        }
    }

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

    macro_rules! vfs_file_exists_test {
        ($body: block, $path: expr, $name: ident) => {
            make_test! {{
                let mut vfs = Vfs::new().unwrap();
                let mut current = default_current();
                let path: &str = $path;
                let path: Path = path.try_into().unwrap();

                if path != "/".try_into().unwrap() {
                    vfs.recursive_creat(&mut current, path.clone(), FilePermissions::S_IRWXU).unwrap();
                }
                assert!(vfs.file_exists(&current, path).unwrap())
            }, $name}
        };
        (failing, $body: block, $path: expr, $name: ident) => {
            make_test! {failing, {
                let mut vfs = Vfs::new().unwrap();
                let mut current = default_current();
                let path: &str = $path;
                let path: Path = path.try_into().unwrap();

                if path != "/".try_into().unwrap() {
                    vfs.recursive_creat(&mut current, path.clone(), FilePermissions::S_IRWXU).unwrap();
                }
                assert!(vfs.file_exists(&current, path).unwrap())
            }, $name}
        };
    }

    vfs_file_exists_test! {{}, "/", file_exists_root}
    vfs_file_exists_test! {failing, {}, "", file_exists_null}
    vfs_file_exists_test! {{
    }, "a", file_exists_basic_a}
    vfs_file_exists_test! {{
    }, "/a", file_exists_basic_root_a}

    vfs_file_exists_test! {{
    }, "a/b", file_exists_basic_a_b}
    vfs_file_exists_test! {{
    }, "a/b/c", file_exists_basic_a_b_c}
    vfs_file_exists_test! {{
    }, "a/b/c/d", file_exists_basic_a_b_c_d}
    vfs_file_exists_test! {{
    }, "a/b/c/d/e/f", file_exists_basic_a_b_c_d_e_f}

    vfs_file_exists_test! {{
    }, "/a/b", file_exists_basic_root_a_b}
    vfs_file_exists_test! {{
    }, "/a/b/c", file_exists_basic_root_a_b_c}
    vfs_file_exists_test! {{
    }, "/a/b/c/d", file_exists_basic_root_a_b_c_d}
    vfs_file_exists_test! {{
    }, "/a/b/c/d/e/f", file_exists_basic_root_a_b_c_d_e_f}

    macro_rules! vfs_recursive_creat_test {
        ($path: expr, $name: ident) => {
            make_test! {{
                let mut vfs = Vfs::new().unwrap();
                let mut current = default_current();
                let path: &str = $path;
                let path: Path = path.try_into().unwrap();

                vfs.recursive_creat(&mut current
                                    , path.clone()
                                    , FilePermissions::S_IRWXU).unwrap();
                assert!(vfs.file_exists(&current, path).unwrap())
            }, $name}
        };
        (failing, $path: expr, $name: ident) => {
            make_test! {failing, {
                let mut vfs = Vfs::new().unwrap();
                let mut current = default_current();
                let path: &str = $path;
                let path: Path = path.try_into().unwrap();

                vfs.recursive_creat(&mut current
                                    , path.clone()
                                    , FilePermissions::S_IRWXU).unwrap();
                for ancestors in path.ancestors() {
                    assert!(vfs.file_exists(&current, ancestor).unwrap())
                }
            }, $name}
        };
    }

    vfs_recursive_creat_test! {"a/b/c/d/e/f/g", recursive_creat_a_b_c_d_e_f_g}
    vfs_recursive_creat_test! {"a/b/c/d/e/f  ", recursive_creat_a_b_c_d_e_f}
    vfs_recursive_creat_test! {"a/b/c/d/e    ", recursive_creat_a_b_c_d_e}
    vfs_recursive_creat_test! {"a/b/c/d      ", recursive_creat_a_b_c_d}
    vfs_recursive_creat_test! {"a/b/c        ", recursive_creat_a_b_c}
    vfs_recursive_creat_test! {"a/b          ", recursive_creat_a_b} // infinite loop
    vfs_recursive_creat_test! {"a            ", recursive_creat_a}
}
