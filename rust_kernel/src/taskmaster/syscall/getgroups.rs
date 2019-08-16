//! sys_getgroups()

use super::scheduler::SCHEDULER;
use super::SysResult;
use core::cmp::min;
use errno::Errno;
use libc_binding::gid_t;

/// The getgroups() function shall fill in the array grouplist with the
/// current supplementary group IDs of the calling process. It is
/// implementation-defined whether getgroups() also returns the
/// effective group ID in the grouplist array.
///
/// The gidsetsize argument specifies the number of elements in the
/// array grouplist. The actual number of group IDs stored in the array
/// shall be returned. The values of array entries with indices greater
/// than or equal to the value returned are undefined.
///
/// If gidsetsize is 0, getgroups() shall return the number of group
/// IDs that it would otherwise return without modifying the array
/// pointed to by grouplist.
///
/// If the effective group ID of the process is returned with the
/// supplementary group IDs, the value returned shall always be greater
/// than or equal to one and less than or equal to the value of
/// {NGROUPS_MAX}+1.

pub fn sys_getgroups(gidsetsize: i32, grouplist: *mut gid_t) -> SysResult<u32> {
    unpreemptible_context!({
        if gidsetsize < 0 {
            return Err(Errno::Einval);
        }
        let scheduler = SCHEDULER.lock();
        let grouplist_slice = {
            let v = scheduler
                .current_task()
                .unwrap_process()
                .get_virtual_allocator();

            // Check if pointer exists in user virtual address space
            v.make_checked_mut_slice(grouplist, gidsetsize as usize)?
        };
        let thread_group = scheduler.current_thread_group();
        let cred = &thread_group.credentials;
        for (grouplist_gid, gid) in grouplist_slice.iter_mut().zip(cred.groups.iter()) {
            *grouplist_gid = *gid;
        }
        if gidsetsize == 0 {
            return Ok(cred.groups.len() as u32);
        }
        Ok(min(cred.groups.len() as u32, gidsetsize as u32))
    })
}
