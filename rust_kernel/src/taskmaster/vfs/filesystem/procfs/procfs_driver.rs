use super::{Driver, FileOperation, IpcResult, SysResult};

use alloc::sync::Arc;
use fallible_collections::FallibleArc;

use core::fmt::Debug;

use libc_binding::OpenFlags;
use sync::DeadMutex;

pub trait ProcFsDriver: Clone {
    type Operations: FileOperation + Default + Send + Sized + 'static;

    fn new_operations(&mut self) -> Self::Operations {
        <Self::Operations>::default()
    }
}

impl<T: ProcFsDriver + Debug + Send> Driver for T {
    fn open(
        &mut self,
        _flags: OpenFlags,
    ) -> SysResult<IpcResult<Arc<DeadMutex<dyn FileOperation>>>> {
        let res = Arc::try_new(DeadMutex::new(self.new_operations()))?;
        Ok(IpcResult::Done(res))
    }
}
