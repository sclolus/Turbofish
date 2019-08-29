use super::DcacheError;
use libc_binding::Errno;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum VfsError {
    FileAlreadyExists,
    NoSuchEntry,
    NotADirectory,
    NotASymlink,
    InvalidEntryIdInDirectory,
    RootDoesNotExists,
    NotEmpty,
    EntryNotConnected,
    NotEnoughArguments,
    DirectoryNotMounted,
    DirectoryIsMounted,
    UndefinedHandler,

    MountError,
    NoSuchInode,
    InodeAlreadyExists,
    Errno(Errno),
}

pub type VfsResult<T> = Result<T, VfsError>;

impl From<DcacheError> for VfsError {
    fn from(value: DcacheError) -> Self {
        match value {
            DcacheError::FileAlreadyExists => VfsError::FileAlreadyExists,
            DcacheError::NoSuchEntry => VfsError::NoSuchEntry,
            DcacheError::NotADirectory => VfsError::NotADirectory,
            DcacheError::NotASymlink => VfsError::NotASymlink,
            DcacheError::InvalidEntryIdInDirectory => VfsError::InvalidEntryIdInDirectory,
            DcacheError::RootDoesNotExists => VfsError::RootDoesNotExists,
            DcacheError::NotEmpty => VfsError::NotEmpty,
            DcacheError::EntryNotConnected => VfsError::EntryNotConnected,
            DcacheError::NotEnoughArguments => VfsError::NotEnoughArguments,
            DcacheError::DirectoryNotMounted => VfsError::DirectoryNotMounted,
            DcacheError::DirectoryIsMounted => VfsError::DirectoryIsMounted,
            DcacheError::Errno(errno) => VfsError::Errno(errno),
        }
    }
}

impl From<Errno> for VfsError {
    fn from(value: Errno) -> Self {
        VfsError::Errno(value)
    }
}

impl From<VfsError> for core::option::NoneError {
    fn from(_value: VfsError) -> Self {
        core::option::NoneError
    }
}
