use crate::*;
use core::cmp::min;

impl Ext2Filesystem {
    /// Open a File
    pub fn open(&mut self, path: &str, flags: OpenFlags) -> IoResult<File> {
        let mut inode_nbr = 2;
        let mut iter_path = path.split('/').peekable();
        while let Some(p) = iter_path.next() {
            let entry = self
                .iter_entries(inode_nbr)?
                .find(|(x, _)| unsafe { x.get_filename() } == p)
                .ok_or(Errno::Enoent);
            // dbg!(entry?.0.get_filename());
            if entry.is_err() && iter_path.peek().is_none() && flags.contains(OpenFlags::CREAT) {
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

    /// for unlink syscall (see man unlink(2))
    pub fn unlink(&mut self, path: &str) -> IoResult<()> {
        let (parent_inode_nbr, entry) = self.find_path(path)?;
        self.unlink_inode(entry.0.get_inode())?;
        self.delete_entry(parent_inode_nbr, entry.1).unwrap();
        Ok(())
    }

    pub fn mkdir(&mut self, path: &str /*, mode: Mode*/) -> IoResult<()> {
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

    /// rmdir(2) deletes a directory, which must be empty.
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
        self.delete_inode(inode_nbr).unwrap();
        self.delete_entry(parent_inode_nbr, entry.1).unwrap();
        Ok(())
    }

    /// for read syscall
    pub fn read(&mut self, file: &mut File, buf: &mut [u8]) -> IoResult<u64> {
        // for i in 0..self.nbr_block_grp {
        //     dbg!(self.get_block_grp_descriptor(i));
        // }
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
            .read_buffer(data_address, &mut buf[0..offset as usize]);
        file.curr_offset += data_read as u64;
        if data_read < offset {
            return Ok(file.curr_offset - file_curr_offset_start);
        }

        for chunk in buf[offset as usize..].chunks_mut(self.block_size as usize) {
            let data_address = self
                .inode_data((&mut inode, inode_addr), file.curr_offset)
                .unwrap();
            let offset = min((inode.get_size() - file.curr_offset) as usize, chunk.len());
            let data_read = self.disk.read_buffer(data_address, &mut chunk[0..offset]);
            file.curr_offset += data_read as u64;
            if data_read < chunk.len() as u64 {
                return Ok(file.curr_offset - file_curr_offset_start);
            }
        }
        Ok(file.curr_offset - file_curr_offset_start)
    }

    /// for write syscall
    pub fn write(&mut self, file: &mut File, buf: &[u8]) -> IoResult<u64> {
        // for i in 0..self.nbr_block_grp {
        //     dbg!(self.get_block_grp_descriptor(i));
        // }
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
            .write_buffer(data_address, &buf[0..offset as usize]);
        file.curr_offset += data_write as u64;
        if inode.get_size() < file.curr_offset {
            inode.update_size(file.curr_offset, self.block_size);
            self.disk.write_struct(inode_addr, &inode);
        }
        if data_write < offset {
            return Ok(file.curr_offset - file_curr_offset_start);
        }

        for chunk in buf[offset as usize..].chunks(self.block_size as usize) {
            let data_address = self.inode_data_alloc((&mut inode, inode_addr), file.curr_offset)?;
            let data_write = self.disk.write_buffer(data_address, &chunk);
            file.curr_offset += data_write as u64;
            if inode.get_size() < file.curr_offset {
                inode.update_size(file.curr_offset, self.block_size);
                self.disk.write_struct(inode_addr, &inode);
            }
            if data_write < chunk.len() as u64 {
                return Ok(file.curr_offset - file_curr_offset_start);
            }
        }
        Ok(file.curr_offset - file_curr_offset_start)
    }
}
