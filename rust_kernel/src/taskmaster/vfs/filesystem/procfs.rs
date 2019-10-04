use super::IpcResult;
use super::{
    DirectoryEntry, DirectoryEntryBuilder, DirectoryEntryId, Driver, FileOperation, FileSystem,
    FileSystemId, MountedFileSystem, SysResult, PATH_MAX, VFS,
};
use super::{Filename, Inode as VfsInode, InodeData as VfsInodeData, InodeId, Path};
use crate::taskmaster::kmodules::CURRENT_UNIX_TIME;
use alloc::collections::CollectionAllocErr;
use core::sync::atomic::Ordering;

use crate::taskmaster::SCHEDULER;

use crate::taskmaster::thread_group::{ThreadGroup, ThreadGroupState};

use super::dead::DeadFileSystem;
use super::{KeyGenerator, Mapper};
use crate::taskmaster::drivers::DefaultDriver;
use crate::taskmaster::vfs::Dcache;
use alloc::{boxed::Box, vec::Vec};
use core::convert::TryFrom;
use core::ops::{Deref, DerefMut};
use fallible_collections::{
    arc::FallibleArc,
    boxed::FallibleBox,
    btree::{BTreeMap, BTreeSet},
    vec::TryCollect,
    TryClone,
};
use libc_binding::{statfs, time_t, Errno, FileType, Pid, NAME_MAX, PAGE_SIZE, PROC_SUPER_MAGIC};

use alloc::sync::Arc;
use core::default::Default;
use sync::DeadMutex;
mod procfs_driver;
pub use procfs_driver::ProcFsOperations;

mod version;
pub use version::VersionDriver;

mod filesystems;
pub use filesystems::FilesystemsDriver;

mod stat;
pub use stat::StatDriver;

mod environ;
pub use environ::EnvironDriver;

mod cmdline;
pub use cmdline::CmdlineDriver;

mod proc_stat;
pub use proc_stat::ProcStatDriver;

mod uptime;
pub use uptime::UptimeDriver;

mod loadavg;
pub use loadavg::LoadavgDriver;

mod meminfo;
pub use meminfo::MeminfoDriver;

mod vmstat;
pub use vmstat::VmstatDriver;

mod mounts;
pub use mounts::MountsDriver;

mod comm;
pub use comm::CommDriver;

mod tty_drivers;
pub use tty_drivers::TtyDriversDriver;

mod status;
pub use status::StatusDriver;

use itertools::unfold;

unsafe impl Send for ProcFs {}

#[derive(Debug)]
pub struct ProcFs {
    fs_id: FileSystemId,
    inodes: BTreeMap<InodeId, Inode>,
    root_direntry_id: DirectoryEntryId,
    root_inode_id: InodeId,
    dcache: Dcache,
    pid_directories: BTreeSet<(DirectoryEntryId, Pid)>,
    tty_directory: Option<DirectoryEntryId>,
    fd_directories: BTreeSet<(DirectoryEntryId, Pid)>,
}

impl KeyGenerator<InodeId> for ProcFs {
    fn gen_filter(&self, mut key: InodeId) -> bool {
        key.filesystem_id = Some(self.fs_id);
        !self.inodes.contains_key(&key)
    }
}
impl KeyGenerator<DirectoryEntryId> for ProcFs {
    fn gen_filter(&self, key: DirectoryEntryId) -> bool {
        !self.dcache.contains_entry(&key)
    }
}

// #[derive(Debug)]
// struct DirectoryEntry(DirectoryEntry);

// impl Deref for DirectoryEntry {
//     type Target = DirectoryEntry;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl DerefMut for DirectoryEntry {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

// impl DirectoryEntry {
//     fn root_entry() -> Self {
//         Self(DirectoryEntry::root_entry())
//     }
// }

// #[derive(Debug)]
struct Inode(
    VfsInode,
    Box<dyn FnMut(InodeId) -> Result<Box<dyn Driver>, CollectionAllocErr>>,
);

impl core::fmt::Debug for Inode {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        Ok(write!(f, "{:?}", self.0)?)
    }
}

