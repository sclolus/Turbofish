use super::direntry::{DirectoryEntry, DirectoryEntryId};
use super::path::Filename;
use super::permissions::FilePermissions;
use super::posix_consts::time_t;
use super::user::{GroupId, UserId};
use alloc::vec::Vec;
pub type InodeNumber = usize;
use super::{FileSystemId, VfsError, VfsHandler, VfsHandlerKind, VfsHandlerParams, VfsResult};
use bitflags::bitflags;
// use libc_binding::ssize_t;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct InodeId {
    pub inode_number: InodeNumber,
    pub filesystem_id: FileSystemId,
}

impl InodeId {
    pub fn new(inode_number: InodeNumber, filesystem_id: FileSystemId) -> Self {
        Self {
            inode_number,
            filesystem_id,
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct InodeOperations {
    pub open: Option<fn(&mut Inode, &mut File) -> VfsResult<i32>>,
    pub lookup_direntry: Option<fn(&Inode, &Filename) -> Option<DirectoryEntry>>,
    pub lookup_inode: Option<fn(InodeId) -> Option<Inode>>,

    // This is temporary.
    pub lookup_entries: Option<fn(&Inode) -> Vec<DirectoryEntry>>,
    // pub creat: Option<fn(Inode, &mut DirectoryEntry, DirectoryEntry, mode_t) -> VfsResult<impl Into<Inode>>>,
    // pub link: Option<fn(&mut Inode, &mut DirectoryEntry, DirectoryEntry) -> VfsResult<i32>>,
    // pub symlink: Option<fn(&mut Inode, &mut DirectoryEntry, DirectoryEntry) -> VfsResult<i32>>,
    pub rename: Option<fn(&mut Inode, &mut DirectoryEntry, DirectoryEntry) -> VfsResult<i32>>,
    // pub stat: Option<fn(&mut Inode, &mut DirectoryEntry, &mut UserStat) -> VfsResult<i32>>,
    // pub mkdir: Option<fn(&mut Inode, &mut DirectoryEntry, mode_t) -> VfsResult<i32>>,
    // pub rmdir: Option<fn(&mut Inode, &mut DirectoryEntry) -> VfsResult<i32>>,
    pub chmod: Option<fn(&mut Inode, &mut DirectoryEntry, FilePermissions) -> VfsResult<i32>>,
    pub chown: Option<fn(&mut Inode, &mut DirectoryEntry, UserId, GroupId) -> VfsResult<i32>>,
    pub lchown: Option<fn(&mut Inode, &mut DirectoryEntry, UserId, GroupId) -> VfsResult<i32>>, // probably can implement this with just chown on VFS' side.
    pub truncate: Option<fn(&mut Inode, &mut DirectoryEntry, Offset) -> VfsResult<i32>>,

    pub test_open: Option<fn(params: VfsHandlerParams) -> VfsResult<i32>>,
}

impl InodeOperations {
    // pub fn set_open(mut self, open: VfsHandler<i32>) -> Self {
    //     self.open = Some(open);
    //     self
    // }

    // pub fn set_lookup_inode(mut self, lookup_inode: VfsHandler<i32>) -> Self {
    //     self.lookup_inode = Some(lookup_inode);
    //     self
    // }

    // pub fn set_lookup_entries(mut self, lookup_entries: VfsHandler<i32>) -> Self {
    //     self.lookup_entries = Some(lookup_entries);
    //     self
    // }

    // pub fn set_creat(mut self, creat: VfsHandler<i32>) -> Self {
    //     self.creat = Some(creat);
    //     self
    // }

    // pub fn set_rename(mut self, rename: VfsHandler<i32>) -> Self {
    //     self.rename = Some(rename);
    //     self
    // }

    // pub fn set_chmod(mut self, chmod: VfsHandler<i32>) -> Self {
    //     self.chmod = Some(chmod);
    //     self
    // }

    // pub fn set_chown(mut self, chown: VfsHandler<i32>) -> Self {
    //     self.chown = Some(chown);
    //     self
    // }

    // pub fn set_lchown(mut self, lchown: VfsHandler<i32>) -> Self {
    //     self.lchown = Some(lchown);
    //     self
    // }

    // pub fn set_truncate(mut self, truncate: VfsHandler<i32>) -> Self {
    //     self.truncate = Some(truncate);
    //     self
    // }

    pub fn set_test_open(mut self, test_open: VfsHandler<i32>) -> Self {
        self.test_open = Some(test_open);
        self
    }

    // pub fn unset_open(mut self) -> Self {
    //     self.open = None;
    //     self
    // }

    // pub fn unset_lookup_inode(mut self) -> Self {
    //     self.lookup_inode = None;
    //     self
    // }

    // pub fn unset_lookup_entries(mut self) -> Self {
    //     self.lookup_entries = None;
    //     self
    // }

    // // pub fn unset_creat(mut self) -> Self {
    // //     self.creat = None;
    // //     self
    // // }

    // pub fn unset_rename(mut self) -> Self {
    //     self.rename = None;
    //     self
    // }

    // pub fn unset_chmod(mut self) -> Self {
    //     self.chmod = None;
    //     self
    // }

    // pub fn unset_chown(mut self) -> Self {
    //     self.chown = None;
    //     self
    // }

    // pub fn unset_lchown(mut self) -> Self {
    //     self.lchown = None;
    //     self
    // }

    // pub fn unset_truncate(mut self) -> Self {
    //     self.truncate = None;
    //     self
    // }

    // pub fn unset_test_open(mut self) -> Self {
    //     self.test_open = None;
    //     self
    // }
}

// /// Type of file
// #[derive(Debug, Copy, Clone, PartialEq)]
// pub enum Filetype {
//     RegularFile,
//     Directory,
//     CharacterDevice,
//     BlockDevice,
//     Fifo,
//     Socket,
//     SymbolicLink,
// }

#[derive(Default)]
pub struct Inode {
    /// This inode's id.
    pub id: InodeId,

    /// This inode's hard link number
    pub link_number: usize,
    opened_by: usize,
    pub access_mode: FilePermissions,
    // pub file_type: Filetype, ??????????
    pub uid: UserId,
    pub gid: GroupId,

    pub atime: time_t,
    pub mtime: time_t,
    pub ctime: time_t,

    pub size: usize,
    // pub status: InodeStatus,
    // pub ref_count: AtomicU32,;
    pub inode_operations: InodeOperations,
}

impl Inode {
    // Builder Pattern
    pub fn set_id(&mut self, id: InodeId) -> &mut Self {
        self.id = id;
        self
    }

    pub fn set_access_mode(&mut self, mode: FilePermissions) -> &mut Self {
        self.access_mode = mode;
        self
    }

    pub fn set_uid(&mut self, uid: UserId) -> &mut Self {
        self.uid = uid;
        self
    }

    pub fn set_gid(&mut self, gid: GroupId) -> &mut Self {
        self.gid = gid;
        self
    }

    pub fn root_inode() -> Self {
        let access_mode = FilePermissions::S_IRWXU | FilePermissions::S_IFDIR;

        Self {
            id: InodeId::new(2, FileSystemId::new(0)),
            link_number: 1,
            opened_by: 0,
            access_mode,
            uid: 0,
            gid: 0,
            atime: 0,
            ctime: 0,
            mtime: 0,
            size: 4096,
            inode_operations: Default::default(),
        }
    }

    pub fn set_inode_operations(&mut self, inode_operations: InodeOperations) -> &mut Self {
        self.inode_operations = inode_operations;
        self
    }
    // Builder Pattern end

    pub fn is_opened(&self) -> bool {
        self.opened_by == 0
    }

    // Explain this
    pub fn open(&mut self) {
        self.opened_by += 1;
    }

    pub fn is_character_device(&self) -> bool {
        self.access_mode.is_character_device()
    }

    pub fn is_fifo(&self) -> bool {
        self.access_mode.is_fifo()
    }

    pub fn is_regular(&self) -> bool {
        self.access_mode.is_regular()
    }

    pub fn is_directory(&self) -> bool {
        self.access_mode.is_directory()
    }

    pub fn is_symlink(&self) -> bool {
        self.access_mode.is_symlink()
    }

    pub fn is_socket(&self) -> bool {
        self.access_mode.is_socket()
    }

    pub fn dispatch_handler(
        &self,
        params: VfsHandlerParams,
        kind: VfsHandlerKind,
    ) -> VfsResult<i32> {
        use VfsHandlerKind::*;
        let ops = self.inode_operations;
        match kind {
            // Open => ops.open.ok_or(VfsError::UndefinedHandler)?(params),
            // LookupInode => ops.lookup_inode.ok_or(VfsError::UndefinedHandler)?(params),
            // LookupEntries => ops.lookup_entries.ok_or(VfsError::UndefinedHandler)?(params),
            // Creat => ops.creat.ok_or(VfsError::UndefinedHandler)?(params),
            // Rename => ops.rename.ok_or(VfsError::UndefinedHandler)?(params),
            // Chmod => ops.chmod.ok_or(VfsError::UndefinedHandler)?(params),
            // Chown => ops.chown.ok_or(VfsError::UndefinedHandler)?(params),
            // Lchown => ops.lchown.ok_or(VfsError::UndefinedHandler)?(params),
            // Truncate => ops.truncate.ok_or(VfsError::UndefinedHandler)?(params),
            TestOpen => ops.test_open.ok_or(VfsError::UndefinedHandler)?(params),
            _ => unimplemented!(),
        }
    }
}

//make some tests
/// The structure defining an `Open File Description` for a file.
pub struct File {
    /// The id of the inode.
    pub id: InodeId,

    /// The id of the directory entry associated with the Open File Description.
    pub dentry_id: DirectoryEntryId,

    pub offset: usize,
    pub flags: OpenFlags,
}

impl File {
    pub fn new(id: InodeId, dentry_id: DirectoryEntryId) -> Self {
        Self {
            id,
            dentry_id,
            offset: 0,
            flags: OpenFlags::default(),
        }
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum SeekType {
    SeekSet,
    SeekCur,
    SeekEnd,
}

pub type Offset = usize; //TODO:  change this

/// Filesystem specific operations on 'OpenFileDescriptions' `File`s
#[allow(unused)] // TODO: remove this
pub struct FileOperations {
    pub read: Option<fn(&mut File, &mut [u8]) -> VfsResult<isize>>,
    pub lseek: Option<fn(&mut File, Offset, SeekType) -> Offset>,
    // pub flush: Option<fn()>,
    pub write: Option<fn(&mut File, &mut [u8]) -> VfsResult<isize>>,
    pub release: Option<fn(&mut File) -> VfsResult<i32>>,
    pub ftruncate: Option<fn(&mut File, Offset) -> VfsResult<i32>>,
    // pub fstat: Option<fn(&mut File, &mut UserStat) -> VfsResult<i32>>,
    pub fchmod: Option<fn(&mut File, FilePermissions) -> VfsResult<i32>>,
    pub fchown: Option<fn(&mut File, UserId, GroupId) -> VfsResult<i32>>,
    // pub open: Option<fn(&mut Inode, &mut File, i32, mode_t) -> VfsResult<i32>>,
}

bitflags! {
    #[derive(Default)] // I wonder for this derive <.<
    pub struct OpenFlags: u32 {
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::VfsHandlerParams;

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

    fn test_open(_params: VfsHandlerParams) -> VfsResult<i32> {
        Ok(0)
    }

    make_test! {
        {
            let mut inode = Inode::default();
            let mut file = File::new(InodeId::new(0, FileSystemId::new(0)), DirectoryEntryId::new(0));

            let mut inode_operations = InodeOperations::default()
                .set_test_open(test_open);


            inode.set_inode_operations(inode_operations);
            let params = VfsHandlerParams::new()
                .set_inode(&inode)
                .set_file(&file);

            let res = inode.dispatch_handler(params, VfsHandlerKind::TestOpen).unwrap();
            assert_eq!(res, 0);
        }, inode_open
    }
}
