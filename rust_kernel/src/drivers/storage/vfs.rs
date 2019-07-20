// #![deny(missing_docs)]

pub mod direntry;
use direntry::{DirectoryEntry, DirectoryEntryHeader};

pub mod inode;
use inode::Inode;

pub mod posix_consts;
pub use posix_consts::*;

use alloc::vec::Vec;
use alloc::boxed::Box;
use errno::Errno;

use core::str::{Split};
use core::iter::Filter;

type DeviceId = usize;

/// Type of file
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Filetype {
    RegularFile,
    Directory,
    CharacterDevice,
    BlockDevice,
    Fifo,
    Socket,
    SymbolicLink,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VfsError {
    MountError,
    IsNotADirectory,
    Errno(Errno),
}

impl From<Errno> for VfsError {
    fn from(value: Errno) -> Self {
        VfsError::Errno(value)
    }
}


pub type VfsResult<T> = Result<T, VfsError>;

pub trait Filesystem {
    fn get_name(&self) -> &str;
    fn read_superblock(&self) -> Superblock;
}

enum FileSystemType {}

pub struct Superblock {
    file_system_type: FileSystemType,
    blocksize: usize,
}


pub struct VirtualFileSystem {
    mounted_filesystems: Vec<Box<Filesystem>>,
    superblocks: Vec<Superblock>,
    inodes: Vec<Inode>,
    root_dentry: DirectoryEntry,
}

type Path = str;

impl VirtualFileSystem {
    pub fn new() -> VfsResult<VirtualFileSystem> {
        let mut new = Self {
            mounted_filesystems: Vec::new(),
            superblocks: Vec::new(),
            inodes: Vec::new(),
            root_dentry: DirectoryEntry::new("/", Filetype::Directory, 2)?
        };

        new.root_dentry.add_direntry(DirectoryEntry::new("test", Filetype::Directory, 3)?);
        new.root_dentry.add_direntry(DirectoryEntry::new("test1", Filetype::Directory, 3)?);
        new.root_dentry.add_direntry(DirectoryEntry::new("test2", Filetype::Directory, 3)?);
        new.root_dentry.add_direntry(DirectoryEntry::new("test3", Filetype::Directory, 3)?);
        Ok(new)
    }

    fn pathname_resolution(&mut self, path: &Path) -> Option<&DirectoryEntry> {
        fn aux<'a>(mut components: Split<char>, root: &'a DirectoryEntry) -> Option<&'a DirectoryEntry> {
            let component = loop { // rewrite all of this with skip_while.
                match components.next() {
                    Some(component) => {
                        if component == "" {
                            continue;
                        }
                        break component

                    },
                    None => return Some(root)
                }
            };
            for dirent in root.directory_entries().unwrap().filter(|x| x.is_directory()) {
                if unsafe { dirent.get_filename() } == component {
                    return aux(components, dirent)
                }
            }
            None
        }

        let path_components = path.split('/');
        let root;

        if &path[0..1] == "/" {
            root = &self.root_dentry;
        } else {
            unimplemented!();
            // root = getcwd();
        }

        aux(path_components, root)
    }

    fn mount_on_dentry(&mut self,
                       filesystem: Box<Filesystem>,
                       dentry: &mut DirectoryEntry) -> VfsResult<()> {


        let new_superblock = filesystem.read_superblock();

        self.superblocks.push(new_superblock);
        self.mounted_filesystems.push(filesystem);
        Ok(())
    }
}

use core::fmt::{Display, Error, Formatter};

impl Display for VirtualFileSystem {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        writeln!(f, "Mounted filesystems:")?;
        for filesystem in self.mounted_filesystems.iter() {
            writeln!(f, "   {}", filesystem.get_name())?;
        }

        self.root_dentry.walk_tree(&mut |entry: &DirectoryEntry| writeln!(f, "-{}-", unsafe { entry.get_filename() } ))?;

        Ok(())
    }
}

pub fn init() -> VfsResult<VirtualFileSystem> {
    let mut vfs = VirtualFileSystem::new()?;

    println!("{}", vfs);
    let result: Result<(), VfsError> = vfs.root_dentry.walk_tree_mut(&mut |entry| { entry.add_direntry(DirectoryEntry::new("lol", Filetype::Directory, 5)?);; Ok(())});

    println!("{}", vfs);
    println!("pathname resolution {:?}", vfs.pathname_resolution("/test/lol"));
    println!("pathname resolution {:?}", vfs.pathname_resolution("/test1/lol"));
    println!("pathname resolution {:?}", vfs.pathname_resolution("/test/dlol"));

    Ok(vfs)
}
