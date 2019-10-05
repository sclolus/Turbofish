use super::SysResult;

use super::scheduler::SCHEDULER;

use libc_binding::{c_char, Errno, HOST_NAME_MAX};

use super::HOSTNAME;

fn sethostname(name: &[u8]) -> SysResult<u32> {
    let mut hostname = HOSTNAME.lock();

    if name.len() + 1 > HOST_NAME_MAX as usize {
        return Err(Errno::EINVAL);
    }

    &hostname[..name.len()].copy_from_slice(name);
    hostname[name.len()] = '\0' as u8;
    Ok(0)
}

pub fn sys_sethostname(name: *const c_char, namelen: usize) -> SysResult<u32> {
    unpreemptible_context!({
        let name: &[u8] = {
            let mut scheduler = SCHEDULER.lock();

            let v = scheduler
                .current_thread_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            v.make_checked_slice(name as *const u8, namelen)?
        };
        sethostname(name)
    })
}
