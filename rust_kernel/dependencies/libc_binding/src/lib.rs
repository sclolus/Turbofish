#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![feature(try_reserve)]
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