impl Deref for Inode {
    type Target = VfsInode;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Inode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Mapper<InodeId, Inode> for ProcFs {
    fn get_map(&mut self) -> &mut BTreeMap<InodeId, Inode> {
        &mut self.inodes
    }
}

impl Mapper<DirectoryEntryId, DirectoryEntry> for ProcFs {
    fn get_map(&mut self) -> &mut BTreeMap<DirectoryEntryId, DirectoryEntry> {
        &mut self.dcache.d_entries
    }
}

impl ProcFs {
    pub fn register_file(
        &mut self,
        parent: DirectoryEntryId,
        name: Filename,
        gen_driver: Box<dyn FnMut(InodeId) -> Result<Box<dyn Driver>, CollectionAllocErr>>,
    ) -> SysResult<DirectoryEntryId> {
        let driver = Box::try_new(DefaultDriver)?;
        let filesystem = Arc::try_new(DeadMutex::new(DeadFileSystem))?;

        let mut inode_id: InodeId = self.gen();
        inode_id.filesystem_id = Some(self.fs_id);
        let access_mode = FileType::REGULAR_FILE | FileType::from_bits(0o444).unwrap();

        let vfs_inode_data = *VfsInodeData::default()
            .set_id(inode_id)
            .set_alltime(unsafe { CURRENT_UNIX_TIME.load(Ordering::Relaxed) } as time_t)
            .set_access_mode(access_mode)
            .set_link_number(1)
            .set_uid(0)
            .set_gid(0);

        let inode = Inode(
            VfsInode::new(filesystem, driver, vfs_inode_data),
            gen_driver,
        );

        let mut direntry = DirectoryEntryBuilder::new();
        direntry
            .set_filename(name)
            .set_inode_id(inode_id)
            .set_parent_id(parent)
            .set_regular();

        let direntry = direntry.build();
        let dir_id = self
            .dcache
            .add_entry(Some(parent), direntry)
            .or(Err(Errno::ENOMEM))?;

        assert!(self
            .inodes
            .try_insert(inode_id, inode)
            .or(Err(Errno::ENOMEM))?
            .is_none());
        Ok(dir_id)
    }

    pub fn register_base_drivers(&mut self) -> SysResult<()> {
        let (root_dir_id, _) = self.root_ids();
        let version_filename = Filename::from_str_unwrap("version");
        let filesystems_filename = Filename::from_str_unwrap("filesystems");
        let proc_stat_filename = Filename::from_str_unwrap("stat");
        let uptime_filename = Filename::from_str_unwrap("uptime");
        let loadavg_filename = Filename::from_str_unwrap("loadavg");
        let meminfo_filename = Filename::from_str_unwrap("meminfo");
        let vmstat_filename = Filename::from_str_unwrap("vmstat");
        let mounts_filename = Filename::from_str_unwrap("mounts");

        self.register_file(
            root_dir_id,
            filesystems_filename,
            Box::try_new(|inode_id| -> Result<Box<dyn Driver>, CollectionAllocErr> {
                Ok(Box::try_new(filesystems::FilesystemsDriver::new(inode_id))? as Box<dyn Driver>)
            })?,
        )?;
        self.register_file(
            root_dir_id,
            version_filename,
            Box::try_new(|inode_id| -> Result<Box<dyn Driver>, CollectionAllocErr> {
                Ok(Box::try_new(version::VersionDriver::new(inode_id))? as Box<dyn Driver>)
            })?,
        )?;

        self.register_file(
            root_dir_id,
            proc_stat_filename,
            Box::try_new(|inode_id| -> Result<Box<dyn Driver>, CollectionAllocErr> {
                Ok(Box::try_new(proc_stat::ProcStatDriver::new(inode_id))? as Box<dyn Driver>)
            })?,
        )?;

        self.register_file(
            root_dir_id,
            uptime_filename,
            Box::try_new(|inode_id| -> Result<Box<dyn Driver>, CollectionAllocErr> {
                Ok(Box::try_new(uptime::UptimeDriver::new(inode_id))? as Box<dyn Driver>)
            })?,
        )?;

        self.register_file(
            root_dir_id,
            loadavg_filename,
            Box::try_new(|inode_id| -> Result<Box<dyn Driver>, CollectionAllocErr> {
                Ok(Box::try_new(loadavg::LoadavgDriver::new(inode_id))? as Box<dyn Driver>)
            })?,
        )?;

        self.register_file(
            root_dir_id,
            meminfo_filename,
            Box::try_new(|inode_id| -> Result<Box<dyn Driver>, CollectionAllocErr> {
                Ok(Box::try_new(meminfo::MeminfoDriver::new(inode_id))? as Box<dyn Driver>)
            })?,
        )?;

        self.register_file(
            root_dir_id,
            vmstat_filename,
            Box::try_new(|inode_id| -> Result<Box<dyn Driver>, CollectionAllocErr> {
                Ok(Box::try_new(vmstat::VmstatDriver::new(inode_id))? as Box<dyn Driver>)
            })?,
        )?;

        self.register_file(
            root_dir_id,
            mounts_filename,
            Box::try_new(|inode_id| -> Result<Box<dyn Driver>, CollectionAllocErr> {
                Ok(Box::try_new(mounts::MountsDriver::new(inode_id))? as Box<dyn Driver>)
            })?,
        )?;

        // Inserting divers basic procfs files.
        Ok(())
    }

