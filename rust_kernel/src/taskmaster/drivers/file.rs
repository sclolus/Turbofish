use alloc::sync::Arc;
use ext2::Ext2Filesystem;
use sync::DeadMutex;

/// a driver of an ext2 file
pub struct Ext2DriverFile {
    ext2: Arc<DeadMutex<Ext2Filesystem>>,
    inode_nbr: u32,
}

/// a file operation of an ext2 file
pub struct Ext2FileOperation {
    ext2_driver_file: Arc<DeadMutex<Ext2DriverFile>>,
    offset: u32,
}
