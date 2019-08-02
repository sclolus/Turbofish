use super::direntry::DirectoryEntryId;
pub type UserId = usize;
pub type GroupId = usize;
use std::collections::BTreeMap;


pub struct Current {
    pub cwd: DirectoryEntryId,
    pub uid: UserId,
    pub euid: UserId,
    pub gid: GroupId,
    pub egid: GroupId,
    pub open_fds: BTreeMap<Fildes>
}
