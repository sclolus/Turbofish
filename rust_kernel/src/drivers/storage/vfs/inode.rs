#[deny(missing_docs)]


struct InodeOperations {

}

pub struct Inode {
    inode_number: usize,
    inode_operations: InodeOperations,
}
