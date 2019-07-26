// #![deny(missing_docs)]

pub mod direntry;
use direntry::{DirectoryEntry, DirectoryEntryHeader, DirectoryEntryId};

pub mod inode;
use inode::{Inode, InodeId, InodeStatus, inode_number, mode_t, File, InodeOperations};


pub mod posix_consts;
pub use posix_consts::*;

pub mod stat;
use stat::UserStat;

use alloc::vec::Vec;
use alloc::boxed::Box;


use core::str::{Split};
use core::iter::Filter;
use core::option::NoneError;
use core::cell::RefCell;

use alloc::collections::btree_map::BTreeMap;

use errno::Errno;
use Errno::*;

use bitflags::bitflags;

pub type DeviceId = usize;

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
    NoSuchInode,
    InodeAlreadyExists,
    Errno(Errno),
}

impl From<Errno> for VfsError {
    fn from(value: Errno) -> Self {
        VfsError::Errno(value)
    }
}

impl From<VfsError> for NoneError {
    fn from(_value: VfsError) -> Self {
        NoneError
    }
}


pub type VfsResult<T> = Result<T, VfsError>;

pub trait Filesystem {
    fn get_name(&self) -> &str;
    fn read_superblock(&self) -> Superblock;
    fn get_root_inode(&self) -> Inode;
}

enum FileSystemType {}

pub struct SuperBlockOperations {

}

pub struct Superblock {
    file_system_type: FileSystemType,
    blocksize: usize,
    super_operations: SuperBlockOperations,
}


pub struct Dcache {
    root_dentry: DirectoryEntryId,
    dentries: BTreeMap<DirectoryEntryId, DirectoryEntry>,
}

impl Dcache {
    pub fn new() -> Self {
        let mut new = Self {
            root_dentry: 2,
            dentries: BTreeMap::new(),
        };

        let root_dentry = DirectoryEntry::new("/", Filetype::Directory, InodeId::new(2, 0)).unwrap();
        new.add_direntry(root_dentry).unwrap();
        new
    }

    fn get_available_id(&self) -> DirectoryEntryId {
        let mut current_id = self.root_dentry; // check this
        loop {
            if let None = self.dentries.get(&current_id) {
                return current_id
            }
            current_id = current_id.checked_add(1).expect("No space left inside the dcache lool");
        }
    }

    pub fn add_direntry(&mut self, mut entry: DirectoryEntry) -> Option<&DirectoryEntry> {
        let id = self.get_available_id();
        entry.header.id = id;
        if self.dentries.contains_key(&id) {
            None
        } else {
            self.dentries.insert(id, entry);
            self.get_direntry(id)
        }
    }

    pub fn get_direntry(&self, id: DirectoryEntryId) -> Option<&DirectoryEntry> {
        self.dentries.get(&id)
    }

    pub fn get_direntry_mut(&mut self, id: DirectoryEntryId) -> Option<&mut DirectoryEntry> {
        self.dentries.get_mut(&id)
    }
}

pub struct VirtualFileSystem {
    mounted_filesystems: Vec<Box<Filesystem>>,
    superblocks: Vec<Superblock>,
    // inodes: Vec<Inode>,
    inodes: BTreeMap<InodeId, Inode>,
    dcache: Dcache,
}
type Path = str;


