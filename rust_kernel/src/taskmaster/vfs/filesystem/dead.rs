use super::FileSystem;

/// This type is the most basic implementation of a FileSystem.
/// It uses the default implementations of the methods of the FileSystem trait,
/// which return ENOSYS.
#[derive(Debug)]
pub struct DeadFileSystem;

impl FileSystem for DeadFileSystem {}
