use super::{Driver, FileOperation, IpcResult, SysResult, VFS};

// use alloc::{boxed::Box, sync::Arc};

// use fallible_collections::{boxed::FallibleBox, FallibleArc};

// use core::fmt::Debug;

use libc_binding::Errno;
// use sync::DeadMutex;

// type Mutex<T> = DeadMutex<T>;

// pub trait ProcFsDriver: Driver + Clone {
//     // fn new_operations(&mut self) -> Box<dyn FileOperation>;
// }

// impl<T: ProcFsDriver + Debug + Send> Driver for T {
//     fn open(
//         &mut self,
//         _flags: OpenFlags,
//     ) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>> {
//         let res = Arc::try_new(DeadMutex::new(self.new_operations()))?;
//         Ok(IpcResult::Done(res))
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, is_enum_variant)]
// pub enum DriverKind {
//     Version,
//     Filesystems,
// }

// static mut DRIVER_DISPATCHER: Mutex<[(Box<dyn FnOnce() -> Box<dyn ProcFsDriver>>, DriverKind); 2]> =
//     Mutex::new([
//         (
//             Box::new(|| Box::new(super::version::VersionDriver)),
//             DriverKind::Version,
//         ),
//         (
//             Box::new(|| Box::new(super::filesystems::FilesystemsDriver)),
//             DriverKind::Filesystems,
//         ),
//     ]);

// fn gen_driver(kind: DriverKind) -> Box<ProcFsDriver> {
//     // Box<T>::try_new() requires that T: Sized. We should fix this.
//     DRIVER_DISPATCHER
//         .lock()
//         .iter()
//         .find(|(_, driver_kind)| kind == *driver_kind)
//         .map(|(function, _)| function)
//         .expect("No corresponding function to create requested ProcFsDriver kind.")()
// }

pub trait ProcFsOperations: FileOperation {
    fn get_seq_string(&self) -> SysResult<&str> {
        Err(Errno::ENOSYS)
    }

    fn get_offset(&mut self) -> &mut usize;

    fn seq_read(&mut self, buf: &mut [u8]) -> SysResult<IpcResult<u32>> {
        if buf.len() > u32::max_value() as usize {
            return Err(Errno::EOVERFLOW);
        }
        let offset = *self.get_offset();
        let seq_string = self.get_seq_string()?;

        if offset >= seq_string.len() {
            return Ok(IpcResult::Done(0));
        }

        let seq_string = &seq_string[offset as usize..];

        let mut bytes = seq_string.bytes();

        let mut ret = 0;
        for (index, to_fill) in buf.iter_mut().enumerate() {
            match bytes.next() {
                Some(byte) => *to_fill = byte,
                None => break,
            }
            ret += 1;
        }
        *self.get_offset() += ret;
        Ok(IpcResult::Done(ret as u32))
    }
}

// impl<T: ProcFsOperations> Drop for T {
//     fn drop(&mut self) {
//         let inode_id = self.get_inode_id();
//         eprintln!("====== Operations DROP: {:?}========", inode_id);
//         VFS.lock().close_file_operation(inode_id);
//     }
// }
