//! this module contains methods of the Ext2 which constitute the posix interface
#![allow(unused_variables)]
use super::DirectoryEntryType;
use super::{DirectoryEntry, Inode};
use crate::tools::IoResult;
use crate::Ext2Filesystem;
use alloc::vec::Vec;
use core::cmp::min;
use fallible_collections::TryCollect;
use libc_binding::{gid_t, uid_t, Errno, FileType, OpenFlags};

impl Ext2Filesystem {
    /// The access() function shall check the file named by the
    /// pathname pointed to by the path argument for accessibility
    /// according to the bit pattern contained in amode
    pub fn access(&mut self, path: &str, amode: i32) -> IoResult<()> {
        //TODO: check rights
        let inode = self.find_inode(path)?;
        Ok(())
    }

    /// The chown() function shall change the user and group ownership
    /// of a file.
    pub fn chown(&mut self, inode_nbr: u32, owner: uid_t, group: gid_t) -> IoResult<()> {
        unimplemented!();
    }

    // /// The lchown() function shall be equivalent to chown(), except
    // /// in the case where the named file is a symbolic link. In this
    // /// case, lchown() shall change the ownership of the symbolic link
    // pub fn lchown(&mut self, inode_nbr: u32, owner: uid_t, group: gid_t) -> IoResult<()> {
    //     unimplemented!();
    // }

    /// The chmod() function shall change S_ISUID, S_ISGID, [XSI]
    /// [Option Start] S_ISVTX, [Option End] and the file permission
    /// bits of the file
    pub fn chmod(&mut self, inode_nbr: u32, mode: FileType) -> IoResult<()> {
        // let (mut inode, inode_addr) = self.get_inode(inode_nbr)?;
        unimplemented!();
    }

    /// The rename() function shall change the name of a file
    pub fn rename(&mut self, old: &str, new: &str) -> IoResult<()> {
        unimplemented!();
    }

    /// The Truncate() Function Shall cause the regular file named by
    /// path to have a size which shall be equal to length bytes.
    pub fn truncate(&mut self, path: &str, length: u64) -> IoResult<()> {
        let (mut inode, inode_addr) = self.find_inode(path)?;
        if !inode.is_a_regular_file() {
            return Err(Errno::EISDIR);
        }
        self.truncate_inode((&mut inode, inode_addr), length)
    }

    pub fn create(
        &mut self,
        filename: &str,
        parent_inode_nbr: u32,
        flags: OpenFlags,
        timestamp: u32,
        mut mode: FileType,
    ) -> IoResult<(DirectoryEntry, Inode)> {
        mode |= FileType::REGULAR_FILE;
        let direntry_type = DirectoryEntryType::RegularFile;
        let inode_nbr = self.alloc_inode().ok_or(Errno::ENOSPC)?;
        let (_, inode_addr) = self.get_inode(inode_nbr)?;
        let mut inode = Inode::new(mode);

        inode.last_access_time = timestamp;
        inode.creation_time = timestamp;
        inode.last_modification_time = timestamp;

        self.disk.write_struct(inode_addr, &inode)?;

        let mut new_entry = DirectoryEntry::new(filename, direntry_type, inode_nbr)?;
        self.push_entry(parent_inode_nbr, &mut new_entry)?;
        Ok((new_entry, inode))
    }

    /// The unlink() function shall remove a link to a file.
    pub fn unlink(&mut self, parent_inode_nbr: u32, filename: &str) -> IoResult<()> {
        let entry = self.find_entry_in_inode(parent_inode_nbr, filename)?;
        self.unlink_inode(entry.0.get_inode())?;
        self.delete_entry(parent_inode_nbr, entry.1).unwrap();
        Ok(())
    }

    // /// The open() function shall establish the connection between a
    // /// file and a file descriptor.
    // pub fn open(&mut self, path: &str, flags: OpenFlags, mode: mode_t) -> IoResult<File> {
    //     let mut inode_nbr = 2;
    //     let mut iter_path = path.split('/').filter(|x| x != &"").peekable();
    //     while let Some(p) = iter_path.next() {
    //         let entry = self
    //             .iter_entries(inode_nbr)?
    //             .find(|(x, _)| unsafe { x.get_filename() } == p)
    //             .ok_or(Errno::ENOENT);
    //         // dbg!(entry?.0.get_filename());
    //         if entry.is_err() && iter_path.peek().is_none() && flags.contains(OpenFlags::O_CREAT) {
    //             inode_nbr = self.create_file(p, inode_nbr, flags)?;
    //         } else {
    //             inode_nbr = entry?.0.get_inode();
    //         }
    //     }
    //     Ok(File {
    //         inode_nbr,
    //         curr_offset: 0,
    //     })
    // }
    /// create a directory entry and an inode on the Directory inode:
    /// `parent_inode_nbr`, return the new inode nbr
    pub fn create_dir(
        &mut self,
        parent_inode_nbr: u32,
        filename: &str,
        mode: FileType,
    ) -> IoResult<(DirectoryEntry, Inode)> {
        //TODO: use mode
        let inode_nbr = self.alloc_inode().ok_or(Errno::ENOSPC)?;
        let (_, inode_addr) = self.get_inode(inode_nbr)?;
        let mut inode = Inode::new(mode | FileType::DIRECTORY);
        //TODO: check that
        inode.nbr_hard_links = 2;

        self.disk.write_struct(inode_addr, &inode)?;
        let mut new_entry =
            DirectoryEntry::new(filename, DirectoryEntryType::Directory, inode_nbr)?;
        self.push_entry(parent_inode_nbr, &mut new_entry)?;

        let mut point = DirectoryEntry::new(".", DirectoryEntryType::Directory, inode_nbr)?;
        let mut point_point =
            DirectoryEntry::new("..", DirectoryEntryType::Directory, parent_inode_nbr)?;
        self.push_entry(inode_nbr, &mut point)?;
        self.push_entry(inode_nbr, &mut point_point)?;
        Ok((new_entry, inode))
    }

