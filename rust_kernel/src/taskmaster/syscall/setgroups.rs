//! sys_setgroups()

use super::scheduler::SCHEDULER;
use super::SysResult;
use alloc::vec::Vec;
use fallible_collections::FallibleVec;
use libc_binding::gid_t;
use libc_binding::Errno;

/// setgroups() sets the supplementary group IDs for the calling
/// process.  Appropriate privileges are required (see the description
/// of the EPERM error, below).  The size argument specifies the
/// number of supplementary group IDs in the buffer pointed to by
/// list.
pub fn sys_setgroups(gidsetsize: i32, grouplist: *const gid_t) -> SysResult<u32> {
    unpreemptible_context!({
        if gidsetsize < 0 {
            return Err(Errno::EINVAL);
        }
        let mut scheduler = SCHEDULER.lock();
        let grouplist = {
            let v = scheduler
                .current_thread()
                .unwrap_process()
                .get_virtual_allocator();

            // Check if pointer exists in user virtual address space
            v.make_checked_slice(grouplist, gidsetsize as usize)?
        };
        let thread_group = scheduler.current_thread_group_mut();
        let cred = &mut thread_group.credentials;
        if cred.uid != 0 {
            return Err(Errno::EPERM);
        }
        let mut new_groups = Vec::new();
        new_groups.try_extend_from_slice(grouplist)?;
        cred.groups = new_groups;
        Ok(0)
    })
}
