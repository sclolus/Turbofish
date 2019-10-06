use super::scheduler::SCHEDULER;
use super::Fd;
use super::SysResult;
use core::mem::transmute;
use libc_binding::{Errno, FcntlCmd};

/// The fcntl() function shall perform the operations described below
/// on open files. The fildes argument is a file descriptor.
///
/// The available values for cmd are defined in <fcntl.h> and are as follows:
///
/// F_DUPFD_CLOEXEC Like F_DUPFD, but the FD_CLOEXEC flag associated
///     with the new file descriptor shall be set.
/// F_GETFD Get the file descriptor flags defined in <fcntl.h> that
///     are associated with the file descriptor fildes. File
///     descriptor flags are associated with a single file descriptor
///     and do not affect other file descriptors that refer to the
///     same file.
/// F_GETFL Get the file status flags and file access modes, defined
///     in <fcntl.h>, for the file description associated with
///     fildes. The file access modes can be extracted from the return
///     value using the mask O_ACCMODE, which is defined in
///     <fcntl.h>. File status flags and file access modes are
///     associated with the file description and do not affect other
///     file descriptors that refer to the same file with different
///     open file descriptions. The flags returned may include
///     non-standard file status flags which the application did not
///     set, provided that these additional flags do not alter the
///     behavior of a conforming application.
/// F_SETFL Set the file status flags, defined in <fcntl.h>, for the
///     file description associated with fildes from the corresponding
///     bits in the third argument, arg, taken as type int. Bits
///     corresponding to the file access mode and the file creation
///     flags, as defined in <fcntl.h>, that are set in arg shall be
///     ignored. If any bits in arg other than those mentioned here
///     are changed by the application, the result is unspecified. If
///     fildes does not support non-blocking operations, it is
///     unspecified whether the O_NONBLOCK flag will be ignored.
/// F_GETOWN
///     If fildes refers to a socket, get the process ID or process
///     group ID specified to receive SIGURG signals when out-of-band
///     data is available. Positive values shall indicate a process
///     ID; negative values, other than -1, shall indicate a process
///     group ID; the value zero shall indicate that no SIGURG signals
///     are to be sent. If fildes does not refer to a socket, the
///     results are unspecified.
/// F_SETOWN
///     If fildes refers to a socket, set the process ID or process
///     group ID specified to receive SIGURG signals when out-of-band
///     data is available, using the value of the third argument, arg,
///     taken as type int. Positive values shall indicate a process
///     ID; negative values, other than -1, shall indicate a process
///     group ID; the value zero shall indicate that no SIGURG signals
///     are to be sent. Each time a SIGURG signal is sent to the
///     specified process or process group, permission checks
///     equivalent to those performed by kill() shall be performed, as
///     if kill() were called by a process with the same real user ID,
///     effective user ID, and privileges that the process calling
///     fcntl() has at the time of the call; if the kill() call would
///     fail, no signal shall be sent. These permission checks may
///     also be performed by the fcntl() call. If the process
///     specified by arg later terminates, or the process group
///     specified by arg later becomes empty, while still being
///     specified to receive SIGURG signals when out-of-band data is
///     available from fildes, then no signals shall be sent to any
///     subsequently created process that has the same process ID or
///     process group ID, regardless of permission; it is unspecified
///     whether this is achieved by the equivalent of a fcntl(fildes,
///     F_SETOWN, 0) call at the time the process terminates or is
///     waited for or the process group becomes empty, or by other
///     means. If fildes does not refer to a socket, the results are
///     unspecified.
///
/// The following values for cmd are available for advisory record
/// locking. Record locking shall be supported for regular files, and
/// may be supported for other files.
///
/// F_GETLK
///     Get any lock which blocks the lock description pointed to by
///     the third argument, arg, taken as a pointer to type struct
///     flock, defined in <fcntl.h>. The information retrieved shall
///     overwrite the information passed to fcntl() in the structure
///     flock. If no lock is found that would prevent this lock from
///     being created, then the structure shall be left unchanged
///     except for the lock type which shall be set to F_UNLCK.
/// F_SETLK
///     Set or clear a file segment lock according to the lock
///     description pointed to by the third argument, arg, taken as a
///     pointer to type struct flock, defined in <fcntl.h>. F_SETLK
///     can establish shared (or read) locks (F_RDLCK) or exclusive
///     (or write) locks (F_WRLCK), as well as to remove either type
///     of lock (F_UNLCK). F_RDLCK, F_WRLCK, and F_UNLCK are defined
///     in <fcntl.h>. If a shared or exclusive lock cannot be set,
///     fcntl() shall return immediately with a return value of -1.
/// F_SETLKW
///     This command shall be equivalent to F_SETLK except that if a
///     shared or exclusive lock is blocked by other locks, the thread
///     shall wait until the request can be satisfied. If a signal
///     that is to be caught is received while fcntl() is waiting for
///     a region, fcntl() shall be interrupted. Upon return from the
///     signal handler, fcntl() shall return -1 with errno set to
///     [EINTR], and the lock operation shall not be done.
///
/// Additional implementation-defined values for cmd may be defined in
/// <fcntl.h>. Their names shall start with F_.
///
/// When a shared lock is set on a segment of a file, other processes
/// shall be able to set shared locks on that segment or a portion of
/// it. A shared lock prevents any other process from setting an
/// exclusive lock on any portion of the protected area. A request for
/// a shared lock shall fail if the file descriptor was not opened
/// with read access.
///
/// An exclusive lock shall prevent any other process from setting a
/// shared lock or an exclusive lock on any portion of the protected
/// area. A request for an exclusive lock shall fail if the file
/// descriptor was not opened with write access.
///
/// The structure flock describes the type (l_type), starting offset
/// (l_whence), relative offset ( l_start), size (l_len), and process
/// ID (l_pid) of the segment of the file to be affected.
///
/// The value of l_whence is SEEK_SET, SEEK_CUR, or SEEK_END, to
/// indicate that the relative offset l_start bytes shall be measured
/// from the start of the file, current position, or end of the file,
/// respectively. The value of l_len is the number of consecutive
/// bytes to be locked. The value of l_len may be negative (where the
/// definition of off_t permits negative values of l_len). The l_pid
/// field is only used with F_GETLK to return the process ID of the
/// process holding a blocking lock. After a successful F_GETLK
/// request, when a blocking lock is found, the values returned in the
/// flock structure shall be as follows:
///
/// l_type
///     Type of blocking lock found.
/// l_whence
///     SEEK_SET.
/// l_start
///     Start of the blocking lock.
/// l_len
///     Length of the blocking lock.
/// l_pid
///     Process ID of the process that holds the blocking lock.
///
/// If the command is F_SETLKW and the process must wait for another
/// process to release a lock, then the range of bytes to be locked
/// shall be determined before the fcntl() function blocks. If the
/// file size or file descriptor seek offset change while fcntl() is
/// blocked, this shall not affect the range of bytes locked.
///
/// If l_len is positive, the area affected shall start at l_start and
/// end at l_start+ l_len-1. If l_len is negative, the area affected
/// shall start at l_start+ l_len and end at l_start-1. Locks may
/// start and extend beyond the current end of a file, but shall not
/// extend before the beginning of the file. A lock shall be set to
/// extend to the largest possible value of the file offset for that
/// file by setting l_len to 0. If such a lock also has l_start set to
/// 0 and l_whence is set to SEEK_SET, the whole file shall be locked.
///
/// There shall be at most one type of lock set for each byte in the
/// file. Before a successful return from an F_SETLK or an F_SETLKW
/// request when the calling process has previously existing locks on
/// bytes in the region specified by the request, the previous lock
/// type for each byte in the specified region shall be replaced by
/// the new lock type. As specified above under the descriptions of
/// shared locks and exclusive locks, an F_SETLK or an F_SETLKW
/// request (respectively) shall fail or block when another process
/// has existing locks on bytes in the specified region and the type
/// of any of those locks conflicts with the type specified in the
/// request.
///
/// All locks associated with a file for a given process shall be
/// removed when a file descriptor for that file is closed by that
/// process or the process holding that file descriptor
/// terminates. Locks are not inherited by a child process.
///
/// A potential for deadlock occurs if a process controlling a locked
/// region is put to sleep by attempting to lock the locked region of
/// another process. If the system detects that sleeping until a
/// locked region is unlocked would cause a deadlock, fcntl() shall
/// fail with an [EDEADLK] error.
///
/// An unlock (F_UNLCK) request in which l_len is non-zero and the
/// offset of the last byte of the requested segment is the maximum
/// value for an object of type off_t, when the process has an
/// existing lock in which l_len is 0 and which includes the last byte
/// of the requested segment, shall be treated as a request to unlock
/// from the start of the requested segment with an l_len equal to
/// 0. Otherwise, an unlock (F_UNLCK) request shall attempt to unlock
/// only the requested segment.
///
/// [SHM] [Option Start] When the file descriptor fildes refers to a
/// shared memory object, the behavior of fcntl() shall be the same as
/// for a regular file except the effect of the following values for
/// the argument cmd shall be unspecified: F_SETFL, F_GETLK, F_SETLK,
/// and F_SETLKW. [Option End]
///
/// [TYM] [Option Start] If fildes refers to a typed memory object,
/// the result of the fcntl() function is unspecified. [Option End]