bitflags! {
    #[derive(Default)] // I wonder for this derive <.<
    pub struct open_flags: u32 {
        /// Open for execute only (non-directory files). The result is unspecified if this flag is applied to a directory.
        const O_EXEC = 0x1;

        /// Open for reading only.
        const O_RDONLY = 0x2;

        /// Open for reading and writing. The result is undefined if this flag is applied to a FIFO.
        const O_RDWR = 0x4;

        /// Open directory for search only. The result is unspecified if this flag is applied to a non-directory file.
        const O_SEARCH = 0x8;

        /// Open for writing only.
        const O_WRONLY = 0x10;

        /// If set, the file offset shall be set to the end of the file prior to each write.
        const O_APPEND = 0x20;

        /// If set, the FD_CLOEXEC flag for the new file descriptor shall be set.
        const O_CLOEXEC = 0x40;

        /// If the file exists, this flag has no effect except as noted under O_EXCL below.
        /// Otherwise, if O_DIRECTORY is not set the file shall be created as a regular file; the user ID of the file shall be set to the effective user ID of the process; the group ID of the file shall be set to the group ID of the file's parent directory or to the effective group ID of the process; and the access permission bits (see <sys/stat.h>) of the file mode shall be set to the value of the argument following the oflag argument taken as type mode_t modified as follows: a bitwise AND is performed on the file-mode bits and the corresponding bits in the complement of the process' file mode creation mask. Thus, all bits in the file mode whose corresponding bit in the file mode creation mask is set are cleared. When bits other than the file permission bits are set, the effect is unspecified. The argument following the oflag argument does not affect whether the file is open for reading, writing, or for both. Implementations shall provide a way to initialize the file's group ID to the group ID of the parent directory. Implementations may, but need not, provide an implementation-defined way to initialize the file's group ID to the effective group ID of the calling process.
        // do something about this pave
        const O_CREAT = 0x80;

        /// If path resolves to a non-directory file, fail and set errno to [ENOTDIR].
        const O_DIRECTORY = 0x100;

        /// Write I/O operations on the file descriptor shall complete as defined by synchronized I/O data integrity completion. [Option End]
        const O_DSYNC = 0x200;

        /// If O_CREAT and O_EXCL are set, open() shall fail if the file exists. The check for the existence of the file and the creation of the file if it does not exist shall be atomic with respect to other threads executing open() naming the same filename in the same directory with O_EXCL and O_CREAT set. If O_EXCL and O_CREAT are set, and path names a symbolic link, open() shall fail and set errno to [EEXIST], regardless of the contents of the symbolic link. If O_EXCL is set and O_CREAT is not set, the result is undefined.
        const O_EXCL = 0x400;

        /// If set and path identifies a terminal device, open() shall not cause the terminal device to become the controlling terminal for the process. If path does not identify a terminal device, O_NOCTTY shall be ignored.
        const O_NOCTTY = 0x800;

        /// If path names a symbolic link, fail and set errno to [ELOOP].
        const O_NOFOLLOW = 0x1000;

        /// When opening a FIFO with O_RDONLY or O_WRONLY set: If O_NONBLOCK is set, an open() for reading-only shall return without delay. An open() for writing-only shall return an error if no process currently has the file open for reading.
        ///
        /// If O_NONBLOCK is clear, an open() for reading-only shall block the calling thread until a thread opens the file for writing. An open() for writing-only shall block the calling thread until a thread opens the file for reading.
        ///
        /// When opening a block special or character special file that supports non-blocking opens:
        ///
        /// If O_NONBLOCK is set, the open() function shall return without blocking for the device to be ready or available. Subsequent behavior of the device is device-specific.
        ///
        /// If O_NONBLOCK is clear, the open() function shall block the calling thread until the device is ready or available before returning.
        ///
        /// Otherwise, the O_NONBLOCK flag shall not cause an error, but it is unspecified whether the file status flags will include the O_NONBLOCK flag.
        const O_NONBLOCK = 0x2000;

        /// Read I/O operations on the file descriptor shall complete at the same level of integrity as specified by the O_DSYNC and O_SYNC flags. If both O_DSYNC and O_RSYNC are set in oflag, all I/O operations on the file descriptor shall complete as defined by synchronized I/O data integrity completion. If both O_SYNC and O_RSYNC are set in flags, all I/O operations on the file descriptor shall complete as defined by synchronized I/O file integrity completion. [Option End]
        const O_RSYNC = 0x4000;

        /// Write I/O operations on the file descriptor shall complete as defined by synchronized I/O file integrity completion. [Option End]
        /// The O_SYNC flag shall be supported for regular files, even if the Synchronized Input and Output option is not supported. [Option End]
        const O_SYNC = 0x8000;

        /// If the file exists and is a regular file, and the file is successfully opened O_RDWR or O_WRONLY, its length shall be truncated to 0, and the mode and owner shall be unchanged. It shall have no effect on FIFO special files or terminal device files. Its effect on other file types is implementation-defined. The result of using O_TRUNC without either O_RDWR or O_WRONLY is undefined.
        const O_TRUNC = 0x10000;

        /// If path identifies a terminal device other than a pseudo-terminal, the device is not already open in any process, and either O_TTY_INIT is set in oflag or O_TTY_INIT has the value zero, open() shall set any non-standard termios structure terminal parameters to a state that provides conforming behavior; see XBD Parameters that Can be Set. It is unspecified whether O_TTY_INIT has any effect if the device is already open in any process. If path identifies the slave side of a pseudo-terminal that is not already open in any process, open() shall set any non-standard termios structure terminal parameters to a state that provides conforming behavior, regardless of whether O_TTY_INIT is set. If path does not identify a terminal device, O_TTY_INIT shall be ignored.
        const O_TTY_INIT = 0x20000;
    }
}

impl VirtualFileSystem {
    pub fn new() -> VfsResult<VirtualFileSystem> {
        let mut new = Self {
            mounted_filesystems: Vec::new(),
            superblocks: Vec::new(),
            inodes: BTreeMap::new(),
            dcache: Dcache::new(),
        };

        new.dcache.add_direntry(DirectoryEntry::new("test", Filetype::Directory, InodeId::new(3, 0))?);
        new.dcache.add_direntry(DirectoryEntry::new("test1", Filetype::Directory, InodeId::new(3, 0))?);
        new.dcache.add_direntry(DirectoryEntry::new("test2", Filetype::Directory, InodeId::new(3, 0))?);
        new.dcache.add_direntry(DirectoryEntry::new("test3", Filetype::Directory, InodeId::new(3, 0))?);
o        Ok(new)
    }