    pub fn fill_root_dir(&mut self) -> SysResult<()> {
        SCHEDULER.force_unlock();
        let scheduler = SCHEDULER.lock();

        let thread_groups = scheduler.iter_thread_groups_with_pid();

        for (&pid, _thread_group) in thread_groups {
            self.register_pid_directory(pid)?;
        }

        self.register_self_directory()?;
        self.register_base_drivers()?;
        Ok(())
    }

    pub fn register_tty_directory(&mut self) -> SysResult<()> {
        let (root_dir_id, _) = self.root_ids();
        let tty_filename = Filename::from_str_unwrap("tty");

        let mode = FileType::DIRECTORY
            | FileType::USER_READ_PERMISSION
            | FileType::USER_EXECUTE_PERMISSION
            | FileType::GROUP_READ_PERMISSION
            | FileType::GROUP_EXECUTE_PERMISSION
            | FileType::OTHER_READ_PERMISSION
            | FileType::OTHER_EXECUTE_PERMISSION;

        let dir_id = self.mkdir(root_dir_id, tty_filename, mode)?;
        self.tty_directory = Some(dir_id);
        Ok(())
    }

    pub fn fill_tty_directory(&mut self) -> SysResult<()> {
        let tty_dir_id = self.tty_directory.expect("No tty directory registered");
        let drivers_filename = Filename::from_str_unwrap("drivers");

        self.register_file(
            tty_dir_id,
            drivers_filename,
            Box::try_new(|inode_id| -> Result<Box<dyn Driver>, CollectionAllocErr> {
                Ok(Box::try_new(tty_drivers::TtyDriversDriver::new(inode_id))? as Box<dyn Driver>)
            })?,
        )?;
        Ok(())
    }

    pub fn register_fd_directory(&mut self, pid: Pid) -> SysResult<()> {
        let fd_filename = Filename::from_str_unwrap("fd");
        let pid_dir_id = self
            .get_pid_directory(pid)
            .expect("Could not get the requested pid directory")
            .id;

        let mode = FileType::DIRECTORY
            | FileType::USER_READ_PERMISSION
            | FileType::USER_EXECUTE_PERMISSION
            | FileType::GROUP_READ_PERMISSION
            | FileType::GROUP_EXECUTE_PERMISSION
            | FileType::OTHER_READ_PERMISSION
            | FileType::OTHER_EXECUTE_PERMISSION;

        let dir_id = self.mkdir(pid_dir_id, fd_filename, mode)?;
        self.fd_directories.try_insert((dir_id, pid))?;
        Ok(())
    }

