use super::direntry::DirectoryEntryId;
pub type UserId = usize;
pub type GroupId = usize;


pub struct Current {
    pub cwd: DirectoryEntryId,
    pub uid: UserId,
    pub euid: UserId,
    pub gid: GroupId,
    pub egid: GroupId,
}
