#![feature(underscore_const_names)]
#![feature(try_trait)]
use std::cmp::{Eq, PartialEq, Ord, PartialOrd};
use std::collections::BTreeMap;
use std::str::FromStr;

extern crate errno;
#[macro_use]
extern crate const_assert;
#[macro_use]
extern crate bitflags;

mod posix_consts;
mod path;
use path::{Path, Filename};

mod direntry;
use direntry::{DirectoryEntry, DirectoryEntryId};

mod dcache;

use dcache::{Dcache, DcacheError, DcacheResult};

mod inode;
use inode::{Inode, InodeNumber, InodeId, OpenFlags, File};

mod user;
use user::{UserId, GroupId, Current};

use errno::Errno;
use Errno::*;

mod permissions;
use permissions::{FilePermissions};

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


pub struct VirtualFileSystem {
    // mounted_filesystems: Vec<Box<Filesystem>>,
    // superblocks: Vec<Superblock>,
    inodes: BTreeMap<InodeId, Inode>,
    dcache: Dcache,
    open_file_descriptions: Vec<File>,
}

type Vfs = VirtualFileSystem;

impl VirtualFileSystem {
    pub fn new() -> VfsResult<VirtualFileSystem> {
        let mut new = Self {
            // mounted_filesystems: Vec::new(),
            // superblocks: Vec::new(),
            inodes: BTreeMap::new(),
            dcache: Dcache::new(),
            open_file_descriptions: Vec::new(),
        };

        let root_inode = Inode::root_inode();
        let root_inode_id = root_inode.id;

        new.inodes.insert(root_inode_id, root_inode);
        Ok(new)
    }

    pub fn unlink(&mut self, current: &Current, path: Path) -> VfsResult<()> {
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

        if corresponding_inode.link_number == 0
            && !corresponding_inode.is_opened() {
                self.inodes.remove(&inode_id).ok_or(NoSuchInode)?;
            }
        Ok(())
    }

    fn get_available_id(&self) -> InodeId {
        let mut current_id = InodeId::new(2); // check this
        loop {
            if let None = self.inodes.get(&current_id) {
                return current_id
            }

            // this is unchecked
            current_id = InodeId::new(current_id.inode_number + 1);
        }
    }


