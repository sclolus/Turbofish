#[deny(missing_docs)]

/// The maximum size (in bytes) of any component of a pathname.
/// By POSIX-2018 as:
/// "The interpretation of a pathname component is dependent on the value
/// of {NAME_MAX} and _POSIX_NO_TRUNC associated with the path prefix of
/// that component. If any pathname component is longer than {NAME_MAX},
/// the implementation shall consider this an error."
pub const NAME_MAX: usize = 256;


/// The maximum size (in bytes) of any path.
pub const PATH_MAX: usize = 4096;
