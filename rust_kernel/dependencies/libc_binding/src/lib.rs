#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![feature(try_reserve)]
#![feature(underscore_const_names)] // has been stabilized in last rust stable.
#![cfg_attr(not(test), no_std)]
pub mod libc;
pub use libc::*;
// ::std::os::raw::c_char
pub type c_char = i8;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_void = i32;
pub type c_longlong = i64;
pub type c_long = i32;
pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type Pid = i32;

#[derive(Debug)]
pub struct InvalidSignum;

use core::convert::TryFrom;
use core::mem::transmute;
/// TryFrom boilerplate to get a Signum relative to raw value
impl TryFrom<u32> for Signum {
    type Error = InvalidSignum;
    fn try_from(n: u32) -> Result<Self, Self::Error> {
        if n >= 32 {
            return Err(InvalidSignum);
        } else {
            Ok(unsafe { transmute(n) })
        }
    }
}
#[macro_use]
extern crate const_assert;
extern crate alloc;
use alloc::collections::CollectionAllocErr;
impl From<CollectionAllocErr> for Errno {
    fn from(_e: CollectionAllocErr) -> Self {
        Errno::ENOMEM
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Errno {
    EPERM = EPERM,
    ENOENT = ENOENT,
    ESRCH = ESRCH,
    EINTR = EINTR,
    EIO = EIO,
    ENXIO = ENXIO,
    E2BIG = E2BIG,
    ENOEXEC = ENOEXEC,
    EBADF = EBADF,
    ECHILD = ECHILD,
    EAGAIN = EAGAIN,
    ENOMEM = ENOMEM,
    EACCES = EACCES,
    EFAULT = EFAULT,
    ENOTBLK = ENOTBLK,
    EBUSY = EBUSY,
    EEXIST = EEXIST,
    EXDEV = EXDEV,
    ENODEV = ENODEV,
    ENOTDIR = ENOTDIR,
    EISDIR = EISDIR,
    EINVAL = EINVAL,
    ENFILE = ENFILE,
    EMFILE = EMFILE,
    ENOTTY = ENOTTY,
    ETXTBSY = ETXTBSY,
    EFBIG = EFBIG,
    ENOSPC = ENOSPC,
    ESPIPE = ESPIPE,
    EROFS = EROFS,
    EMLINK = EMLINK,
    EPIPE = EPIPE,
    EDOM = EDOM,
    ERANGE = ERANGE,
    EDEADLK = EDEADLK,
    ENAMETOOLONG = ENAMETOOLONG,
    ENOLCK = ENOLCK,
    ENOSYS = ENOSYS,
    ENOTEMPTY = ENOTEMPTY,
    ELOOP = ELOOP,
    ENOMSG = ENOMSG,
    EIDRM = EIDRM,
    ECHRNG = ECHRNG,
    EL2NSYNC = EL2NSYNC,
    EL3HLT = EL3HLT,
    EL3RST = EL3RST,
    ELNRNG = ELNRNG,
    EUNATCH = EUNATCH,
    ENOCSI = ENOCSI,
    EL2HLT = EL2HLT,
    EBADE = EBADE,
    EBADR = EBADR,
    EXFULL = EXFULL,
    ENOANO = ENOANO,
    EBADRQC = EBADRQC,
    EBADSLT = EBADSLT,
    EBFONT = EBFONT,
    ENOSTR = ENOSTR,
    ENODATA = ENODATA,
    ETIME = ETIME,
    ENOSR = ENOSR,
    ENONET = ENONET,
    ENOPKG = ENOPKG,
    EREMOTE = EREMOTE,
    ENOLINK = ENOLINK,
    EADV = EADV,
    ESRMNT = ESRMNT,
    ECOMM = ECOMM,
    EPROTO = EPROTO,
    EMULTIHOP = EMULTIHOP,
    EDOTDOT = EDOTDOT,
    EBADMSG = EBADMSG,
    EOVERFLOW = EOVERFLOW,
    ENOTUNIQ = ENOTUNIQ,
    EBADFD = EBADFD,
    EREMCHG = EREMCHG,
    ELIBACC = ELIBACC,
    ELIBBAD = ELIBBAD,
    ELIBSCN = ELIBSCN,
    ELIBMAX = ELIBMAX,
    ELIBEXEC = ELIBEXEC,
    EILSEQ = EILSEQ,
    ERESTART = ERESTART,
    ESTRPIPE = ESTRPIPE,
    EUSERS = EUSERS,
    ENOTSOCK = ENOTSOCK,
    EDESTADDRREQ = EDESTADDRREQ,
    EMSGSIZE = EMSGSIZE,
    EPROTOTYPE = EPROTOTYPE,
    ENOPROTOOPT = ENOPROTOOPT,
    EPROTONOSUPPORT = EPROTONOSUPPORT,
    ESOCKTNOSUPPORT = ESOCKTNOSUPPORT,
    EOPNOTSUPP = EOPNOTSUPP,
    EPFNOSUPPORT = EPFNOSUPPORT,
    EAFNOSUPPORT = EAFNOSUPPORT,
    EADDRINUSE = EADDRINUSE,
    EADDRNOTAVAIL = EADDRNOTAVAIL,
    ENETDOWN = ENETDOWN,
    ENETUNREACH = ENETUNREACH,
    ENETRESET = ENETRESET,
    ECONNABORTED = ECONNABORTED,
    ECONNRESET = ECONNRESET,
    ENOBUFS = ENOBUFS,
    EISCONN = EISCONN,
    ENOTCONN = ENOTCONN,
    ESHUTDOWN = ESHUTDOWN,
    ETOOMANYREFS = ETOOMANYREFS,
    ETIMEDOUT = ETIMEDOUT,
    ECONNREFUSED = ECONNREFUSED,
    EHOSTDOWN = EHOSTDOWN,
    EHOSTUNREACH = EHOSTUNREACH,
    EALREADY = EALREADY,
    EINPROGRESS = EINPROGRESS,
    ESTALE = ESTALE,
    EUCLEAN = EUCLEAN,
    ENOTNAM = ENOTNAM,
    ENAVAIL = ENAVAIL,
    EISNAM = EISNAM,
    EREMOTEIO = EREMOTEIO,
    EDQUOT = EDQUOT,
    ENOMEDIUM = ENOMEDIUM,
    EMEDIUMTYPE = EMEDIUMTYPE,
}

impl Errno {
    pub const EACCESS: Errno = Errno::EACCES;
}
impl Errno {
    pub const EWOULDBLOCK: Errno = Errno::EAGAIN;
}
impl Errno {
    pub const EDEADLOCK: Errno = Errno::EDEADLK;
}

impl Signum {
    pub const SIGIOT: Signum = Signum::SIGABRT;
}
impl Signum {
    pub const SIGCLD: Signum = Signum::SIGCHLD;
}
impl Signum {
    pub const SIGPOLL: Signum = Signum::SIGIO;
}
impl Signum {
    pub const SIGUNUSED: Signum = Signum::SIGSYS;
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Signum {
    SIGNULL = SIGNULL,
    SIGHUP = SIGHUP,
    SIGINT = SIGINT,
    SIGQUIT = SIGQUIT,
    SIGILL = SIGILL,
    SIGTRAP = SIGTRAP,
    SIGABRT = SIGABRT,
    SIGBUS = SIGBUS,
    SIGFPE = SIGFPE,
    SIGKILL = SIGKILL,
    SIGUSR1 = SIGUSR1,
    SIGSEGV = SIGSEGV,
    SIGUSR2 = SIGUSR2,
    SIGPIPE = SIGPIPE,
    SIGALRM = SIGALRM,
    SIGTERM = SIGTERM,
    SIGSTKFLT = SIGSTKFLT,
    SIGCHLD = SIGCHLD,
    SIGCONT = SIGCONT,
    SIGSTOP = SIGSTOP,
    SIGTSTP = SIGTSTP,
    SIGTTIN = SIGTTIN,
    SIGTTOU = SIGTTOU,
    SIGURG = SIGURG,
    SIGXCPU = SIGXCPU,
    SIGXFSZ = SIGXFSZ,
    SIGVTALRM = SIGVTALRM,
    SIGPROF = SIGPROF,
    SIGWINCH = SIGWINCH,
    SIGIO = SIGIO,
    SIGPWR = SIGPWR,
    SIGSYS = SIGSYS,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FcntlCmd {
    F_DUPFD = F_DUPFD,
    F_DUPFD_CLOEXEC = F_DUPFD_CLOEXEC,
    F_GETFD = F_GETFD,
    F_SETFD = F_SETFD,
    F_GETFL = F_GETFL,
    F_SETFL = F_SETFL,
    F_GETLK = F_GETLK,
    F_SETLK = F_SETLK,
    F_SETLKW = F_SETLKW,
    F_GETOWN = F_GETOWN,
    F_SETOWN = F_SETOWN,
}

//// The number of I/O operations that can be specified in a list I/O call.
// pub const _POSIX_AIO_LISTIO_MAX: usize = 2;
// const_assert!(AIO_LISTIO_MAX >= _POSIX_AIO_LISTIO_MAX);

//// The number of outstanding asynchronous I/O operations.
// pub const _POSIX_AIO_MAX: usize = 1;
// const_assert!(AIO_MAX >= _POSIX_AIO_MAX);

//// Maximum length of argument to the exec functions including environment data.
// pub const _POSIX_ARG_MAX: usize = 4_096;
// const_assert!(ARG_MAX >= _POSIX_ARG_MAX);

//// Maximum number of simultaneous processes per real user ID.
// pub const _POSIX_CHILD_MAX: usize = 25;
// const_assert!(CHILD_MAX >= _POSIX_CHILD_MAX);

//// The number of timer expiration overruns.
// pub const _POSIX_DELAYTIMER_MAX: usize = 32;
// const_assert!(DELAYTIMER_MAX >= _POSIX_DELAYTIMER_MAX);

//// Maximum length of a host name (not including the terminating null) as returned from the gethostname() function.
// pub const _POSIX_HOST_NAME_MAX: usize = 255;
// const_assert!(HOST_NAME_MAX >= _POSIX_HOST_NAME_MAX);

//// Maximum number of links to a single file.
// pub const _POSIX_LINK_MAX: usize = 8;
// const_assert!(LINK_MAX >= _POSIX_LINK_MAX);

//// The size of the storage required for a login name, in bytes (including the terminating null).
// pub const _POSIX_LOGIN_NAME_MAX: usize = 9;
// const_assert!(LOGIN_NAME_MAX >= _POSIX_LOGIN_NAME_MAX);

//// Maximum number of bytes in a terminal canonical input queue.
// pub const _POSIX_MAX_CANON: usize = 255;
// const_assert!(MAX_CANON >= _POSIX_MAX_CANON);

//// Maximum number of bytes allowed in a terminal input queue.
// pub const _POSIX_MAX_INPUT: usize = 255;
// const_assert!(MAX_INPUT >= _POSIX_MAX_INPUT);

//// The number of message queues that can be open for a single process.
// pub const _POSIX_MQ_OPEN_MAX: usize = 8;
// const_assert!(MQ_OPEN_MAX >= _POSIX_MQ_OPEN_MAX);

//// The maximum number of message priorities supported by the implementation.
// pub const _POSIX_MQ_PRIO_MAX: usize = 32;
// const_assert!(MQ_PRIO_MAX >= _POSIX_MQ_PRIO_MAX);

/// Maximum number of bytes in a filename (not including the terminating null of a filename string).
pub const _POSIX_NAME_MAX: usize = 14;
const_assert!(NAME_MAX as usize >= _POSIX_NAME_MAX);

//// Maximum number of simultaneous supplementary group IDs per process.
// pub const _POSIX_NGROUPS_MAX: usize = 8;
// const_assert!(NGROUPS_MAX >= _POSIX_NGROUPS_MAX);

// pub const _POSIX_OPEN_MAX: usize = 20;
// const_assert!(OPEN_MAX >= _POSIX_OPEN_MAX);

/// Minimum number the implementation will accept as the maximum number of bytes in a pathname.
pub const _POSIX_PATH_MAX: usize = 256;
const_assert!(PATH_MAX as usize >= _POSIX_PATH_MAX);

//// Maximum number of bytes that is guaranteed to be atomic when writing to a pipe.
// pub const _POSIX_PIPE_BUF: usize = 512;
// const_assert!(PIPE_BUF as usize >= _POSIX_PIPE_BUF);

//// Maximum number of repeated occurrences of a BRE or ERE interval expression; see BREs Matching Multiple Characters and EREs Matching Multiple Characters.
// pub const _POSIX_RE_DUP_MAX: usize = 255;
// const_assert!(RE_DUP_MAX >= _POSIX_RE_DUP_MAX);

//// The number of realtime signal numbers reserved for application use.
// pub const _POSIX_RTSIG_MAX: usize = 8;
// const_assert!(RTSIG_MAX >= _POSIX_RTSIG_MAX);

//// The number of semaphores that a process may have.
// pub const _POSIX_SEM_NSEMS_MAX: usize = 256;
// const_assert!(SEM_NSEMS_MAX >= _POSIX_SEM_NSEMS_MAX);

//// The maximum value a semaphore may have.
// pub const _POSIX_SEM_VALUE_MAX: usize = 32_767;
// const_assert!(SEM_VALUE_MAX >= _POSIX_SEM_VALUE_MAX);

//// The number of queued signals that a process may send and have pending at the receiver(s) at any time.
// pub const _POSIX_SIGQUEUE_MAX: usize = 32;
// const_assert!(SIGQUEUE_MAX >= _POSIX_SIGQUEUE_MAX);

//// The value that can be stored in an object of type ssize_t.
// pub const _POSIX_SSIZE_MAX: usize = 32_767;
// const_assert!(SSIZE_MAX >= _POSIX_SSIZE_MAX);

//// The number of replenishment operations that may be simultaneously pending for a particular sporadic server scheduler.
// pub const _POSIX_SS_REPL_MAX: usize = 4;
// const_assert!(SS_REPL_MAX >= _POSIX_SS_REPL_MAX);

//// The number of streams that one process can have open at one time.
// pub const _POSIX_STREAM_MAX: usize = 8;
// const_assert!(STREAM_MAX >= _POSIX_STREAM_MAX);

//// The number of bytes in a symbolic link.
// pub const _POSIX_SYMLINK_MAX: usize = 255;
// const_assert!(SYMLINK_MAX >= _POSIX_SYMLINK_MAX);

//// The number of symbolic links that can be traversed in the resolution of a pathname in the absence of a loop.
// pub const _POSIX_SYMLOOP_MAX: usize = 8;
// const_assert!(SYMLOOP_MAX >= _POSIX_SYMLOOP_MAX);

//// The number of data keys per process.
// pub const _POSIX_THREAD_DESTRUCTOR_ITERATIONS: usize = 4;
// const_assert!(THREAD_DESTRUCTOR_ITERATIONS >= _POSIX_THREAD_DESTRUCTOR_ITERATIONS);

//// The number of data keys per process.
// pub const _POSIX_THREAD_KEYS_MAX: usize = 128;
// const_assert!(THREAD_KEYS_MAX >= _POSIX_THREAD_KEYS_MAX);

//// The number of threads per process.
// pub const _POSIX_THREAD_THREADS_MAX: usize = 64;
// const_assert!(THREAD_THREADS_MAX >= _POSIX_THREAD_THREADS_MAX);

//// The per-process number of timers.
// pub const _POSIX_TIMER_MAX: usize = 32;
// const_assert!(TIMER_MAX >= _POSIX_TIMER_MAX);

//// The length in bytes of a trace event name (not including the terminating null).
// pub const _POSIX_TRACE_EVENT_NAME_MAX: usize = 30;
// const_assert!(TRACE_EVENT_NAME_MAX >= _POSIX_TRACE_EVENT_NAME_MAX);

//// The length in bytes of a trace generation version string or a trace stream name (not including the terminating null).
// pub const _POSIX_TRACE_NAME_MAX: usize = 8;
// const_assert!(TRACE_NAME_MAX >= _POSIX_TRACE_NAME_MAX);

//// The number of trace streams that may simultaneously exist in the system.
// pub const _POSIX_TRACE_SYS_MAX: usize = 8;
// const_assert!(TRACE_SYS_MAX >= _POSIX_TRACE_SYS_MAX);

//// The number of user trace event type identifiers that may simultaneously exist in a traced process, including the predefined user trace event POSIX_TRACE_UNNAMED_USER_EVENT.
// pub const _POSIX_TRACE_USER_EVENT_MAX: usize = 32;
// const_assert!(TRACE_USER_EVENT_MAX >= _POSIX_TRACE_USER_EVENT_MAX);

//// The size of the storage required for a terminal device name, in bytes (including the terminating null).
// pub const _POSIX_TTY_NAME_MAX: usize = 9;
// const_assert!(TTY_NAME_MAX >= _POSIX_TTY_NAME_MAX);

//// Maximum number of bytes supported for the name of a timezone (not of the TZ variable).
// pub const _POSIX_TZNAME_MAX: usize = 6;
// const_assert!(TZNAME_MAX >= _POSIX_TZNAME_MAX);

//// Maximum obase values allowed by the bc utility.
// pub const _POSIX2_BC_BASE_MAX: usize = 99;
// const_assert!(BC_BASE_MAX >= _POSIX2_BC_BASE_MAX);

//// Maximum number of elements permitted in an array by the bc utility.
// pub const _POSIX2_BC_DIM_MAX: usize = 2_048;
// const_assert!(BC_DIM_MAX >= _POSIX2_BC_DIM_MAX);

//// Maximum number of bytes in a character class name.
// pub const _POSIX2_BC_SCALE_MAX: usize = 99;
// const_assert!(BC_SCALE_MAX >= _POSIX2_BC_SCALE_MAX);

//// Maximum length of a string constant accepted by the bc utility.
// pub const _POSIX2_BC_STRING_MAX: usize = 1_000;
// const_assert!(BC_STRING_MAX >= _POSIX2_BC_STRING_MAX);

//// Maximum number of bytes in a character class name.
// pub const _POSIX2_CHARCLASS_NAME_MAX: usize = 14;
// const_assert!(CHARCLASS_NAME_MAX >= _POSIX2_CHARCLASS_NAME_MAX);

//// Maximum number of weights that can be assigned to an entry of the LC_COLLATE order keyword in the locale definition file
// pub const _POSIX2_COLL_WEIGHTS_MAX: usize = 2;
// const_assert!(COLL_WEIGHTS_MAX >= _POSIX2_COLL_WEIGHTS_MAX);

//// Maximum number of expressions that can be nested within parentheses by the expr utility.
// pub const _POSIX2_EXPR_NEST_MAX: usize = 32;
// const_assert!(EXPR_NEST_MAX >= _POSIX2_EXPR_NEST_MAX);

//// Unless otherwise noted, the maximum length, in bytes, of a utility's input line (either standard input or another file), when the utility is described as processing text files. The length includes room for the trailing <newline>.
// pub const _POSIX2_LINE_MAX: usize = 2_048;
// const_assert!(LINE_MAX >= _POSIX2_LINE_MAX);

///// Maximum number of repeated occurrences of a BRE or ERE interval expression; see BREs Matching Multiple Characters and EREs Matching Multiple Characters.
// pub const _POSIX2_RE_DUP_MAX: usize = 255;
// const_assert!(RE_DUP_MAX >= _POSIX2_RE_DUP_MAX);

//// Maximum number of iovec structures that one process has available for use with readv() or writev().
// pub const _XOPEN_IOV_MAX: usize =  16 ;

// pub const _XOPEN_NAME_MAX: usize =  255 ;
// pub const _XOPEN_PATH_MAX: usize =  1024 ;

use bitflags::bitflags;

bitflags! {
    #[derive(Default)] // I wonder for this derive <.<
    pub struct OpenFlags: u32 {
        /// Open for execute only (non-directory files). The result is
        /// unspecified if this flag is applied to a directory.
        const O_EXEC = O_EXEC;

        /// Open for reading only.
        const O_RDONLY = O_RDONLY;

        /// Open for reading and writing. The result is undefined if
        /// this flag is applied to a FIFO.
        const O_RDWR = O_RDWR;

        /// Open directory for search only. The result is unspecified
        /// if this flag is applied to a non-directory file.
        const O_SEARCH = O_SEARCH;

        /// Open for writing only.
        const O_WRONLY = O_WRONLY;

        /// If set, the file offset shall be set to the end of the
        /// file prior to each write.
        const O_APPEND = O_APPEND;

        /// If set, the FD_CLOEXEC flag for the new file descriptor
        /// shall be set.
        const O_CLOEXEC = O_CLOEXEC;

        /// If the file exists, this flag has no effect except as
        /// noted under O_EXCL below.  Otherwise, if O_DIRECTORY is
        /// not set the file shall be created as a regular file; the
        /// user ID of the file shall be set to the effective user ID
        /// of the process; the group ID of the file shall be set to
        /// the group ID of the file's parent directory or to the
        /// effective group ID of the process; and the access
        /// permission bits (see <sys/stat.h>) of the file mode shall
        /// be set to the value of the argument following the oflag
        /// argument taken as type mode_t modified as follows: a
        /// bitwise AND is performed on the file-mode bits and the
        /// corresponding bits in the complement of the process' file
        /// mode creation mask. Thus, all bits in the file mode whose
        /// corresponding bit in the file mode creation mask is set
        /// are cleared. When bits other than the file permission bits
        /// are set, the effect is unspecified. The argument following
        /// the oflag argument does not affect whether the file is
        /// open for reading, writing, or for both. Implementations
        /// shall provide a way to initialize the file's group ID to
        /// the group ID of the parent directory. Implementations may,
        /// but need not, provide an implementation-defined way to
        /// initialize the file's group ID to the effective group ID
        /// of the calling process.
        // do something about this pave
        const O_CREAT = O_CREAT;

        /// If path resolves to a non-directory file, fail and set errno to [ENOTDIR].
        const O_DIRECTORY = O_DIRECTORY;

        /// Write I/O operations on the file descriptor shall complete
        /// as defined by synchronized I/O data integrity
        /// completion. [Option End]
        const O_DSYNC = O_DSYNC;

        /// If O_CREAT and O_EXCL are set, open() shall fail if the
        /// file exists. The check for the existence of the file and
        /// the creation of the file if it does not exist shall be
        /// atomic with respect to other threads executing open()
        /// naming the same filename in the same directory with O_EXCL
        /// and O_CREAT set. If O_EXCL and O_CREAT are set, and path
        /// names a symbolic link, open() shall fail and set errno to
        /// [EEXIST], regardless of the contents of the symbolic
        /// link. If O_EXCL is set and O_CREAT is not set, the result
        /// is undefined.
        const O_EXCL = O_EXCL;

        /// If set and path identifies a terminal device, open() shall
        /// not cause the terminal device to become the controlling
        /// terminal for the process. If path does not identify a
        /// terminal device, O_NOCTTY shall be ignored.
        const O_NOCTTY = O_NOCTTY;

        /// If path names a symbolic link, fail and set errno to [ELOOP].
        const O_NOFOLLOW = O_NOFOLLOW;

        /// When opening a FIFO with O_RDONLY or O_WRONLY set: If
        /// O_NONBLOCK is set, an open() for reading-only shall return
        /// without delay. An open() for writing-only shall return an
        /// error if no process currently has the file open for
        /// reading.
        ///
        /// If O_NONBLOCK is clear, an open() for reading-only shall
        /// block the calling thread until a thread opens the file for
        /// writing. An open() for writing-only shall block the
        /// calling thread until a thread opens the file for reading.
        ///
        /// When opening a block special or character special file
        /// that supports non-blocking opens:
        ///
        /// If O_NONBLOCK is set, the open() function shall return
        /// without blocking for the device to be ready or
        /// available. Subsequent behavior of the device is
        /// device-specific.
        ///
        /// If O_NONBLOCK is clear, the open() function shall block
        /// the calling thread until the device is ready or available
        /// before returning.
        ///
        /// Otherwise, the O_NONBLOCK flag shall not cause an error,
        /// but it is unspecified whether the file status flags will
        /// include the O_NONBLOCK flag.
        const O_NONBLOCK = O_NONBLOCK;

        /// Read I/O operations on the file descriptor shall complete
        /// at the same level of integrity as specified by the O_DSYNC
        /// and O_SYNC flags. If both O_DSYNC and O_RSYNC are set in
        /// oflag, all I/O operations on the file descriptor shall
        /// complete as defined by synchronized I/O data integrity
        /// completion. If both O_SYNC and O_RSYNC are set in flags,
        /// all I/O operations on the file descriptor shall complete
        /// as defined by synchronized I/O file integrity
        /// completion. [Option End]
        const O_RSYNC = O_RSYNC;

        /// Write I/O operations on the file descriptor shall complete
        /// as defined by synchronized I/O file integrity
        /// completion. [Option End]
        /// The O_SYNC flag shall be supported for regular files, even
        /// if the Synchronized Input and Output option is not
        /// supported. [Option End]
        const O_SYNC = O_SYNC;

        /// If the file exists and is a regular file, and the file is
        /// successfully opened O_RDWR or O_WRONLY, its length shall
        /// be truncated to 0, and the mode and owner shall be
        /// unchanged. It shall have no effect on FIFO special files
        /// or terminal device files. Its effect on other file types
        /// is implementation-defined. The result of using O_TRUNC
        /// without either O_RDWR or O_WRONLY is undefined.
        const O_TRUNC = O_TRUNC;

        /// If path identifies a terminal device other than a
        /// pseudo-terminal, the device is not already open in any
        /// process, and either O_TTY_INIT is set in oflag or
        /// O_TTY_INIT has the value zero, open() shall set any
        /// non-standard termios structure terminal parameters to a
        /// state that provides conforming behavior; see XBD
        /// Parameters that Can be Set. It is unspecified whether
        /// O_TTY_INIT has any effect if the device is already open in
        /// any process. If path identifies the slave side of a
        /// pseudo-terminal that is not already open in any process,
        /// open() shall set any non-standard termios structure
        /// terminal parameters to a state that provides conforming
        /// behavior, regardless of whether O_TTY_INIT is set. If path
        /// does not identify a terminal device, O_TTY_INIT shall be
        /// ignored.
        const O_TTY_INIT = O_TTY_INIT;
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Whence {
    SeekSet = SEEK_SET,
    SeekCur = SEEK_CUR,
    SeekEnd = SEEK_END,
}

impl TryFrom<u32> for Whence {
    type Error = Errno;
    fn try_from(n: u32) -> Result<Self, Self::Error> {
        Ok(match n {
            SEEK_SET => Whence::SeekSet,
            SEEK_CUR => Whence::SeekCur,
            SEEK_END => Whence::SeekEnd,
            _ => Err(Errno::EINVAL)?,
        })
    }
}

bitflags! {
    #[derive(Default)]
    pub struct FileType: u16 {
        const S_IFMT = S_IFMT as u16;
        const UNIX_SOCKET = S_IFSOCK as u16;
        const SYMBOLIC_LINK = S_IFLNK as u16;
        const REGULAR_FILE = S_IFREG as u16;
        const BLOCK_DEVICE = S_IFBLK as u16;
        const DIRECTORY = S_IFDIR as u16;
        const CHARACTER_DEVICE = S_IFCHR as u16;
        const FIFO = S_IFIFO as u16;

        const SET_USER_ID = S_ISUID as u16;
        const SET_GROUP_ID = S_ISGID as u16;
        const STICKY_BIT = S_ISVTX as u16;

        const S_IRWXU = S_IRWXU as u16;
        const USER_READ_PERMISSION = S_IRUSR as u16;
        const USER_WRITE_PERMISSION = S_IWUSR as u16;
        const USER_EXECUTE_PERMISSION = S_IXUSR as u16;

        const S_IRWXG = S_IRWXG as u16;
        const GROUP_READ_PERMISSION = S_IRGRP as u16;
        const GROUP_WRITE_PERMISSION = S_IWGRP as u16;
        const GROUP_EXECUTE_PERMISSION = S_IXGRP as u16;

        const S_IRWXO = S_IRWXO as u16;
        const OTHER_READ_PERMISSION = S_IROTH as u16;
        const OTHER_WRITE_PERMISSION = S_IWOTH as u16;
        const OTHER_EXECUTE_PERMISSION = S_IXOTH as u16;
        const PERMISSIONS_MASK = S_IRWXU as u16 |
                                 S_IRWXG as u16 |
                                 S_IRWXO as u16;
    }
}

impl FileType {
    pub fn is_character_device(&self) -> bool {
        self.contains(Self::CHARACTER_DEVICE)
    }

    pub fn is_fifo(&self) -> bool {
        self.contains(Self::FIFO)
    }

    pub fn is_regular(&self) -> bool {
        self.contains(Self::REGULAR_FILE)
    }

    pub fn is_directory(&self) -> bool {
        self.contains(Self::DIRECTORY)
    }

    pub fn is_symlink(&self) -> bool {
        self.contains(Self::SYMBOLIC_LINK)
    }

    pub fn is_socket(&self) -> bool {
        self.contains(Self::UNIX_SOCKET)
    }

    /// retrurn the owner rights on the file, in a bitflags Amode
    pub fn owner_access(&self) -> Amode {
        Amode::from_bits((*self & FileType::S_IRWXU).bits() as u32 >> 6)
            .expect("bits should be valid")
    }

    /// retrurn the group rights on the file, in a bitflags Amode
    pub fn group_access(&self) -> Amode {
        Amode::from_bits((*self & FileType::S_IRWXG).bits() as u32 >> 3)
            .expect("bits should be valid")
    }

    /// retrurn the other rights on the file, in a bitflags Amode
    pub fn other_access(&self) -> Amode {
        Amode::from_bits((*self & FileType::S_IRWXO).bits() as u32).expect("bits should be valid")
    }
}

bitflags! {
    pub struct Amode: u32 {
        const F_OK = F_OK;
        const R_OK = R_OK;
        const W_OK = W_OK;
        const X_OK = X_OK;
    }
}