    pub fn open(&mut self,
                current: &Current,
                path: Path,
                flags: OpenFlags,
                mode: FilePermissions) -> VfsResult<File> {
        let entry_id;
        match self.dcache.pathname_resolution(current.cwd, path.clone()) {
            Ok(id) if flags.contains(OpenFlags::O_CREAT | OpenFlags::O_EXCL) => return Err(Errno(Errno::Eexist)),
            Ok(id) => entry_id = id,
            Err(e) if !flags.contains(OpenFlags::O_CREAT) => return Err(e.into()),
            _ => {
                let mut new_inode = Inode::default();
                let new_id = self.get_available_id();

                new_inode
                    .set_id(new_id)
                    .set_access_mode(mode)
                    .set_uid(current.uid)
                    .set_gid(current.gid); // posix does not really like this.

                new_inode.link_number += 1;
                assert!(self.inodes.insert(new_id, new_inode).is_none());
                let mut new_direntry = DirectoryEntry::default();
                let parent_id = self.dcache.pathname_resolution(current.cwd, path.parent())?;

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
        if flags.contains(OpenFlags::O_DIRECTORY)
            && !entry.is_directory() {
                return Err(NotADirectory);
            }

        let inode = self.inodes.get_mut(&entry.inode_id).ok_or(NoSuchInode)?;
        Ok(inode.open(entry.id, flags))
    }

    pub fn creat(&mut self,
                 current: &Current,
                 path: Path,
                 mode: FilePermissions) -> VfsResult<File> {
        let flags = OpenFlags::O_WRONLY |
        OpenFlags::O_CREAT |
        OpenFlags::O_TRUNC;

        Ok(self.open(current, path, flags, mode)?)
    }

    pub fn chmod(&mut self, current: &Current, path: Path, mode: FilePermissions) -> VfsResult<()> {
        let entry_id = self.dcache.pathname_resolution(current.cwd, path)?;

        let entry = self.dcache.get_entry(&entry_id)?;

        let inode = self.inodes.get_mut(&entry.inode_id).ok_or(NoSuchInode)?;

        inode.set_access_mode(mode);
        Ok(())
    }

    pub fn chown(&mut self, current: &Current, path: Path, owner: UserId, group: GroupId) -> VfsResult<()> {
        let entry_id = self.dcache.pathname_resolution(current.cwd, path)?;

        let entry = self.dcache.get_entry(&entry_id)?;

        let inode = self.inodes.get_mut(&entry.inode_id).ok_or(NoSuchInode)?;

        inode.set_uid(owner);
        inode.set_gid(group);
        Ok(())
    }

    pub fn mkdir(&mut self, current: &Current, path: Path, mode: FilePermissions) -> VfsResult<()> {
        let flags = OpenFlags::O_DIRECTORY | OpenFlags::O_CREAT;

        self.open(current, path, flags, mode)?;
        Ok(())
    }

    pub fn rmdir(&mut self, current: &Current, path: Path) -> VfsResult<()> {
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

    pub fn link(&mut self, current: &Current, oldpath: Path, newpath: Path) -> VfsResult<()> {
        let oldentry_id = self.dcache.pathname_resolution(current.cwd, oldpath)?;
        let oldentry = self.dcache.get_entry(&oldentry_id)?;

        if oldentry.is_directory() {//link on directories not currently supported.
            return Err(Errno(Eisdir));
        }

        if self.dcache.pathname_resolution(current.cwd, newpath.clone()).is_ok() {
            return Err(Errno(Eexist));
        }

        let parent_new = self.dcache.pathname_resolution(current.cwd, newpath.parent())?;

        let inode = self.inodes.get_mut(&oldentry.inode_id).ok_or(NoSuchInode)?;

        let mut newentry = oldentry.clone();

        newentry.filename = *newpath.filename().unwrap(); // remove this unwrap somehow.
        self.dcache.add_entry(Some(parent_new), newentry)?;
        inode.link_number += 1;
        Ok(())
    }

    pub fn rename(&mut self, current: &Current, oldpath: Path, newpath: Path) -> VfsResult<()> {
        let oldentry_id = self.dcache.pathname_resolution(current.cwd, oldpath)?;

        self.dcache.rename_dentry(current.cwd, oldentry_id, newpath)?;
        Ok(())
    }
}





use walkdir::WalkDir;
use std::fs::{FileType, DirEntry, read_dir};
use std::os::unix::fs::PermissionsExt;
use std::path::Path as StdPath;
use std::convert::{TryFrom, TryInto};
fn main() {
    use std::env;
    let mut vfs = Vfs::new().unwrap();

    let mut args = env::args().skip(1);
    let current = Current {
        cwd: DirectoryEntryId::new(2),
        uid: 0,
        euid: 0,
        gid: 0,
        egid: 0,
    };

    fn construct_tree(vfs: &mut Vfs, current: &Current, root: &StdPath, current_path: Path) {
        let mut iter = read_dir(root).unwrap().filter_map(|e| e.ok());

        for entry in iter {
            let filename = Filename::try_from(entry.file_name().to_str().unwrap()).unwrap();
            let mut path = current_path.clone();

            path.push(filename).unwrap();
            // let mut new = DirectoryEntry::default();

            // new.set_filename();
            // new.set_id(get_available_directory_entry_id());
            let filetype = entry.file_type().unwrap();
            let mode = unsafe { FilePermissions::from_u32(entry.metadata().unwrap().permissions().mode()) };

            let mut flags = OpenFlags::O_CREAT;

            if filetype.is_dir() {
                flags |= OpenFlags::O_DIRECTORY;
            } else if filetype.is_symlink() {
                // let std_path = std::fs::read_link(entry.path()).unwrap();
                // let path = std_path.as_os_str().to_str().unwrap().try_into().unwrap();
                // new.set_symlink(path);
            }

            // println!("{}", path);
            vfs.open(&current, path.clone(), flags, mode).unwrap();
            if entry.file_type().unwrap().is_dir() {
                construct_tree(vfs, current, &entry.path(), path);
            }
        }
    }

    let path = args.next().unwrap();

    construct_tree(&mut vfs, &current, &StdPath::new(&path), "/".try_into().unwrap());


    let mut line = String::new();
    let mut stdin = stdin();
    use std::io::stdin;

    // let mut callbacks: Vec<Box<ReplClosures>> = Vec::new();

    // let ls_closure = |fs: &mut Vfs, current: &mut Current, args: Vec<&str>| -> DcacheResult<()> {
    //     let arg = args.get(0);
    //     let path;
    //     let entry;
    //     let entry_id;

    //     match arg {
    //         Some(&arg) => {
    //             path = Path::try_from(arg)?;
    //             entry_id = dc.pathname_resolution(current.cwd, path)?;
    //             entry = dc.d_entries.get(&entry_id).ok_or(NoSuchEntry)?;

    //         },
    //         None => {
    //             entry_id = current.cwd;
    //             entry = dc.d_entries.get(cwd).ok_or(NoSuchEntry)?;
    //         }
    //     }


    //     if entry.is_directory() {
    //         let directory = entry.get_directory()?;

    //         println!("(DIRECTORY {}):", entry.filename);
    //         for entry_id in directory.entries() {
    //             let entry = dc.d_entries.get(entry_id).ok_or(NoSuchEntry)?;

    //             let postfix: Option<String>;
    //             let prefix;
    //             if entry.is_directory() {
    //                 postfix = None;
    //                 prefix = "d---------";
    //             } else if entry.is_symlink() {
    //                 postfix = Some(format!("-> {}", entry.get_symbolic_content()?));
    //                 prefix = "l---------";
    //             } else {
    //                 postfix = None;
    //                 prefix = "----------";
    //             }
    //             println!("+={} {} {}", prefix, entry.filename, &postfix.unwrap_or("".to_string()));
    //         }
    //     } else {
    //         println!("-> {}", dc.dentry_path(entry_id)?);
    //     }
    //     Ok(())
    // };
    // // let cd_closure = |dcache: &mut Dcache, cwd: &mut DirectoryEntryId, args: Vec<&str>| -> DcacheResult<()> {
    // //     let path = *args.get(0).ok_or(NotEnoughArguments)?;
    // //     let path = Path::try_from(path)?;
    // //     let search_root;
    // //     search_root = *cwd;

    // //     let entry_id = dcache.pathname_resolution(search_root, path)?;
    // //     let entry = dcache.d_entries.get(&entry_id).ok_or(NoSuchEntry)?;
    // //     if entry.is_directory() {
    // //         *cwd = entry_id;
    // //     } else {
    // //         return Err(NotADirectory)
    // //     }
    // //     Ok(())
    // // };
    // // let unlink_closure = |dc: &mut Dcache, cwd: &mut DirectoryEntryId, args: Vec<&str>| -> DcacheResult<()> {
    // //     let path = *args.get(0).ok_or(NotEnoughArguments)?;
    // //     let path = Path::try_from(path)?;

    // //     let search_root;
    // //     search_root = *cwd;

    // //     let entry_id = dc.pathname_resolution(search_root, path)?;
    // //     if entry_id == *cwd {
    // //         *cwd = dc.d_entries.get(&entry_id).ok_or(EntryNotConnected)?.parent_id;
    // //     }
    // //     dc.remove_entry(entry_id)?;
    // //     Ok(())
    // // };

    // // let rename_closure = |dc: &mut Dcache, cwd: &mut DirectoryEntryId, args: Vec<&str>| -> DcacheResult<()> {
    // //     let path = *args.get(0).ok_or(NotEnoughArguments)?;
    // //     let new_pathname: Path = args.get(1).ok_or(NotEnoughArguments).map(|x| *x)?.try_into()?;
    // //     let path = Path::try_from(path)?;

    // //     let search_root;
    // //         search_root = *cwd;

    // //     let entry_id = dc.pathname_resolution(search_root, path)?;
    // //     dc.rename_dentry(*cwd, entry_id, new_pathname)?;
    // //     Ok(())
    // // };

    // // let symlink_closure = |dc: &mut Dcache, cwd: &mut DirectoryEntryId, args: Vec<&str>| -> DcacheResult<()> {
    // //     let path = *args.get(0).ok_or(NotEnoughArguments)?;
    // //     let new_symlink_pathname = args.get(1).ok_or(NotEnoughArguments)?;
    // //     let path = Path::try_from(path)?;
    // //     let new_symlink_path = Path::try_from(*new_symlink_pathname)?;

    // //     let search_root;
    // //         search_root = *cwd;

    // //     let parent_path = new_symlink_path.parent();
    // //     let filename = new_symlink_path.filename().unwrap(); //remove this unwrap
    // //     let parent_id = dc.pathname_resolution(search_root, parent_path)?;
    // //     let mut new_symlink_entry = DirectoryEntry::default();

    // //     println!("Created symlink {} with path: {}", new_symlink_path, path);

    // //     new_symlink_entry
    // //         .set_filename(*filename)
    // //         .set_id(get_available_directory_entry_id())
    // //         .set_symlink(path);

    // //     dc.add_entry(Some(parent_id), new_symlink_entry)?;
    // //     Ok(())
    // // };

    // let no_such_command_closure = |dcache: &mut Dcache, cwd: &mut DirectoryEntryId, args: Vec<&str>| -> DcacheResult<()> {
    //     println!("No such command");
    //     Ok(())
    // };
    // let callbacks_strings = ["ls"// , "cd", "unlink", "rename", "symlink"
    //                          , "help", ""];

    // let help = |_dcache: &mut Dcache, _cwd: &mut DirectoryEntryId, _args: Vec<&str>| -> DcacheResult<()> {
    //     let command_strings = ["ls"// , "cd", "unlink", "rename", "symlink"
    //                            , "help", ""];

    //     println!("Available commands:");
    //     for command in command_strings.iter() {
    //         println!("- {}", command);
    //     }
    //     Ok(())
    // };

    // let print_prompt_closure = |dcache: &Dcache, cwd: &DirectoryEntryId| {
    //     let entry = dcache.d_entries.get(cwd).unwrap();
    //     print!("{}> ", entry.filename);
    //     use std::io::{stdout, Write};

    //     stdout().flush()
    // };

    // type ReplClosures = dyn Fn(&mut Vfs, &mut Current, Vec<&str>) -> DcacheResult<()>;
    // callbacks.push(Box::new(ls_closure));
    // // callbacks.push(Box::new(cd_closure));
    // // callbacks.push(Box::new(unlink_closure));
    // // callbacks.push(Box::new(rename_closure));
    // // callbacks.push(Box::new(symlink_closure));
    // callbacks.push(Box::new(help));
    // callbacks.push(Box::new(no_such_command_closure));
    // let mut cwd_id = dcache.root_id;

    // loop {
    //     line.clear();
    //     print_prompt_closure(&dcache, &cwd_id);
    //     match stdin.read_line(&mut line) {
    //         Ok(_) => {
    //             println!("-> {}", line);
    //         },
    //         Err(e) => {
    //             println!("(ERROR) -> {}", e);
    //         }
    //     }
    //     let fields = line.split_ascii_whitespace().collect::<Vec<&str>>();
    //     if fields.len() == 0 {
    //         continue
    //     }

    //     let callback = callbacks_strings.iter().zip(callbacks.iter()).find(|(&x, _)| x == fields[0] || x == "")
    //         .map(|(_, callback)| callback).unwrap();

    //     if let Err(e) = (callback)(&mut dcache, &mut cwd_id, fields[1..].to_vec()) {
    //         println!("Error(e) => {:?}", e);
    //     }
    // }
}
