// use super::{Driver, FileOperation, IpcResult, SysResult};

// use alloc::{boxed::Box, sync::Arc};

// use fallible_collections::{boxed::FallibleBox, FallibleArc};

// use core::fmt::Debug;

// use libc_binding::OpenFlags;
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
