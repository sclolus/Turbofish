pub type InodeNumber = u32;
use super::DeadFileSystem;
use super::DefaultDriver;
use super::Driver;
use super::FileSystem;
use crate::taskmaster::SysResult;
// use super::{FileSystemId, VfsError, VfsHandler, VfsHandlerKind, VfsHandlerParams, VfsResult};
use super::FileSystemId;
use alloc::boxed::Box;
use alloc::sync::Arc;
use libc_binding::{
    dev_t, gid_t, ino_t, mode_t, nlink_t, off_t, stat, time_t, timespec, uid_t, FileType,
};
use sync::DeadMutex;

#[derive(Debug)]
pub struct Inode {
    inode_data: InodeData,
    pub driver: Box<dyn Driver>,
    pub filesystem: Arc<DeadMutex<dyn FileSystem>>,
}

use core::ops::{Deref, DerefMut};

impl Deref for Inode {
    type Target = InodeData;
    fn deref(&self) -> &Self::Target {
        &self.inode_data
    }
}

impl DerefMut for Inode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inode_data
    }
}

impl Inode {
    pub fn new(
        filesystem: Arc<DeadMutex<dyn FileSystem>>,
        driver: Box<dyn Driver>,
        inode_data: InodeData,
    ) -> Self {
        Self {
            inode_data,
            filesystem,
            driver,
        }
    }
    pub fn root_inode() -> Self {
        Self {
            inode_data: InodeData::root_inode(),
            // TODO: FallibleArc
            driver: Box::new(DefaultDriver),
            filesystem: Arc::new(DeadMutex::new(DeadFileSystem)),
        }
    }
    pub fn stat(&self, stat: &mut stat) -> SysResult<u32> {
        self.inode_data.stat(stat)
    }
    pub fn write(&mut self, offset: &mut u64, buf: &[u8]) -> SysResult<u32> {
        Ok(self
            .filesystem
            .lock()
            .write(self.id.inode_number, offset, buf)? as u32)
    }
    pub fn read(&mut self, offset: &mut u64, buf: &mut [u8]) -> SysResult<u32> {
        Ok(self
            .filesystem
            .lock()
            .read(self.id.inode_number, offset, buf)? as u32)
    }
}

impl Default for Inode {
    fn default() -> Self {
        Self {
            inode_data: Default::default(),
            // TODO: FallibleArc
            driver: Box::new(DefaultDriver),
            filesystem: Arc::new(DeadMutex::new(DeadFileSystem)),
        }
    }
}

#[derive(Default, Debug)]
pub struct InodeData {
    /// This inode's id.
    pub id: InodeId,

    /// This inode's hard link number
    pub link_number: nlink_t,
    pub access_mode: FileType,
    // pub file_type: Filetype, ??????????
    pub uid: uid_t,
    pub gid: gid_t,

    pub atime: time_t,
    pub mtime: time_t,
    pub ctime: time_t,

    pub size: u64,
    // pub status: InodeStatus,
    // pub ref_count: AtomicU32,;
}

impl InodeData {
    pub fn stat(&self, stat: &mut stat) -> SysResult<u32> {
        *stat = stat {
            st_dev: 42 as dev_t,                   // Device ID of device containing file.
            st_ino: self.id.inode_number as ino_t, // File serial number.
            st_mode: self.access_mode.bits() as mode_t, // Mode of file (see below).
            st_nlink: self.link_number,            // Number of hard links to the file.
            st_uid: self.uid,                      // User ID of file.
            st_gid: self.gid,                      // Group ID of file.
            st_rdev: 0 as dev_t, //TODO // Device ID (if file is character or block special).
            st_size: self.size as off_t, // For regular files, the file size in bytes.
            st_atim: timespec {
                // Last data access timestamp.
                tv_sec: self.atime as time_t,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: self.mtime as time_t,
                tv_nsec: 0,
            }, // Last data modification timestamp.
            st_ctim: timespec {
                tv_sec: self.ctime as time_t,
                tv_nsec: 0,
            }, // Last file status change timestamp.
            st_blksize: 42, //self.ext2.lock().get_block_size() as blksize_t, // A file system-specific preferred I/O block size
            st_blocks: 42, //self.nbr_disk_sectors as blkcnt_t, // Number of blocks allocated for this object.
        };
        Ok(0)
    }
    // Builder Pattern
    pub fn set_id(&mut self, id: InodeId) -> &mut Self {
        self.id = id;
        self
    }

