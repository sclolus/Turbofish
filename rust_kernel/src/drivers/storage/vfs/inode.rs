#[deny(missing_docs)]

use super::DeviceId;
pub type off_t = usize;
pub type inode_number = usize;
pub type time_t = usize;
pub type uid_t = usize;
pub type gid_t = usize;

use super::Path;
use super::direntry::{DirectoryEntry, DirectoryEntryId};

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

pub struct InodeOperations {
    pub lookup_direntry: fn(&Inode, &Path) -> Option<DirectoryEntry>,
    pub lookup_inode: fn(InodeId) -> Option<Inode>,
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

    pub inode_operations: InodeOperations,
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
        }
    }


}

#[repr(u32)]
pub enum SeekType {
    SEEK_SET,
    SEEK_CUR,
    SEEK_END,
}

pub struct FileOperations {
    pub read: Option<fn(&mut File, &mut [u8], usize)>,
    // pub open: Option<fn(&mut )>,
    pub llseek: Option<fn(&mut File, usize, SeekType)>,
    // pub flush: Option<fn()>,
    pub write: Option<fn(&mut File, &mut [u8], usize)>,
    pub release: Option<fn(&mut File)>,
    // read: fn(),
    // read: fn(),
    // read: fn(),
    // read: fn(),
    // read: fn(),
    // read: fn(),
}
