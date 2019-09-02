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

impl From<DcacheError> for Errno {
    fn from(dcache_error: DcacheError) -> Errno {
        match dcache_error {
            DcacheError::Errno(e) => e,
            // TODO: check that
            _ => Errno::EINVAL,
        }
    }
}

impl From<VfsError> for Errno {
    fn from(vfs_error: VfsError) -> Errno {
        match vfs_error {
            VfsError::Errno(e) => e,
            // TODO: check that
            _ => Errno::EINVAL,
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

#[derive(Debug, Copy, Clone)]
pub enum DcacheError {
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
    Errno(Errno),
}

impl From<Errno> for DcacheError {
    fn from(errno: Errno) -> Self {
        DcacheError::Errno(errno)
    }
}

pub type DcacheResult<T> = Result<T, DcacheError>;

use core::cmp::Ord;
use core::ops::Add;

use alloc::collections::BTreeMap;

pub trait KeyGenerator<K>
where
    K: Ord + Add<usize, Output = K> + Default + Copy,
{
    fn gen(&mut self) -> K {
        let mut cur = K::default();

        while !self.gen_filter(cur) {
            cur = cur + 1
        }
        cur
    }

    fn gen_filter(&self, _key: K) -> bool {
        true
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum MapperError {
    EntryAlreadyExists,
    NoSuchEntry,
}

#[allow(dead_code)]
pub type MapperResult<T> = Result<T, MapperError>;

pub trait Mapper<K, V>: KeyGenerator<K>
where
    K: Ord + Add<usize, Output = K> + Default + Copy,
{
    fn get_map(&mut self) -> &mut BTreeMap<K, V>;

    fn add_entry(&mut self, entry: V) -> MapperResult<K> {
        let key = self.gen();
        let map = self.get_map();

        if map.contains_key(&key) {
            panic!("Mapper: KeyGenerator::gen() returned a contained key");
        }

        map.insert(key, entry);
        Ok(key)
    }

    fn remove_entry(&mut self, key: K) -> MapperResult<V> {
        let map = self.get_map();
        if !map.contains_key(&key) {
            return Err(MapperError::NoSuchEntry);
        }
        Ok(map
            .remove(&key)
            .expect("Entry is unexpectedly not contained"))
    }
}
