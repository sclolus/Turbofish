//! contain the standard errno definition
#![cfg_attr(not(test), no_std)]
#![feature(try_reserve)]
extern crate alloc;
use alloc::collections::CollectionAllocErr;

/// Standard error errno
#[repr(i8)]
#[derive(Debug, Copy, Clone, PartialEq)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub enum Errno {
    /// Operation not permitted.
    Eperm = 1,
    /// No such file or directory
    Enoent = 2,
    /// No such process.
    Esrch = 3,
    /// Interrupted system call.
    Eintr = 4,
    /// I/O error.
    Eio = 5,
    /// No such device or address.
    Enxio = 6,
    /// Argument list too long.
    E2Big = 7,
    /// No such file or directory/Exec Format Error.
    Enoexec = 8,
    /// Bad file descriptor.
    Ebadf = 9,
    /// No child processes.
    Echild = 10,
    /// Resource unavailable, try again (may be the same value as [EWOULDBLOCK]).
    Eagain = 11,
    /// Out of memory.
    Enomem = 12,
    /// Permission denied.
    Eacces = 13,
    /// Bad address.
    Efault = 14,
    /// Block device required
    Enotblk = 15,
    /// Device or resource busy.
    Ebusy = 16,
    /// File exists.
    Eexist = 17,
    /// Cross-device link.
    Exdev = 18,
    // TODO SORT NEXT ERRORS BY NUMBER
    /// Address in use.
    Eaddrinuse,
    /// Address not available.
    Eaddrnotavail,
    /// Address family not supported.
    Eafnosupport,
    /// Connection already in progress.
    Ealready,
    /// Bad message.
    Ebadmsg,
    /// Operation canceled.
    Ecanceled,
    /// Connection aborted.
    Econnaborted,
    /// Connection refused.
    Econnrefused,
    /// Connection reset.
    Econnreset,
    /// Resource deadlock would occur.
    Edeadlk,
    /// Destination address required.
    Edestaddrreq,
    /// Mathematics argument out of domain of function.
    Edom,
    /// Reserved.
    Edquot,
    /// File too large.
    Efbig,
    /// Host is unreachable.
    Ehostunreach,
    /// Identifier removed.
    Eidrm,
    /// Illegal byte sequence.
    Eilseq,
    /// Operation in progress.
    Einprogress,
    /// Invalid argument.
    Einval,
    /// Socket is connected.
    Eisconn,
    /// Is a directory.
    Eisdir,
    /// Too many levels of symbolic links.
    Eloop,
    /// File descriptor value too large.
    Emfile,
    /// Too many links.
    Emlink,
    /// Message too large.
    Emsgsize,
    /// Reserved.
    Emultihop,
    /// Filename too long.
    Enametoolong,
    /// Network is down.
    Enetdown,
    /// Connection aborted by network.
    Enetreset,
    /// Network unreachable.
    Enetunreach,
    /// Too many files open in system.
    Enfile,
    /// No buffer space available.
    Enobufs,
    /// [OB XSR] [Option Start] No message is available on the STREAM head read queue. [Option End]
    Enodata,
    /// No such device.
    Enodev,
    /// No locks available.
    Enolck,
    /// Reserved.
    Enolink,
    /// No message of the desired type.
    Enomsg,
    /// Protocol not available.
    Enoprotoopt,
    /// No space left on device.
    Enospc,
    /// [OB XSR] [Option Start] No STREAM resources. [Option End]
    Enosr,
    /// [OB XSR] [Option Start] Not a STREAM. [Option End]
    Enostr,
    /// Functionality not supported.
    Enosys,
    /// The socket is not connected.
    Enotconn,
    /// Not a directory or a symbolic link to a directory.
    Enotdir,
    /// Directory not empty.
    Enotempty,
    /// State not recoverable.
    Enotrecoverable,
    /// Not a socket.
    Enotsock,
    /// Not supported (may be the same value as [EOPNOTSUPP]).
    Enotsup,
    /// Inappropriate I/O control operation.
    Enotty,
    /// Operation not supported on socket (may be the same value as [ENOTSUP]).
    Eopnotsupp,
    /// Value too large to be stored in data type.
    Eoverflow,
    /// Previous owner died.
    Eownerdead,
    /// Broken pipe.
    Epipe,
    /// Protocol error.
    Eproto,
    /// Protocol not supported.
    Eprotonosupport,
    /// Protocol wrong type for socket.
    Eprototype,
    /// Result too large.
    Erange,
    /// Read-only file system.
    Erofs,
    /// Invalid seek.
    Espipe,
    /// Reserved.
    Estale,
    /// [OB XSR] [Option Start] Stream ioctl() timeout. [Option End]
    Etime,
    /// Connection timed out.
    Etimedout,
    /// Text file busy.
    Etxtbsy,
    /// Operation would block (may be the same value as [EAGAIN]).
    Ewouldblock,
}

impl From<CollectionAllocErr> for Errno {
    fn from(_e: CollectionAllocErr) -> Self {
        Errno::Enomem
    }
}