    /// The rmdir() function shall remove the directory pointed by
    /// filename in the parent directory corresponding to
    /// parent_inode_nbr
    /// # Warining: the caller must assure that the directory is empty
    pub fn rmdir(&mut self, parent_inode_nbr: u32, filename: &str) -> IoResult<()> {
        let entry = self.find_entry_in_inode(parent_inode_nbr, filename)?;
        let inode_nbr = entry.0.get_inode();
        let (mut inode, inode_addr) = self.get_inode(inode_nbr)?;
        debug_assert!(inode.is_a_directory());
        self.free_inode((&mut inode, inode_addr), inode_nbr)?;
        self.delete_entry(parent_inode_nbr, entry.1)?;
        Ok(())
    }

    /// for write syscall
    pub fn write(&mut self, inode_nbr: u32, file_offset: &mut u64, buf: &[u8]) -> IoResult<u64> {
        let (mut inode, inode_addr) = self.get_inode(inode_nbr)?;
        let file_curr_offset_start = *file_offset;
        if *file_offset > inode.get_size() {
            panic!("file_offset > inode.get_size()");
        }
        if buf.len() == 0 {
            return Ok(0);
        }
        let data_address = self.inode_data_alloc((&mut inode, inode_addr), *file_offset)?;
        let offset = min(
            self.block_size as u64 - *file_offset % self.block_size as u64,
            buf.len() as u64,
        );
        let data_write = self
            .disk
            .write_buffer(data_address, &buf[0..offset as usize])?;
        *file_offset += data_write as u64;
        if inode.get_size() < *file_offset {
            inode.update_size(*file_offset, self.block_size);
            self.disk.write_struct(inode_addr, &inode)?;
        }
        if data_write < offset {
            return Ok(*file_offset - file_curr_offset_start);
        }

        for chunk in buf[offset as usize..].chunks(self.block_size as usize) {
            let data_address = self.inode_data_alloc((&mut inode, inode_addr), *file_offset)?;
            let data_write = self.disk.write_buffer(data_address, &chunk)?;
            *file_offset += data_write as u64;
            if inode.get_size() < *file_offset {
                inode.update_size(*file_offset, self.block_size);
                self.disk.write_struct(inode_addr, &inode)?;
            }
            if data_write < chunk.len() as u64 {
                return Ok(*file_offset - file_curr_offset_start);
            }
        }
        Ok(*file_offset - file_curr_offset_start)
    }

    /// return all the (directory, inode) conainted in inode_nbr
    pub fn lookup_directory(&mut self, inode_nbr: u32) -> IoResult<Vec<(DirectoryEntry, Inode)>> {
        //TODO: fallible
        let entries: Vec<DirectoryEntry> =
            self.iter_entries(inode_nbr)?.map(|(dir, _)| dir).collect();
        Ok(entries
            .into_iter()
            .filter_map(|dir| match self.get_inode(dir.get_inode()) {
                Ok((inode, _)) => Some((dir, inode)),
                Err(_e) => None,
            })
            .try_collect()?)
    }

    /// return the root inode of the ext2
    pub fn root_inode(&mut self) -> IoResult<Inode> {
        Ok(self.get_inode(2).expect("no inode 2, wtf").0)
    }

    pub fn read_inode(&mut self, inode_number: u32) -> IoResult<Inode> {
        Ok(self.get_inode(inode_number)?.0)
    }

    /// for read syscall
    pub fn read(&mut self, inode_nbr: u32, file_offset: &mut u64, buf: &mut [u8]) -> IoResult<u64> {
        let (mut inode, inode_addr) = self.get_inode(inode_nbr)?;
        let file_curr_offset_start = *file_offset;
        if *file_offset > inode.get_size() {
            panic!("file_offset > inode.get_size()");
        }
        if *file_offset == inode.get_size() {
            return Ok(0);
        }

        let data_address = self
            .inode_data((&mut inode, inode_addr), *file_offset)
            .unwrap();
        let offset = min(
            inode.get_size() - *file_offset,
            min(
                self.block_size as u64 - *file_offset % self.block_size as u64,
                buf.len() as u64,
            ),
        );
        let data_read = self
            .disk
            .read_buffer(data_address, &mut buf[0..offset as usize])?;
        *file_offset += data_read as u64;
        if data_read < offset {
            return Ok(*file_offset - file_curr_offset_start);
        }

        for chunk in buf[offset as usize..].chunks_mut(self.block_size as usize) {
            let data_address = self
                .inode_data((&mut inode, inode_addr), *file_offset)
                .unwrap();
            let offset = min((inode.get_size() - *file_offset) as usize, chunk.len());
            let data_read = self.disk.read_buffer(data_address, &mut chunk[0..offset])?;
            *file_offset += data_read as u64;
            if data_read < chunk.len() as u64 {
                return Ok(*file_offset - file_curr_offset_start);
            }
        }
        Ok(*file_offset - file_curr_offset_start)
    }

    /// return the block size of ext2
    pub fn get_block_size(&self) -> u32 {
        self.block_size
    }
}
