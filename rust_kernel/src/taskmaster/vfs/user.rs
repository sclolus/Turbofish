use super::direntry::DirectoryEntryId;
pub type UserId = usize;
pub type GroupId = usize;
use alloc::collections::BTreeMap;

use super::fildes::{Fd, Fildes, KeyGenerator, Mapper, MapperResult};

pub struct Current {
    pub cwd: DirectoryEntryId,
    pub uid: UserId,
    pub euid: UserId,
    pub gid: GroupId,
    pub egid: GroupId,
    pub open_fds: BTreeMap<Fd, Fildes>,
}

impl KeyGenerator<Fd> for Current {
    fn gen_filter(&self, fd: Fd) -> bool {
        !self.open_fds.contains_key(&fd)
    }
}

impl Mapper<Fd, Fildes> for Current {
    fn get_map(&mut self) -> &mut BTreeMap<Fd, Fildes> {
        &mut self.open_fds
    }
}

impl Current {
    pub fn add_fd(&mut self, fildes: Fildes) -> MapperResult<Fd> {
        self.add_entry(fildes)
    }

    pub fn remove_fd(&mut self, fd: Fd) -> MapperResult<Fildes> {
        self.remove_entry(fd)
    }
}
