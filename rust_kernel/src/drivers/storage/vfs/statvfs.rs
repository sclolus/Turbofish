/// This is the layout of the statvfs structure as defined by POSIX-2018.
/// unsigned long f_bsize    File system block size.
/// unsigned long f_frsize   Fundamental file system block size.
/// fsblkcnt_t    f_blocks   Total number of blocks on file system in units of f_frsize.
/// fsblkcnt_t    f_bfree    Total number of free blocks.
/// fsblkcnt_t    f_bavail   Number of free blocks available to
///                          non-privileged process.
/// fsfilcnt_t    f_files    Total number of file serial numbers.
/// fsfilcnt_t    f_ffree    Total number of free file serial numbers.
/// fsfilcnt_t    f_favail   Number of file serial numbers available to
///                          non-privileged process.
/// unsigned long f_fsid     File system ID.
/// unsigned long f_flag     Bit mask of f_flag values.
/// unsigned long f_namemax  Maximum filename length.

type FsBlockCount = usize;
type FsFileCount = usize;
type FileSystemId = usize;


#[allow(snake_case)]
enum f_flag {
    ST_RDONLY,
    ST_NOSUID,
}

struct StatVfs {
    f_bsize: usize,
    f_frsize: usize,
    f_blocks: FsBlockCount,
    f_bfree: FsBlockCount,
    f_bavail: FsBlockCount,
    f_files: FsFileCount,
    f_ffree: FsFileCount,
    f_favail: FsFileCount,
    f_fsid: FileSystemId,
    f_flag: f_flag,
    f_namemax: usize,
}