pub fn sys_fcntl(fildes: Fd, cmd: u32, arg: Fd) -> SysResult<u32> {
    unpreemptible_context!({
        // dbg!(cmd);
        let cmd: FcntlCmd = unsafe { transmute(cmd) };
        // dbg!(fildes);
        // dbg!(cmd);
        // dbg!(arg);
        let mut scheduler = SCHEDULER.lock();

        match cmd {
            // F_DUPFD Return a new file descriptor which shall be
            //     allocated as described in File Descriptor
            //     Allocation, except that it shall be the lowest
            //     numbered available file descriptor greater than or
            //     equal to the third argument, arg, taken as an
            //     integer of type int. The new file descriptor shall
            //     refer to the same open file description as the
            //     original file descriptor, and shall share any
            //     locks. The FD_CLOEXEC flag associated with the new
            //     file descriptor shall be cleared to keep the file
            //     open across calls to one of the exec functions.
            FcntlCmd::F_DUPFD => scheduler
                .current_thread_group_running_mut()
                .file_descriptor_interface
                .dup(fildes, Some(arg)),
            //TODO:
            // F_SETFD Set the file descriptor flags defined in
            //     <fcntl.h>, that are associated with fildes, to the
            //     third argument, arg, taken as type int. If the
            //     FD_CLOEXEC flag in the third argument is 0, the
            //     file descriptor shall remain open across the exec
            //     functions; otherwise, the file descriptor shall be
            //     closed upon successful execution of one of the exec
            //     functions.
            FcntlCmd::F_SETFD => Ok(0),
            _ => Err(Errno::EINVAL),
        }
    })
}