    fn pathname_resolution(&mut self, path: &Path) -> Option<&DirectoryEntry> {
        // fn aux<'a>(virtual_file_system: &'a mut VirtualFileSystem, mut components: Split<char>, root: &'a mut DirectoryEntry) -> Option<&'a DirectoryEntry> {
        //     let component = loop { // rewrite all of this with skip_while.
        //         match components.next() {
        //             Some(component) => {
        //                 if component == "" {
        //                     continue;
        //                 }
        //                 break component

        //             },
        //             None => return Some(root)
        //         }
        //     };

        //     let root_inode_id = root.header.inode_id;
        //     for dirent in root.directory_entries_mut().unwrap().filter(|x| x.is_directory()) {
        //         if unsafe { dirent.get_filename() } == component {
        //             return aux(virtual_file_system, components, dirent)
        //         }
        //     }
        //     let root_inode = virtual_file_system.get_inode(root_inode_id)?;
        //     let new_direntry = (root_inode.inode_operations.lookup_direntry)(&root_inode, component);

        //     if let Some(direntry) = new_direntry {
        //         let direntry = root.add_direntry(direntry)?;
        //         let new_inode = (root_inode.inode_operations.lookup_inode)(direntry.header.inode_id).unwrap();

        //         virtual_file_system.add_inode(new_inode);
        //         return Some(direntry);
        //     }

        //     None
        // }

        let path_components = path.split('/');
        // let root;

        if &path[0..1] == "/" {
            // root = &mut self.root_dentry;
        } else {
            unimplemented!();
            // root = getcwd();
        }
        let mut components = path_components;
        let mut current_dir_id = self.dcache.root_dentry;
        'walk: loop {
            let component = loop { // rewrite all of this with skip_while.
                match components.next() {
                    Some(component) => {
                        if component == "" {
                            continue;
                        }
                        break component

                    },
                    None => return Some(self.dcache.get_direntry(current_dir_id)?)
                }
            };
            let current_dir_inode_id;
            {
                let current_dir = self.dcache.get_direntry(current_dir_id)?;
                current_dir_inode_id = current_dir.header.inode_id;
                if !current_dir.is_directory() {
                    return None
                }
                for &d_id in current_dir.directory_entries().unwrap() {
                    let dirent = self.dcache.get_direntry(d_id)?;
                    if unsafe { dirent.get_filename() } == component {
                        current_dir_id = dirent.header.id;
                        continue 'walk;
                    }
                }
            }
            let current_dir_inode = self.inodes.get(&current_dir_inode_id)?;
            let new_direntry = (current_dir_inode.inode_operations.lookup_direntry.unwrap())(&current_dir_inode, component); // remove this unwrap

            if let Some(direntry) = new_direntry {
                let direntry = self.dcache.add_direntry(direntry)?;
                let direntry_id = direntry.header.id;
                let direntry_inode_id = direntry.header.inode_id;
                self.dcache.get_direntry_mut(current_dir_id)?.add_direntry(direntry_id).unwrap(); //remove this unwrap
                let new_inode = (current_dir_inode.inode_operations.lookup_inode.unwrap())(direntry_inode_id).unwrap();
                self.add_inode(new_inode);
                current_dir_id = direntry_id;
            }
        }
    }

    fn mount_on_dentry(&mut self,
                       filesystem: Box<Filesystem>,
                       dentry: &mut DirectoryEntry) -> VfsResult<()> {


        let new_superblock = filesystem.read_superblock();

        self.superblocks.push(new_superblock);
        self.mounted_filesystems.push(filesystem);
        Ok(())
    }

    fn add_inode(&mut self,
                 inode: Inode) -> VfsResult<()> {
        if self.inodes.contains_key(&inode.id) {
            Result::Err(VfsError::InodeAlreadyExists)
        } else {
            self.inodes.insert(inode.id, inode);
            Result::Ok(())
        }
    }



    pub fn get_inode(&self, id: InodeId) -> Option<&Inode> {
        self.inodes.get(&id)
    }

    pub fn get_inode_mut(&mut self, id: InodeId) -> Option<&mut Inode> {
        self.inodes.get_mut(&id)
    }

    pub fn unlink_inode(&mut self, id: InodeId) -> VfsResult<()> {
        let ref_count;
        {
            let inode = match self.get_inode_mut(id) {
                None => return Err(VfsError::NoSuchInode),
                Some(inode) => inode,
            };

            inode.link_number -= 1;
            if (inode.link_number == 0) {
                inode.status = InodeStatus::ToBeRemoved;
            }
            ref_count = inode.ref_count.load(Ordering::Relaxed);
        }
        if (ref_count == 0) {
            self.inodes.remove(&id);
        }
        return Ok(())
    }

    pub fn open(&mut self, path: &Path, flags: open_flags, mode: mode_t) -> VfsResult<File> {
        // Applications shall specify exactly one of the first five values (file access modes)
        let unique_necessary = open_flags::O_EXEC
                                | open_flags::O_RDONLY
                                | open_flags::O_RDWR
                                | open_flags::O_SEARCH
                                | open_flags::O_WRONLY;
        if !flags.intersects(unique_necessary) {
            return Err(VfsError::Errno(Einval))
        }

        // This checks for the exclusivity of those 5 flags.
        // problably should make this clearer
        if !((flags.bits()) & (unique_necessary.bits())).is_power_of_two() {
            return Err(VfsError::Errno(Einval))
        }


        let dirent = match self.pathname_resolution(path) {
            None => {
                if flags.contains(open_flags::O_CREAT) {
                    unimplemented!()
                } else {
                    return Err(VfsError::Errno(Eacces))
                }
            },
            Some(dirent) => dirent,
        };

        if flags.contains(open_flags::O_CREAT | open_flags::O_EXCL) {
            return Err(VfsError::Errno(Eexist))
        }

        let mut file = File::from_dentry(dirent);
        let mut inode = self.get_inode_mut(dirent.header.inode_id).unwrap();// remove this unwrap
        file.flags = flags;

        (inode.inode_operations.open.unwrap())(inode, file, flags, mode)
    }

    pub fn creat(&mut self) -> VfsResult<()> {
        Result::Err(VfsError::Errno(Eopnotsupp))
    }

    pub fn close(&mut self) -> VfsResult<()> {
        Result::Err(VfsError::Errno(Eopnotsupp))
    }

    pub fn read(&mut self) -> VfsResult<()> {
        Result::Err(VfsError::Errno(Eopnotsupp))
    }

    pub fn write(&mut self) -> VfsResult<()> {
        Result::Err(VfsError::Errno(Eopnotsupp))
    }

    pub fn truncate(&mut self) -> VfsResult<()> {
        Result::Err(VfsError::Errno(Eopnotsupp))
    }

    pub fn unlink(&mut self) -> VfsResult<()> {
        Result::Err(VfsError::Errno(Eopnotsupp))
    }

    pub fn link(&mut self) -> VfsResult<()> {
        Result::Err(VfsError::Errno(Eopnotsupp))
    }

    pub fn symlink(&mut self) -> VfsResult<()> {
        Result::Err(VfsError::Errno(Eopnotsupp))
    }

    pub fn rename(&mut self) -> VfsResult<()> {
        Result::Err(VfsError::Errno(Eopnotsupp))
    }

    pub fn stat(&mut self) -> VfsResult<()> {
        Result::Err(VfsError::Errno(Eopnotsupp))
    }

    pub fn mkdir(&mut self) -> VfsResult<()> {
        Result::Err(VfsError::Errno(Eopnotsupp))
    }

    pub fn rmdir(&mut self) -> VfsResult<()> {
        Result::Err(VfsError::Errno(Eopnotsupp))
    }

    pub fn chmod(&mut self) -> VfsResult<()> {
        Result::Err(VfsError::Errno(Eopnotsupp))
    }

    pub fn chown(&mut self) -> VfsResult<()> {
        Result::Err(VfsError::Errno(Eopnotsupp))
    }
}

use core::fmt::{Display, Error, Formatter};

impl Display for VirtualFileSystem {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        writeln!(f, "Mounted filesystems:")?;
        for filesystem in self.mounted_filesystems.iter() {
            writeln!(f, "   {}", filesystem.get_name())?;
        }

        // self.root_dentry.walk_tree(&mut |entry: &DirectoryEntry| writeln!(f, "-{}-", unsafe { entry.get_filename() } ))?;

        Ok(())
    }
}

pub fn init() -> VfsResult<VirtualFileSystem> {
    let vfs = VirtualFileSystem::new()?;

    println!("{}", vfs);
    let result: Result<(), VfsError> = vfs.dcache.walk_tree_mut(&mut |entry| { entry.add_direntry(DirectoryEntry::new("lol", Filetype::Directory, InodeId::new(5, 0))?);; Ok(())});

    println!("{}", vfs);
    println!("pathname resolution {:?}", vfs.pathname_resolution("/test/lol"));
    println!("pathname resolution {:?}", vfs.pathname_resolution("/test1/lol"));
    println!("pathname resolution {:?}", vfs.pathname_resolution("/test/dlol"));

    Ok(vfs)
}
