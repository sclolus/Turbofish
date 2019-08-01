#[macro_use]
use const_assert;

/// The maximum size (in bytes) of any component of a pathname.
/// By POSIX-2018 as:
/// "The interpretation of a pathname component is dependent on the value
/// of {NAME_MAX} and _POSIX_NO_TRUNC associated with the path prefix of
/// that component. If any pathname component is longer than {NAME_MAX},
/// the implementation shall consider this an error."
pub const NAME_MAX: usize = 256;


/// The maximum size (in bytes) of any path.
pub const PATH_MAX: usize = 4096;

/// Posix says:
/// 3.381 Symbolic Link
/// A type of file with the property that when the file is encountered during
/// pathname resolution, a string stored by the file is used to modify the pathname resolution.
/// The stored string has a length of {SYMLINK_MAX} bytes or fewer.
pub const SYMLINK_MAX: usize = 256;

pub const SYMLOOP_MAX: usize = _POSIX_SYMLOOP_MAX * 4;

pub const _POSIX_SYMLOOP_MAX: usize = 8;
const_assert!(SYMLOOP_MAX >= _POSIX_SYMLOOP_MAX);

/// {OPEN_MAX}
/// A value one greater than the maximum value that the system may assign to a newly-created file descriptor.
/// Minimum Acceptable Value: {_POSIX_OPEN_MAX}
pub const OPEN_MAX: usize = _POSIX_OPEN_MAX;

pub const _POSIX_OPEN_MAX: usize = 20;
const_assert!(OPEN_MAX >= _POSIX_OPEN_MAX);

/// {STREAM_MAX}
/// Maximum number of streams that one process can have open at one time. If defined, it has the same value as {FOPEN_MAX} (see <stdio.h>).
/// Minimum Acceptable Value: {_POSIX_STREAM_MAX}
pub const STREAM_MAX: usize = _POSIX_STREAM_MAX;

pub const _POSIX_STREAM_MAX: usize = 8;
const_assert!(STREAM_MAX >= _POSIX_STREAM_MAX);

/// {TTY_NAME_MAX}
/// Maximum length of terminal device name.
/// Minimum Acceptable Value: {_POSIX_TTY_NAME_MAX}
pub const TTY_NAME_MAX: usize = _POSIX_TTY_NAME_MAX;

pub const _POSIX_TTY_NAME_MAX: usize = 9;
const_assert!(TTY_NAME_MAX >= _POSIX_TTY_NAME_MAX);

pub type time_t = usize;

#[repr(C)]
pub struct timespec {
    seconds: time_t,
    nanoseconds: time_t,
}
