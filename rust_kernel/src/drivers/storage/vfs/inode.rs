#[deny(missing_docs)]

use super::DevicdId;


struct InodeOperations {

}

pub struct Inode {
    device_id: DeviceId,
    inode_number: usize,
    inode_operations: InodeOperations,
}
