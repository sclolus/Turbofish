//! Implement Fallible Box
use super::TryClone;
use alloc::boxed::Box;
use alloc::collections::CollectionAllocErr;
use core::borrow::Borrow;

/// trait to implement Fallible Box
pub trait FallibleBox<T> {
    /// try creating a new box, returning a Result<Box<T>,
    /// CollectionAllocErr> if allocation failed
    fn try_new(t: T) -> Result<Self, CollectionAllocErr>
    where
        Self: Sized;
}

impl<T> FallibleBox<T> for Box<T> {
    fn try_new(t: T) -> Result<Self, CollectionAllocErr> {
        let mut g = alloc::alloc::Global;
        let ptr = core::alloc::Alloc::alloc_one(&mut g)?;
        unsafe {
            core::ptr::write(ptr.as_ptr(), t);
            Ok(Box::from_raw(ptr.as_ptr()))
        }
    }
}

impl<T: TryClone> TryClone for Box<T> {
    fn try_clone(&self) -> Result<Self, CollectionAllocErr> {
        Self::try_new(Borrow::<T>::borrow(self).try_clone()?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn boxed() {
        let mut v = Box::try_new(5).unwrap();
        assert_eq!(*v, 5);
        *v = 3;
        assert_eq!(*v, 3);
    }
}
