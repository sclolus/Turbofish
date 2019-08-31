use super::scheduler::SCHEDULER;
use super::Fd;
use super::SysResult;
use core::convert::TryFrom;
use libc_binding::{off_t, Errno, Whence};

/// The lseek() function shall set the file offset for the open file
/// description associated with the file descriptor fildes, as
/// follows:
///     If whence is SEEK_SET, the file offset shall be set to offset
///     bytes.
///     If whence is SEEK_CUR, the file offset shall be set to its
///     current location plus offset.
///     If whence is SEEK_END, the file offset shall be set to the
///     size of the file plus offset.
/// The symbolic constants SEEK_SET, SEEK_CUR, and SEEK_END are
/// defined in <unistd.h>.
/// The behavior of lseek() on devices which are incapable of seeking
/// is implementation-defined. The value of the file offset associated
/// with such a device is undefined.
/// The lseek() function shall allow the file offset to be set beyond
/// the end of the existing data in the file. If data is later written
/// at this point, subsequent reads of data in the gap shall return
/// bytes with the value 0 until data is actually written into the
/// gap.
///
/// The lseek() function shall not, by itself, extend the size of a
/// file.
/// Upon successful completion, the resulting offset, as measured in
/// bytes from the beginning of the file, shall be
/// returned. Otherwise, -1 shall be returned, errno shall be set to
/// indicate the error, and the file offset shall remain unchanged.
///ORS
/// [EBADF]
///     The fildes argument is not an open file descriptor.
/// [EINVAL]
///     The whence argument is not a proper value, or the resulting
///     file offset would be negative for a regular file, block
///     special file, or directory.
/// [EOVERFLOW]
///     The resulting file offset would be a value which cannot be
///     represented correctly in an object of type off_t.
/// [ESPIPE]
///     The fildes argument is associated with a pipe, FIFO, or
///     socket.
pub fn sys_lseek(ret: *mut off_t, fildes: Fd, offset: off_t, whence: u32) -> SysResult<u32> {
    // lseek in turbofish takes its return value as a pointer as it is a 64 bit value
    unpreemptible_context!({
        let ret = {
            let scheduler = SCHEDULER.lock();
            let v = scheduler
                .current_thread()
                .unwrap_process()
                .get_virtual_allocator();

            if ret.is_null() {
                return Err(Errno::EINVAL);
            } else {
                v.make_checked_ref_mut(ret)?
            }
        };
        *ret = match lseek(fildes, offset, whence) {
            Ok(return_value) => return_value as off_t,
            Err(errno) => (-(errno as off_t)) as off_t,
        };
        Ok(0)
    })
}

fn lseek(fildes: Fd, offset: off_t, whence: u32) -> SysResult<off_t> {
    // dbg!(offset);
    let whence = Whence::try_from(whence)?;
    // dbg!(whence);
    let mut scheduler = SCHEDULER.lock();

    let fd_interface = &mut scheduler
        .current_thread_group_running_mut()
        .file_descriptor_interface;

    let file_operation = &mut fd_interface.get_file_operation(fildes)?;
    file_operation.lseek(offset, whence)
}