    pub fn fill_fd_directory(&mut self, pid: Pid) -> SysResult<()> {
        let fd_dir = self.get_fd_directory(pid)?;
        let fd_dir_id = fd_dir.id;
        SCHEDULER.force_unlock();
        let scheduler = SCHEDULER.lock();

        let thread_group = scheduler
            .get_thread_group(pid)
            .expect("Could not find corresponding thread group for pid");

        let state = match &thread_group.thread_group_state {
            ThreadGroupState::Running(running) => Some(&running.file_descriptor_interface),
            _ => None,
        };
        let fds = state.iter().flat_map(|x| x.iter());

        // VFS.force_unlock();
        // let vfs = VFS.lock();
        for (fd, descriptor) in fds {
            let fd_string = tryformat!(32, "{}", fd)?;
            let fd_filename = Filename::from_str_unwrap(&fd_string);
            let path = descriptor.get_open_path().try_clone()?;
            self.symlink(fd_dir_id, fd_filename, path)?;
        }
        Ok(())
    }

    pub fn register_pid_directory(&mut self, pid: Pid) -> SysResult<()> {
        let (root_dir_id, _) = self.root_ids();

        let pid_filename = {
            // 10 is len of Pid::max
            let s = tryformat!(10, "{}", pid)?;
            Filename::from_str_unwrap(s.as_str())
        };

        if self
            .children_direntries(root_dir_id)?
            .any(|entry| entry.filename == pid_filename)
        {
            return Ok(());
        }

        let mode = FileType::DIRECTORY
            | FileType::USER_READ_PERMISSION
            | FileType::USER_EXECUTE_PERMISSION
            | FileType::GROUP_READ_PERMISSION
            | FileType::GROUP_EXECUTE_PERMISSION
            | FileType::OTHER_READ_PERMISSION
            | FileType::OTHER_EXECUTE_PERMISSION;

        let dir_id = self.mkdir(root_dir_id, pid_filename, mode)?;
        self.pid_directories.try_insert((dir_id, pid))?;

        Ok(())
    }

    pub fn fill_pid_directory(&mut self, pid: Pid) -> SysResult<()> {
        let cwd_filename = Filename::from_str_unwrap("cwd");
        let stat_filename = Filename::from_str_unwrap("stat");
        let environ_filename = Filename::from_str_unwrap("environ");
        let cmdline_filename = Filename::from_str_unwrap("cmdline");
        let exe_filename = Filename::from_str_unwrap("exe");
        let comm_filename = Filename::from_str_unwrap("comm");
        let status_filename = Filename::from_str_unwrap("status");
        // let self_filename = Filename::from_str_unwrap("self");

        SCHEDULER.force_unlock();
        let scheduler = SCHEDULER.lock();

        let thread_group = scheduler
            .get_thread_group(pid)
            .expect("Could not find corresponding thread group for pid");

        let dir_id = self
            .get_pid_directory(pid)
            .expect("Could not get the requested pid directory")
            .id;
        self.register_file(
            dir_id,
            stat_filename,
            Box::try_new(
                move |inode_id| -> Result<Box<dyn Driver>, CollectionAllocErr> {
                    Ok(Box::try_new(StatDriver::new(inode_id, pid))? as Box<dyn Driver>)
                },
            )?,
        )?;

        let cwd = thread_group.cwd.try_clone()?;

        self.symlink(dir_id, cwd_filename, cwd)?;

        self.register_file(
            dir_id,
            environ_filename,
            Box::try_new(
                move |inode_id| -> Result<Box<dyn Driver>, CollectionAllocErr> {
                    Ok(Box::try_new(EnvironDriver::new(inode_id, pid))? as Box<dyn Driver>)
                },
            )?,
        )?;

        self.register_file(
            dir_id,
            cmdline_filename,
            Box::try_new(
                move |inode_id| -> Result<Box<dyn Driver>, CollectionAllocErr> {
                    Ok(Box::try_new(CmdlineDriver::new(inode_id, pid))? as Box<dyn Driver>)
                },
            )?,
        )?;

        self.register_file(
            dir_id,
            comm_filename,
            Box::try_new(
                move |inode_id| -> Result<Box<dyn Driver>, CollectionAllocErr> {
                    Ok(Box::try_new(CommDriver::new(inode_id, pid))? as Box<dyn Driver>)
                },
            )?,
        )?;

        self.register_file(
            dir_id,
            status_filename,
            Box::try_new(
                move |inode_id| -> Result<Box<dyn Driver>, CollectionAllocErr> {
                    Ok(Box::try_new(StatusDriver::new(inode_id, pid))? as Box<dyn Driver>)
                },
            )?,
        )?;

        if let Some(filename) = &thread_group.filename {
            self.symlink(dir_id, exe_filename, filename.try_clone()?)?;
        }

        self.register_fd_directory(pid)?;
        Ok(())
    }

