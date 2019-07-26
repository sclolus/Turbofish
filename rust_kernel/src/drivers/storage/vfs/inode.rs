#[deny(missing_docs)]

use super::DeviceId;
use super::{off_t, time_t, uid_t, gid_t};
use super::{Path, VfsResult, open_flags};
use super::direntry::{DirectoryEntry, DirectoryEntryId};
use super::stat::UserStat;

use core::sync::atomic::{AtomicU32, Ordering};

pub type inode_number = usize;

use bitflags::bitflags;

bitflags! {
    #[allow(snake_case)]
    pub struct mode_t: u32 {
        /// Read, write, execute/search by owner.
        const S_IRWXU = 0o700;

        /// Read permission, owner.
        const S_IRUSR = 0o400;

        /// Write permission, owner.
        const S_IWUSR = 0o200;

        /// Execute/search permission, owner.
        const S_IXUSR = 0o100;

        /// Read, write, execute/search by group.
        const S_IRWXG = 0o70;

        /// Read permission, group.
        const S_IRGRP = 0o40;

        /// Write permission, group.
        const S_IWGRP = 0o20;

        /// Execute/search permission, group.
        const S_IXGRP = 0o10;

        /// Read, write, execute/search by others.
        const S_IRWXO = 0o7;

        ///Read permission, others.
        const S_IROTH = 0o4;

        /// Write permission, others.
        const S_IWOTH = 0o2;

        /// Execute/search permission, others.
        const S_IXOTH = 0o1;

        /// Set-user-ID on execution.
        const S_ISUID = 0o4000;

        /// Set-group-ID on execution.
        const S_ISGID = 0o2000;

        /// On directories, restricted deletion flag.   [Option End]
        const S_ISVTX = 0o1000;
    }
}

#[derive(Default, Copy, Clone)]
pub struct InodeOperations {
    pub lookup_direntry: Option<fn(&Inode, &Path) -> Option<DirectoryEntry>>,
    pub lookup_inode: Option<fn(InodeId) -> Option<Inode>>,
    pub creat: Option<fn(Inode, &mut DirectoryEntry, DirectoryEntry, mode_t) -> VfsResult<impl Into<Inode>>>,
    pub link: Option<fn(&mut Inode, &mut DirectoryEntry, DirectoryEntry) -> VfsResult<i32>>,
    pub symlink: Option<fn(&mut Inode, &mut DirectoryEntry, DirectoryEntry) -> VfsResult<i32>>,
    pub rename: Option<fn(&mut Inode, &mut DirectoryEntry, DirectoryEntry) -> VfsResult<i32>>,
    pub stat: Option<fn(&mut Inode, &mut DirectoryEntry, &mut UserStat) -> VfsResult<i32>>,
    pub mkdir: Option<fn(&mut Inode, &mut DirectoryEntry, mode_t) -> VfsResult<i32>>,
    pub rmdir: Option<fn(&mut Inode, &mut DirectoryEntry) -> VfsResult<i32>>,
    pub chmod: Option<fn(&mut Inode, &mut DirectoryEntry, mode_t) -> VfsResult<i32>>,
    pub chown: Option<fn(&mut Inode, &mut DirectoryEntry, uid_t, gid_t) -> VfsResult<i32>>,
    pub lchown: Option<fn(&mut Inode, &mut DirectoryEntry, uid_t, gid_t) -> VfsResult<i32>>, // probably can implement this with just chown on VFS' side.
    pub truncate: Option<fn(&mut Inode, &mut DirectoryEntry, off_t) -> VfsResult<i32>>,
}

#[derive(Debug, Copy, Clone)]
pub struct InodeId
{
    pub device_id: DeviceId,
    pub inode_number: inode_number,
}

impl InodeId {
    pub fn new(inode_number: inode_number, device_id: DeviceId) -> Self {
        Self {
            device_id,
            inode_number
        }
    }

    pub fn device_id(&self) -> DeviceId {
        self.device_id
    }

    pub fn inode_number(&self) -> inode_number {
        self.inode_number
    }
}

pub enum InodeStatus {
    Normal,
    ToBeRemoved,
}
pub struct Inode {
    /// This inode's id.
    pub id: InodeId,

    /// This inode's hard link number
    pub link_number: usize,
    pub opened_by: usize,
    pub access_mode: mode_t,

    pub uid: uid_t,
    pub gid: gid_t,

    pub atime: time_t,
    pub mtime: time_t,
    pub ctime: time_t,

    pub size: usize,
    pub status: InodeStatus,
    pub ref_count: AtomicU32,
    pub inode_operations: InodeOperations,
}

impl Inode {

}

use core::cmp::{PartialEq, Eq};
impl PartialEq<InodeId> for InodeId {
    fn eq(&self, other: &Self) -> bool {
        self.device_id.eq(&other.device_id) &&
            self.inode_number.eq(&other.inode_number)
    }
}

impl Eq for InodeId {}


use core::cmp::{PartialOrd, Ordering};
impl PartialOrd<Self> for InodeId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let major = self.device_id.partial_cmp(&other.device_id);

        if let Ordering::Equal = major.unwrap() { // partial_cmp on inode must never fail.
            self.inode_number.partial_cmp(&other.inode_number)
        } else {
            major
        }
    }
}

use core::cmp::{Ord};
impl Ord for InodeId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap() // partial_cmp on inode must never fail.
    }
}

//make some tests
/// The structure defining an `Open File Description` for a file.
pub struct File {
    /// The id of the inode.
    id: InodeId,

    /// The id of the directory entry associated with the Open File Description.
    dentry_id: DirectoryEntryId,
    // status: status,
    offset: off_t,
    pub flags: open_flags,



    // file_operations: FileOperations,
}

impl File {
    // pub fn minimal() -> Self {
    //     Self {
    //         id,
    //         offset: 0,
    //     }
    // }

    pub fn from_dentry(dentry: &DirectoryEntry) -> Self {
        Self {
            id: dentry.header.inode_id,
            dentry_id: dentry.header.id,
            offset: 0,
            flags: Default::default(),
        }
    }


}

#[repr(u32)]
pub enum SeekType {
    SEEK_SET,
    SEEK_CUR,
    SEEK_END,
}

type ssize_t = i64;

pub struct FileOperations {
    pub read: Option<fn(&mut File, &mut [u8]) -> VfsResult<ssize_t>>,
    pub lseek: Option<fn(&mut File, off_t, SeekType) -> off_t>,
    // pub flush: Option<fn()>,
    pub write: Option<fn(&mut File, &mut [u8]) -> VfsResult<ssize_t>>,
    pub release: Option<fn(&mut File) -> VfsResult<i32>>,
    pub ftruncate: Option<fn(&mut File, off_t) -> VfsResult<i32>>,
    pub fstat: Option<fn(&mut File, &mut UserStat) -> VfsResult<i32>>,
    pub fchmod: Option<fn(&mut File, mode_t) -> VfsResult<i32>>,
    pub fchown: Option<fn(&mut File, uid_t, gid_t) -> VfsResult<i32>>,
    pub open: Option<fn(&mut Inode, &mut File, i32, mode_t) -> VfsResult<i32>>,
}
