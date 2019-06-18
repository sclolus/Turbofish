//! sys_execve implementation

use super::SysResult;

use super::scheduler::SCHEDULER;

use alloc::format;
use alloc::vec;
use alloc::vec::Vec;
use errno::Errno;
use ext2::syscall::OpenFlags;

use crate::drivers::storage::ext2::EXT2;
use crate::ffi::{c_char, c_str, strlen};

use crate::taskmaster::process::{Process, UserProcess};
use crate::taskmaster::TaskOrigin;

/// Execute a program
pub fn sys_execve(filename: *const u8, _argv: u32, _envp: u32) -> SysResult<u32> {
    unpreemptible_context!({
        let mut scheduler = SCHEDULER.lock();

        let v = &mut scheduler.current_task_mut().unwrap_process_mut().virtual_allocator;

        // TODO: check with len
        v.check_user_ptr::<u8>(filename)?;

        // TODO: Unsafe strlen here. the check must be done before
        let len = unsafe { strlen(filename as *const c_char) };

        let filename = unsafe { core::str::from_utf8_unchecked(core::slice::from_raw_parts(filename, len)) };
        let pathname = format!("bin/{}", filename);

        println!("{:?}", pathname);

        let ext2 = unsafe { EXT2.as_mut().ok_or("ext2 not init").map_err(|_| Errno::Enodev)? };
        let mut file = ext2.open(&pathname, OpenFlags::O_RDONLY, 0).map_err(|_| Errno::Enodev)?;

        println!("{:?}", file);

        if let Ok(inode) = ext2.get_inode(file.inode_nbr) {
            println!("{:?}", inode);
            let mut v: Vec<u8> = vec![0; inode.0.low_size as usize];
            let len = ext2.read(&mut file, v.as_mut()).map_err(|_| Errno::Enoent)?;
            println!("{} bytes readen", len);

            unsafe {
                let p = UserProcess::new(TaskOrigin::Elf(v.as_mut())).map_err(|_| Errno::Enodev)?;
                scheduler.add_user_process(None, p).unwrap();
            }
        }
    });
    Ok(0)
}