    pub fn is_fd_directory(&self, direntry_id: DirectoryEntryId) -> bool {
        self.fd_directories.iter().any(|(id, _)| *id == direntry_id)
    }

    fn get_fd_directory(&self, pid: Pid) -> SysResult<&DirectoryEntry> {
        let (direntry_id, _) = self
            .fd_directories
            .iter()
            .find(|(_, entry_pid)| *entry_pid == pid)
            .expect("No such fd directory exists");

        self.dcache.get_entry(direntry_id)
    }

    fn get_fd_directory_entry(
        &self,
        direntry_id: DirectoryEntryId,
    ) -> SysResult<(DirectoryEntryId, Pid)> {
        Ok(*self // TODO: think about actually returning a reference
            .fd_directories
            .iter()
            .find(|(id, _)| *id == direntry_id)
            .ok_or(Errno::ENOENT)?)
    }

    pub fn is_pid_directory(&self, direntry_id: DirectoryEntryId) -> bool {
        self.pid_directories
            .iter()
            .any(|(id, _)| *id == direntry_id)
    }

    fn get_pid_directory(&self, pid: Pid) -> SysResult<&DirectoryEntry> {
        let (direntry_id, _) = self
            .pid_directories
            .iter()
            .find(|(_, entry_pid)| *entry_pid == pid)
            .expect("No such pid file exists");

        self.dcache.get_entry(direntry_id)
    }

    fn get_pid_directory_entry(
        &self,
        direntry_id: DirectoryEntryId,
    ) -> SysResult<(DirectoryEntryId, Pid)> {
        Ok(*self // TODO: think about actually returning a reference
            .pid_directories
            .iter()
            .find(|(id, _)| *id == direntry_id)
            .ok_or(Errno::ENOENT)?)
    }

    pub fn register_self_directory(&mut self) -> SysResult<()> {
        let (root_dir_id, _) = self.root_ids();
        let self_filename = Filename::from_str_unwrap("self");

        SCHEDULER.force_unlock();
        let scheduler = SCHEDULER.lock();

        let (current_pid, _) = scheduler.current_task_id();
        let pid_path = Path::try_from(tryformat!(16, "/proc/{}", current_pid)?.as_str())?;

        self.symlink(root_dir_id, self_filename, pid_path)
            .expect("Failed to creat the /proc/self symlink");
        Ok(())
    }

    pub fn new(fs_id: FileSystemId) -> SysResult<Self> {
        let mut new = Self {
            fs_id: fs_id,
            inodes: BTreeMap::new(),
            dcache: Dcache::new(),
            pid_directories: BTreeSet::new(),
            root_direntry_id: DirectoryEntryId::new(0),
            root_inode_id: InodeId::new(0, Some(fs_id)),
            tty_directory: None,
            fd_directories: BTreeSet::new(),
        };

        new.root_direntry_id = new.dcache.root_id;

        let root_dir_id = new.root_direntry_id;
        let root_direntry = new.dcache.get_entry(&root_dir_id)?;
        let root_inode_id = new.new_inode_id(root_direntry.inode_id.inode_number);
        let root_direntry = new.dcache.get_entry_mut(&root_dir_id)?;
        root_direntry.set_inode_id(root_inode_id);

        let inode = VfsInode::root_inode()?;

        new.root_inode_id = root_inode_id;

        let inode = Inode(
            inode,
            Box::try_new(|_inode_id| Ok(Box::try_new(DefaultDriver)? as Box<dyn Driver>))?,
        );

        new.inodes.try_insert(root_inode_id, inode)?;

        // new.register_base_drivers()?;
        Ok(new)
    }

    pub fn new_inode_id(&self, inode_nbr: u32) -> InodeId {
        InodeId::new(inode_nbr, Some(self.fs_id))
    }

