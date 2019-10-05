use super::SysResult;

use super::scheduler::SCHEDULER;

use libc_binding::{c_char, Errno, HOST_NAME_MAX};

use core::cmp::min;
use sync::DeadMutex;

lazy_static! {
    pub static ref HOSTNAME: DeadMutex<[u8; HOST_NAME_MAX as usize]> = {
        let mut buf = [0u8; HOST_NAME_MAX as usize];

        for (index, byte) in b"Turbofish\0".iter().enumerate() {
            buf[index] = *byte;
        }
        DeadMutex::new(buf)
    };
}

fn gethostname(name: &mut [u8]) -> SysResult<u32> {
    let hostname = HOSTNAME.lock();
    let null_byte_pos = hostname
        .iter()
        .position(|u| *u == '\0' as u8)
        .expect("There should be a null byte in HOSTNAME");
    let size = min(name.len(), null_byte_pos + 1);

    name[..size].copy_from_slice(&hostname[..size]);
    Ok(0)
}

pub fn sys_gethostname(name: *mut c_char, namelen: usize) -> SysResult<u32> {
    unpreemptible_context!({
        let name: &mut [u8] = {
            let mut scheduler = SCHEDULER.lock();

            let v = scheduler
                .current_thread_mut()
                .unwrap_process_mut()
                .get_virtual_allocator();

            v.make_checked_mut_slice(name as *mut u8, namelen)?
        };
        gethostname(name)
    })
}
