// #![deny(missing_docs)]

use super::{VfsResult, Inode, DeviceId};


struct FileOperations {
    open: Fn(&mut Inode, &mut File) -> VfsResult<usize>,
}

/// The Kernel side FileDescriptor struct
#[derive(Debug, Copy, Clone)]
struct File {
    device_id: DeviceId,
    inode_nbr: usize,
    // curr_offset: usize,
}