    pub fn root_ids(&self) -> (DirectoryEntryId, InodeId) {
        (self.root_direntry_id, self.root_inode_id)
    }

    fn mkdir(
        &mut self,
        parent: DirectoryEntryId,
        filename: Filename,
        mode: FileType,
    ) -> SysResult<DirectoryEntryId> {
        let parent_dir = self.dcache.get_entry_mut(&parent)?;
        let driver = Box::try_new(DefaultDriver)?;
        let filesystem = Arc::try_new(DeadMutex::new(DeadFileSystem))?;

        parent_dir
            .get_directory_mut()
            .expect("Parent in Procfs::mkdir() should be a directory");

        let inode_id: InodeId = self.gen();
        let inode_id = self.new_inode_id(inode_id.inode_number);
        let vfs_inode_data = *VfsInodeData::default()
            .set_id(inode_id)
            .set_alltime(unsafe { CURRENT_UNIX_TIME.load(Ordering::Relaxed) } as time_t)
            .set_access_mode(mode)
            .set_link_number(1)
            .set_uid(0) //TODO change this.
            .set_gid(0);

        let inode = Inode(
            VfsInode::new(filesystem, driver, vfs_inode_data),
            Box::try_new(|_inode_id| Ok(Box::try_new(DefaultDriver)? as Box<dyn Driver>))?,
        );

        let mut direntry = DirectoryEntryBuilder::new();
        direntry
            .set_filename(filename)
            .set_inode_id(inode_id)
            .set_directory();

        let direntry = direntry.build();

        self.inodes.try_insert(inode_id, inode)?;
        match self.dcache.add_entry(Some(parent), direntry) {
            Ok(id) => Ok(id),
            Err(e) => {
                self.inodes.remove(&inode_id);
                Err(e)
            }
        }
    }

    // fn children(
    //     &self,
    //     dir_id: DirectoryEntryId,
    // ) -> SysResult<impl Iterator<Item = (&DirectoryEntry, &Inode)>> {
    //     let dcache = &self.dcache;
    //     let inodes = &self.inodes;
    //     let mut children_iter = self.dcache.get_entry(&dir_id)?.get_directory()?.entries();

    //     Ok(unfold((), move |_| match children_iter.next() {
    //         None => None,
    //         Some(id) => {
    //             let entry = match dcache.get_entry(&id) {
    //                 Ok(entry) => entry,
    //                 Err(_) => return None, // TODO: change this maybe
    //             };
    //             // eprintln!("Searching {:?}", entry.inode_id);
    //             let inode = inodes
    //                 .get(&entry.inode_id)
    //                 .expect("No corresponding inode for direntry");
    //             Some((entry, inode))
    //         }
    //     }))
    // }

    fn children_direntries(
        &self,
        dir_id: DirectoryEntryId,
    ) -> SysResult<impl Iterator<Item = &DirectoryEntry>> {
        let dcache = &self.dcache;
        let mut children_iter = self.dcache.get_entry(&dir_id)?.get_directory()?.entries();

        Ok(unfold((), move |_| match children_iter.next() {
            None => None,
            Some(id) => {
                let entry = match dcache.get_entry(&id) {
                    Ok(entry) => entry,
                    Err(_) => return None, // TODO: change this maybe
                };
                Some(entry)
            }
        }))
    }

    fn recursive_remove(&mut self, dir_id: DirectoryEntryId) -> SysResult<()> {
        let children: Vec<DirectoryEntryId> = self
            .dcache
            .children(dir_id)?
            .map(|entry| entry.id)
            .try_collect()?;

        for child in children {
            let (_inode_id, is_dir) = {
                let entry = self
                    .dcache
                    .get_entry(&child)
                    .expect("There should be a child here");

                (entry.inode_id, entry.is_directory())
            };

            if is_dir {
                self.recursive_remove(child)?;
            }

            self.dcache.remove_entry(child)?;
            // self.inodes.remove(&inode_id);
        }
        let entry = self.dcache.get_entry(&dir_id)?;
        // let inode_id = entry.inode_id;
        self.dcache.remove_entry(dir_id)?;
        // self.inodes.remove(&inode_id);
        Ok(())
    }

