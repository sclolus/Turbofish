use super::permissions::FilePermissions;
use super::posix_consts::time_t;
use super::user::{GroupId, UserId};
pub type InodeNumber = usize;
use super::DefaultDriver;
use super::Driver;
// use super::{FileSystemId, VfsError, VfsHandler, VfsHandlerKind, VfsHandlerParams, VfsResult};
use super::FileSystemId;
use alloc::sync::Arc;
use sync::DeadMutex;

#[derive(Debug)]
pub struct Inode {
    inode_data: InodeData,
    pub inode_operations: Arc<DeadMutex<dyn Driver>>,
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
    pub fn new(inode_operations: Arc<DeadMutex<dyn Driver>>, inode_data: InodeData) -> Self {
        Self {
            inode_data,
            inode_operations,
        }
    }
    pub fn root_inode() -> Self {
        Self {
            inode_data: InodeData::root_inode(),
            // TODO: FallibleArc
            inode_operations: Arc::new(DeadMutex::new(DefaultDriver)),
        }
    }
}

impl Default for Inode {
    fn default() -> Self {
        Self {
            inode_data: Default::default(),
            // TODO: FallibleArc
            inode_operations: Arc::new(DeadMutex::new(DefaultDriver)),
        }
    }
}

#[derive(Default, Debug)]
pub struct InodeData {
    /// This inode's id.
    pub id: InodeId,

    /// This inode's hard link number
    pub link_number: usize,
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
}

impl InodeData {
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
        let access_mode = FilePermissions::S_IRWXU | FilePermissions::S_IFDIR;

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
//     pub chmod: Option<fn(&mut Inode, &mut DirectoryEntry, FilePermissions) -> VfsResult<i32>>,
//     pub chown: Option<fn(&mut Inode, &mut DirectoryEntry, UserId, GroupId) -> VfsResult<i32>>,
//     pub lchown: Option<fn(&mut Inode, &mut DirectoryEntry, UserId, GroupId) -> VfsResult<i32>>, // probably can implement this with just chown on VFS' side.
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
//     pub fchmod: Option<fn(&mut File, FilePermissions) -> VfsResult<i32>>,
//     pub fchown: Option<fn(&mut File, UserId, GroupId) -> VfsResult<i32>>,
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
