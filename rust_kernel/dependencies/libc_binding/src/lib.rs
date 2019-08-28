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