    fn symlink(
        &mut self,
        parent: DirectoryEntryId,
        link_name: Filename,
        path: Path,
    ) -> SysResult<DirectoryEntryId> {
        let driver = Box::try_new(DefaultDriver)?;
        let filesystem = Arc::try_new(DeadMutex::new(DeadFileSystem))?;

        let inode_id: InodeId = self.gen();
        let inode_id = self.new_inode_id(inode_id.inode_number);

        let mut direntry = DirectoryEntryBuilder::new();
        direntry
            .set_symlink(path)
            .set_filename(link_name)
            .set_inode_id(inode_id);
        let direntry = direntry.build();

        let vfs_inode_data = *VfsInodeData::default()
            .set_id(inode_id)
            .set_alltime(unsafe { CURRENT_UNIX_TIME.load(Ordering::Relaxed) } as time_t)
            .set_link_number(1)
            .set_access_mode(
                FileType::SYMBOLIC_LINK | FileType::S_IRWXO | FileType::S_IRWXG | FileType::S_IRWXU,
            )
            .set_uid(0)
            .set_gid(0);

        let inode = Inode(
            VfsInode::new(filesystem, driver, vfs_inode_data),
            Box::try_new(|_inode_id| Ok(Box::try_new(DefaultDriver)? as Box<dyn Driver>))?,
        );

        let dir_id = self.dcache.add_entry(Some(parent), direntry)?;
        self.inodes.try_insert(inode_id, inode)?; // TODO: cleanup direntry in failure condition
        Ok(dir_id)
    }
}

impl FileSystem for ProcFs {
    fn is_dynamic(&self) -> bool {
        true
    }

    fn root(&self) -> SysResult<(DirectoryEntry, VfsInodeData, Box<dyn Driver>)> {
        let (root_dir_id, root_inode_id) = self.root_ids();

        let direntry = self
            .dcache
            .get_entry(&root_dir_id)
            .expect("There should be a root direntry for procfs");
        let inode = self
            .inodes
            .get(&root_inode_id)
            .expect("There should be a root inode for procfs");

        let mut new_direntry = direntry.try_clone()?;

        new_direntry.get_directory_mut().unwrap().clear_entries();
        Ok((new_direntry, inode.inode_data, Box::try_new(DefaultDriver)?))
    }

    fn lookup_directory(
        &mut self,
        inode_nbr: u32,
    ) -> SysResult<Vec<(DirectoryEntry, VfsInodeData, Box<dyn Driver>)>> {
        let inode_id = self.new_inode_id(inode_nbr);

        let inode = self.inodes.get_mut(&inode_id).ok_or(Errno::ENOENT)?;

        if !inode.is_directory() {
            return Err(Errno::ENOTDIR);
        }

        // That's very dummy but hey, fuck this design.
        let (root_dir_id, _root_inode_id) = self.root_ids();

        // Remove pid directories that points to PID that no longer exists.
        let pid_directories_to_remove: Vec<(DirectoryEntryId, Pid)> = self
            .pid_directories
            .iter()
            .filter_map(|(id, pid)| {
                SCHEDULER.force_unlock();
                if SCHEDULER.lock().get_thread_group(*pid).is_none() {
                    Some((*id, *pid))
                } else {
                    None
                }
            })
            .try_collect()?;
        for (pid_directory, pid) in pid_directories_to_remove {
            self.recursive_remove(pid_directory)?;
            self.pid_directories.remove(&(pid_directory, pid));
            // if let Some((fd_dir_id, _)) = self.get_fd_directory_entry(pid) {
            //     self.fd_directories.remove(&(fd_dir_id, pid));
            // }
        }

        // let self_filename = Filename::from_str_unwrap("self");

        // let self_dir_id = self
        //     .dcache
        //     .children(root_dir_id)?
        //     .find(|entry| entry.filename == self_filename)
        //     .map(|entry| entry.id);
        // if self_dir_id.is_some() {
        //     // refactor this, this is horrible.
        //     self.dcache
        //         .remove_entry(self_dir_id.unwrap())
        //         .expect("Failed to removed the old /proc/self symlink");
        // }

        // log::info!("Trying to lock scheduler");
        let direntry_id = self
            .dcache
            .iter()
            .find(|dir| dir.inode_id == inode_id)
            .expect("No corresponding directory for Inode")
            .id;

        if direntry_id == root_dir_id {
            self.fill_root_dir()?;
        } else if self.is_pid_directory(direntry_id) {
            // self.fill_root_dir()?;
            let (_, pid) = self.get_pid_directory_entry(direntry_id).unwrap(); // TODO: change this to expect
            self.fill_pid_directory(pid)?;
        } else if self.is_fd_directory(direntry_id) {
            // self.fill_root_dir()?;
            let (_, pid) = self.get_fd_directory_entry(direntry_id).unwrap(); // TODO: change this to expect
                                                                              // self.fill_pid_directory(pid)?;
            self.fill_fd_directory(pid)?;
        }

        let direntry = self
            .dcache
            .get_entry(&direntry_id)
            .expect("looked up direntry is supposed to exists");
        let inodes = &mut self.inodes;
        let dcache = &self.dcache;

        Ok(direntry
            .get_directory()
            .expect("Direntry was not a directory.")
            .entries()
            .filter_map(|direntry_id| {
                let direntry = dcache.get_entry(direntry_id).expect("WTF");
                let inode = inodes.get_mut(&direntry.inode_id);
                if let (ent, Some(inode)) = (direntry, inode) {
                    let inode_id = inode.id;
                    let mut entry = ent.try_clone().ok()?;

                    if entry.is_directory() {
                        // Cleanup the incompatible-with-vfs directoryEntryIds in the direntry.
                        entry.get_directory_mut().unwrap().clear_entries();
                    }
                    Some((entry, inode.inode_data, inode.1(inode_id).ok()?))
                } else {
                    None
                }
            })
            .try_collect()?)
    }