    pub fn set_access_mode(&mut self, mode: FileType) -> &mut Self {
        self.access_mode = mode;
        self
    }

    pub fn set_uid(&mut self, uid: uid_t) -> &mut Self {
        self.uid = uid;
        self
    }

    pub fn set_gid(&mut self, gid: gid_t) -> &mut Self {
        self.gid = gid;
        self
    }

    // Builder Pattern end

    pub fn get_id(&self) -> InodeId {
        self.id
    }

    // pub fn set_inode_operations(
    //     &mut self,
    //     inode_operations: Arc<DeadMutex<dyn Driver>>,
    // ) -> &mut Self {
    //     self.inode_operations = inode_operations;
    //     self
    // }

    pub fn root_inode() -> Self {
        let access_mode = FileType::S_IRWXU | FileType::DIRECTORY;

        Self {
            id: InodeId::new(2, None),
            link_number: 1,
            access_mode,
            uid: 0,
            gid: 0,
            atime: 0,
            ctime: 0,
            mtime: 0,
            size: 4096,
        }
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

    // pub fn dispatch_handler(
    //     &self,
    //     params: VfsHandlerParams,
    //     kind: VfsHandlerKind,
    // ) -> VfsResult<i32> {
    //     use VfsHandlerKind::*;
    //     let ops = self.inode_operations;
    //     match kind {
    //         // Open => ops.open.ok_or(VfsError::UndefinedHandler)?(params),
    //         // LookupInode => ops.lookup_inode.ok_or(VfsError::UndefinedHandler)?(params),
    //         // LookupEntries => ops.lookup_entries.ok_or(VfsError::UndefinedHandler)?(params),
    //         // Creat => ops.creat.ok_or(VfsError::UndefinedHandler)?(params),
    //         // Rename => ops.rename.ok_or(VfsError::UndefinedHandler)?(params),
    //         // Chmod => ops.chmod.ok_or(VfsError::UndefinedHandler)?(params),
    //         // Chown => ops.chown.ok_or(VfsError::UndefinedHandler)?(params),
    //         // Lchown => ops.lchown.ok_or(VfsError::UndefinedHandler)?(params),
    //         // Truncate => ops.truncate.ok_or(VfsError::UndefinedHandler)?(params),
    //         TestOpen => ops.test_open.ok_or(VfsError::UndefinedHandler)?(params),
    //         _ => unimplemented!(),
    //     }
    // }
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct InodeId {
    pub inode_number: InodeNumber,
    //TODO: VFS Option<FileSystemId>
    pub filesystem_id: Option<FileSystemId>,
}

impl InodeId {
    pub fn new(inode_number: InodeNumber, filesystem_id: Option<FileSystemId>) -> Self {
        Self {
            inode_number,
            filesystem_id,
        }
    }
}

// #[derive(Default, Copy, Clone)]
// pub struct InodeOperations {
//     pub open: Option<fn(&mut Inode, &mut File) -> VfsResult<i32>>,
//     pub lookup_direntry: Option<fn(&Inode, &Filename) -> Option<DirectoryEntry>>,
//     pub lookup_inode: Option<fn(InodeId) -> Option<Inode>>,

//     // This is temporary.
//     pub lookup_entries: Option<fn(&Inode) -> Vec<DirectoryEntry>>,
//     // pub creat: Option<fn(Inode, &mut DirectoryEntry, DirectoryEntry, mode_t) -> VfsResult<impl Into<Inode>>>,
//     // pub link: Option<fn(&mut Inode, &mut DirectoryEntry, DirectoryEntry) -> VfsResult<i32>>,
//     // pub symlink: Option<fn(&mut Inode, &mut DirectoryEntry, DirectoryEntry) -> VfsResult<i32>>,
//     pub rename: Option<fn(&mut Inode, &mut DirectoryEntry, DirectoryEntry) -> VfsResult<i32>>,
//     // pub stat: Option<fn(&mut Inode, &mut DirectoryEntry, &mut UserStat) -> VfsResult<i32>>,
//     // pub mkdir: Option<fn(&mut Inode, &mut DirectoryEntry, mode_t) -> VfsResult<i32>>,
//     // pub rmdir: Option<fn(&mut Inode, &mut DirectoryEntry) -> VfsResult<i32>>,
//     pub chmod: Option<fn(&mut Inode, &mut DirectoryEntry, FileType) -> VfsResult<i32>>,
//     pub chown: Option<fn(&mut Inode, &mut DirectoryEntry, uid_t, gid_t) -> VfsResult<i32>>,
//     pub lchown: Option<fn(&mut Inode, &mut DirectoryEntry, uid_t, gid_t) -> VfsResult<i32>>, // probably can implement this with just chown on VFS' side.
//     pub truncate: Option<fn(&mut Inode, &mut DirectoryEntry, Offset) -> VfsResult<i32>>,

//     pub test_open: Option<fn(params: VfsHandlerParams) -> VfsResult<i32>>,
// }

// impl InodeOperations {
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

// pub fn set_test_open(mut self, test_open: VfsHandler<i32>) -> Self {
//     self.test_open = Some(test_open);
//     self
// }

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
// }

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

//make some tests
// /// The structure defining an `Open File Description` for a file.
// pub struct File {
//     /// The id of the inode.
//     pub id: InodeId,

//     /// The id of the directory entry associated with the Open File Description.
//     pub dentry_id: DirectoryEntryId,

//     pub offset: usize,
//     pub flags: OpenFlags,
// }

// impl File {
//     pub fn new(id: InodeId, dentry_id: DirectoryEntryId) -> Self {
//         Self {
//             id,
//             dentry_id,
//             offset: 0,
//             flags: OpenFlags::default(),
//         }
//     }
// }

// pub type Offset = usize; //TODO:  change this

// /// Filesystem specific operations on 'OpenFileDescriptions' `File`s
// #[allow(unused)] // TODO: remove this
// pub struct FileOperations {
//     pub read: Option<fn(&mut File, &mut [u8]) -> VfsResult<isize>>,
//     pub lseek: Option<fn(&mut File, Offset, Whence) -> Offset>,
//     // pub flush: Option<fn()>,
//     pub write: Option<fn(&mut File, &mut [u8]) -> VfsResult<isize>>,
//     pub release: Option<fn(&mut File) -> VfsResult<i32>>,
//     pub ftruncate: Option<fn(&mut File, Offset) -> VfsResult<i32>>,
//     // pub fstat: Option<fn(&mut File, &mut UserStat) -> VfsResult<i32>>,
//     pub fchmod: Option<fn(&mut File, FileType) -> VfsResult<i32>>,
//     pub fchown: Option<fn(&mut File, uid_t, gid_t) -> VfsResult<i32>>,
//     // pub open: Option<fn(&mut Inode, &mut File, i32, mode_t) -> VfsResult<i32>>,
// }

#[cfg(test)]
mod test {
    // use super::VfsHandlerParams;
    use super::*;

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

    // fn test_open(_params: VfsHandlerParams) -> VfsResult<i32> {
    //     Ok(0)
    // }

    // make_test! {
    //     {
    //         let mut inode = Inode::default();
    //         let mut file = File::new(InodeId::new(0, FileSystemId::new(0)), DirectoryEntryId::new(0));

    //         let mut inode_operations = InodeOperations::default()
    //             .set_test_open(test_open);

    //         inode.set_inode_operations(inode_operations);
    //         let params = VfsHandlerParams::new()
    //             .set_inode(&inode)
    //             .set_file(&file);

    //         let res = inode.dispatch_handler(params, VfsHandlerKind::TestOpen).unwrap();
    //         assert_eq!(res, 0);
    //     }, inode_open
    // }
}
