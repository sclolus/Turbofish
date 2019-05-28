//! contain the standard errno definition
#![cfg_attr(not(test), no_std)]

#[repr(i8)]
#[derive(Debug, Copy, Clone, PartialEq)]
/// standard error errno
pub enum Errno {
    /// Argument list too long.
    E2Big,
    /// Permission denied.
    Eacces,
    /// Address in use.
    Eaddrinuse,
    /// Address not available.
    Eaddrnotavail,
    /// Address family not supported.
    Eafnosupport,
    /// Resource unavailable, try again (may be the same value as [EWOULDBLOCK]).
    Eagain,
    /// Connection already in progress.
    Ealready,
    /// Bad file descriptor.
    Ebadf,
    /// Bad message.
    Ebadmsg,
    /// Device or resource busy.
    Ebusy,
    /// Operation canceled.
    Ecanceled,
    /// No child processes.
    Echild,
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
    /// File exists.
    Eexist,
    /// Bad address.
    Efault,
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
    /// Interrupted function.
    Eintr,
    /// Invalid argument.
    Einval,
    /// I/O error.
    Eio,
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
    /// No such file or directory.
    Enoent,
    /// Executable file format error.
    Enoexec,
    /// No locks available.
    Enolck,
    /// Reserved.
    Enolink,
    /// Not enough space.
    Enomem,
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
    /// No such device or address.
    Enxio,
    /// Operation not supported on socket (may be the same value as [ENOTSUP]).
    Eopnotsupp,
    /// Value too large to be stored in data type.
    Eoverflow,
    /// Previous owner died.
    Eownerdead,
    /// Operation not permitted.
    Eperm,
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
    /// No such process.
    Esrch,
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
    /// Cross-device link.
    Exdev,
}