    fn remove_inode(&mut self, inode_nbr: u32) -> SysResult<()> {
        let inode_id = self.new_inode_id(inode_nbr);

        self.inodes.remove(&inode_id).ok_or(Errno::ENOENT)?;
        Ok(())
    }

    fn unlink(
        &mut self,
        _dir_inode_nbr: u32,
        _name: &str,
        free_inode_data: bool,
        inode_nbr: u32,
    ) -> SysResult<()> {
        let inode_id = self.new_inode_id(inode_nbr);
        let direntry_id = self
            .dcache
            .iter()
            .find(|entry| entry.inode_id == inode_id)
            .ok_or(Errno::ENOENT)?
            .id;

        // eprintln!(
        //     "Unlinking {:?}, freeing_inode: {}",
        //     inode_id, free_inode_data
        // );

        let inode = self.inodes.get_mut(&inode_id).ok_or(Errno::ENOENT)?;

        // In our use case, we just wanna remove the orphan inodes,
        // TODO: see how to handle normal unlinks.
        if inode.link_number != 0 {
            inode.link_number -= 1;
        }

        if self.is_pid_directory(direntry_id) {
            let (_, pid) = self.get_pid_directory_entry(direntry_id).unwrap(); // TODO: change this to expect
            assert!(self.pid_directories.remove(&(direntry_id, pid)));
        } else if self.is_fd_directory(direntry_id) {
            // self.fill_root_dir()?;
            let (_, pid) = self.get_fd_directory_entry(direntry_id).unwrap(); // TODO: change this to expect
                                                                              // self.fill_pid_directory(pid)?;
            assert!(self.fd_directories.remove(&(direntry_id, pid)));
        }
        self.dcache.remove_entry(direntry_id)?;

        if free_inode_data {
            self.inodes.remove(&inode_id).ok_or(Errno::ENOENT)?;
        }
        Ok(())
    }

    fn statfs(&self, buf: &mut statfs) -> SysResult<()> {
        Ok(*buf = statfs {
            f_type: PROC_SUPER_MAGIC,
            f_bsize: PAGE_SIZE,
            f_blocks: 0,
            f_bfree: 0,
            f_bavail: 0,
            f_files: 0,
            f_ffree: 0,
            f_fsid: self.fs_id.0 as u32,
            f_namelen: NAME_MAX - 1,
            f_frsize: 0,
            f_flags: 0,
        })
    }
}
