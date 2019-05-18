//! this module contains methods of the Ext2 which constitute the posix interface
#![allow(unused_variables)]
use crate::tools::IoResult;
use crate::{Ext2Filesystem, File};
use core::cmp::min;
pub mod data;
pub use data::*;
pub use data::{Errno, OpenFlags};

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
    pub fn chown(&mut self, path: &str, owner: uid_t, group: gid_t) -> IoResult<()> {
        unimplemented!();
    }

    /// The lchown() function shall be equivalent to chown(), except
    /// in the case where the named file is a symbolic link. In this
    /// case, lchown() shall change the ownership of the symbolic link
    pub fn lchown(&mut self, path: &str, owner: uid_t, group: gid_t) -> IoResult<()> {
        unimplemented!();
    }

    /// The chmod() function shall change S_ISUID, S_ISGID, [XSI]
    /// [Option Start] S_ISVTX, [Option End] and the file permission
    /// bits of the file
    pub fn chmod(&mut self, path: &str, mode: mode_t) -> IoResult<()> {
        unimplemented!();
    }

    /// Should behave as 'return open(path, O_WRONLY|O_CREAT|O_TRUNC, mode);'
    pub fn creat(&mut self, path: &str, mode: mode_t) -> IoResult<File> {
        self.open(
            path,
            OpenFlags::O_WRONLY | OpenFlags::O_CREAT | OpenFlags::O_TRUNC,
            mode,
        )
    }

    /// The stat() function shall obtain information about the named
    /// file and write it to the area pointed to by the buf argument.
    pub fn stat(&mut self, path: &str, buf: &mut Stat) -> IoResult<()> {
        unimplemented!();
    }

    /// The rename() function shall change the name of a file
    pub fn rename(&mut self, old: &str, new: &str) -> IoResult<()> {
        unimplemented!();
    }

    /// The truncate() function shall cause the regular file named by
    /// path to have a size which shall be equal to length bytes.
    pub fn truncate(&mut self, path: &str, length: u64) -> IoResult<()> {
        let (mut inode, inode_addr) = self.find_inode(path)?;
        if !inode.is_a_regular_file() {
            return Err(Errno::Eisdir);
        }
        self.truncate_inode((&mut inode, inode_addr), length)
    }

    /// The open() function shall establish the connection between a
    /// file and a file descriptor.
    pub fn open(&mut self, path: &str, flags: OpenFlags, mode: mode_t) -> IoResult<File> {
        let mut inode_nbr = 2;
        let mut iter_path = path.split('/').peekable();
        while let Some(p) = iter_path.next() {
            let entry = self
                .iter_entries(inode_nbr)?
                .find(|(x, _)| unsafe { x.get_filename() } == p)
                .ok_or(Errno::Enoent);
            // dbg!(entry?.0.get_filename());
            if entry.is_err() && iter_path.peek().is_none() && flags.contains(OpenFlags::O_CREAT) {
                inode_nbr = self.create_file(p, inode_nbr, flags)?;
            } else {
                inode_nbr = entry?.0.get_inode();
            }
        }
        Ok(File {
            inode_nbr,
            curr_offset: 0,
        })
    }

    /// The unlink() function shall remove a link to a file.
    pub fn unlink(&mut self, path: &str) -> IoResult<()> {
        let (parent_inode_nbr, entry) = self.find_path(path)?;
        self.unlink_inode(entry.0.get_inode())?;
        self.delete_entry(parent_inode_nbr, entry.1).unwrap();
        Ok(())
    }

    /// The mkdir() function shall create a new directory with name
    /// path.
    pub fn mkdir(&mut self, path: &str, _mode: mode_t) -> IoResult<()> {
        let mut inode_nbr = 2;
        let mut iter_path = path.split('/').peekable();
        while let Some(p) = iter_path.next() {
            let entry = self
                .iter_entries(inode_nbr)?
                .find(|(x, _)| unsafe { x.get_filename() } == p)
                .ok_or(Errno::Enoent);
            // dbg!(entry?.0.get_filename());
            if entry.is_err() && iter_path.peek().is_none() {
                inode_nbr = self.create_dir(p, inode_nbr)?;
            } else {
                inode_nbr = entry?.0.get_inode();
            }
        }
        Ok(())
    }

    /// The rmdir() function shall remove a directory only if it is an
    /// empty directory.
    pub fn rmdir(&mut self, path: &str) -> IoResult<()> {
        let (parent_inode_nbr, entry) = self.find_path(path)?;
        let inode_nbr = entry.0.get_inode();
        let (inode, _inode_addr) = self.get_inode(inode_nbr)?;

        if !inode.is_a_directory() {
            return Err(Errno::Enotdir);
        }
        if self
            .iter_entries(inode_nbr)?
            .any(|(x, _)| unsafe { x.get_filename() != "." && x.get_filename() != ".." })
            || inode.nbr_hard_links > 2
        {
            return Err(Errno::Enotempty);
        }
        let (mut inode, inode_addr) = self.get_inode(inode_nbr)?;
        self.free_inode((&mut inode, inode_addr), inode_nbr)
            .unwrap();
        self.delete_entry(parent_inode_nbr, entry.1).unwrap();
        Ok(())
    }

    /// for read syscall
    pub fn read(&mut self, file: &mut File, buf: &mut [u8]) -> IoResult<u64> {
        let (mut inode, inode_addr) = self.get_inode(file.inode_nbr)?;
        let file_curr_offset_start = file.curr_offset;
        if file.curr_offset > inode.get_size() {
            return Err(Errno::Ebadf);
        }
        if file.curr_offset == inode.get_size() {
            return Ok(0);
        }

        let data_address = self
            .inode_data((&mut inode, inode_addr), file.curr_offset)
            .unwrap();
        let offset = min(
            inode.get_size() - file.curr_offset,
            min(
                self.block_size as u64 - file.curr_offset % self.block_size as u64,
                buf.len() as u64,
            ),
        );
        let data_read = self
            .disk
            .read_buffer(data_address, &mut buf[0..offset as usize])?;
        file.curr_offset += data_read as u64;
        if data_read < offset {
            return Ok(file.curr_offset - file_curr_offset_start);
        }

        for chunk in buf[offset as usize..].chunks_mut(self.block_size as usize) {
            let data_address = self
                .inode_data((&mut inode, inode_addr), file.curr_offset)
                .unwrap();
            let offset = min((inode.get_size() - file.curr_offset) as usize, chunk.len());
            let data_read = self.disk.read_buffer(data_address, &mut chunk[0..offset])?;
            file.curr_offset += data_read as u64;
            if data_read < chunk.len() as u64 {
                return Ok(file.curr_offset - file_curr_offset_start);
            }
        }
        Ok(file.curr_offset - file_curr_offset_start)
    }

    /// for write syscall
    pub fn write(&mut self, file: &mut File, buf: &[u8]) -> IoResult<u64> {
        let (mut inode, inode_addr) = self.get_inode(file.inode_nbr)?;
        let file_curr_offset_start = file.curr_offset;
        if file.curr_offset > inode.get_size() {
            return Err(Errno::Ebadf);
        }
        if buf.len() == 0 {
            return Ok(0);
        }
        let data_address = self.inode_data_alloc((&mut inode, inode_addr), file.curr_offset)?;
        let offset = min(
            self.block_size as u64 - file.curr_offset % self.block_size as u64,
            buf.len() as u64,
        );
        let data_write = self
            .disk
            .write_buffer(data_address, &buf[0..offset as usize])?;
        file.curr_offset += data_write as u64;
        if inode.get_size() < file.curr_offset {
            inode.update_size(file.curr_offset, self.block_size);
            self.disk.write_struct(inode_addr, &inode)?;
        }
        if data_write < offset {
            return Ok(file.curr_offset - file_curr_offset_start);
        }

        for chunk in buf[offset as usize..].chunks(self.block_size as usize) {
            let data_address = self.inode_data_alloc((&mut inode, inode_addr), file.curr_offset)?;
            let data_write = self.disk.write_buffer(data_address, &chunk)?;
            file.curr_offset += data_write as u64;
            if inode.get_size() < file.curr_offset {
                inode.update_size(file.curr_offset, self.block_size);
                self.disk.write_struct(inode_addr, &inode)?;
            }
            if data_write < chunk.len() as u64 {
                return Ok(file.curr_offset - file_curr_offset_start);
            }
        }
        Ok(file.curr_offset - file_curr_offset_start)
    }
}
