//! this is LockForest, a Lock free Queue
use alloc::vec::Vec;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, AtomicUsize};
use core::sync::atomic::Ordering;

// true means locked
pub struct RawLock(AtomicBool);

/// a Raw lock implemeneted with Atomic
impl RawLock {
    fn try_lock(&self) -> bool {
        let current = false;
        let old = self.0.compare_and_swap(current, true, Ordering::SeqCst);
        old == current
    }
    fn unlock(&self) {
        self.0.store(false, Ordering::SeqCst);
    }
    fn new() -> Self {
        Self(AtomicBool::new(false))
    }
    // fn new_locked() -> Self {
    //     Self(AtomicBool::new(true))
    // }
}

pub struct LockGuard<'a, T>(&'a mut Lock<T>);

impl<'a, T> Drop for LockGuard<'a, T> {
    fn drop(&mut self) {
        self.0.raw_lock.unlock();
    }
}

impl<'a, T> Deref for LockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0.data
    }
}

impl<'a, T> DerefMut for LockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0.data
    }
}

/// A lock Wrapper for a generic Datatype
pub struct Lock<T> {
    data: T,
    raw_lock: RawLock,
}

impl<'a, T> Lock<T> {
    fn new(data: T) -> Self {
        Lock {
            data,
            raw_lock: RawLock::new(),
        }
    }
    fn try_lock(&'a self) -> Option<LockGuard<'a, T>> {
        if self.raw_lock.try_lock() {
            Some(LockGuard(unsafe {
                &mut *(self as *const Self as *mut Self)
            }))
        } else {
            None
        }
    }
}

/// Iterator which remove all elems
pub struct Drain<'a, T> {
    lock_forest: &'a LockForest<T>,
    index: usize,
}

impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.lock_forest.pop_after(self.index);
        self.index += 1;
        // at the end of iteration we reput the write index at the
        // begining
        if res.is_none() {
            self.lock_forest.write_index.store(0, Ordering::SeqCst);
        }
        res
    }
}

#[derive(Debug)]
pub struct PushOutOfMem;

pub struct LockForest<T> {
    /// A satic vec whose size never change
    list: Vec<Lock<Option<T>>>,
    /// where to start to write messages
    write_index: AtomicUsize,
}

impl<T> LockForest<T> {
    pub fn new(len: usize) -> LockForest<T> {
        let mut list = Vec::with_capacity(len);
        for _ in 0..len {
            list.push(Lock::new(None));
        }
        Self {
            list,
            write_index: AtomicUsize::new(0),
        }
    }

    fn push_after(&self, t: T, index: usize) -> Result<(), PushOutOfMem> {
        if index >= self.list.len() {
            return Err(PushOutOfMem)
        }
        for elem in &self.list[index..] {
            match elem.try_lock() {
                Some(mut elem) => {
                    if elem.is_some() {
                        continue;
                    }
                    *elem = Some(t);
                    return Ok(());
                }
                None => {
                    continue;
                }
            }
        }
        Err(PushOutOfMem)
    }

    /// push on the queue, O(n) complexity for the moment
    pub fn push(&self, t: T) -> Result<(), PushOutOfMem> {
        let index = self.write_index.fetch_add(1, Ordering::SeqCst);
        self.push_after(t,index)
    }

    /// Clears the LockForest, returning all value as an
    /// iterator. Keeps the allocated memory for reuse.
    pub fn drain(&self) -> Drain<T> {
        Drain {
            lock_forest: &self,
            index: 0,
        }
    }

    /// pop the next element after index `index` of the queue
    fn pop_after(&self, index: usize) -> Option<T> {
        if index > self.list.len() {
            return None;
        }
        for elem in &self.list[index..] {
            match elem.try_lock() {
                Some(mut elem) => {
                    if elem.is_none() {
                        continue;
                    }
                    return elem.take();
                }
                None => {
                    continue;
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a: LockForest<u32> = LockForest::new(10000);
        a.push(4).unwrap();
        a.push(2).unwrap();
        let mut iterator = a.drain();
        assert!(iterator.next() == Some(4));
        assert!(iterator.next() == Some(2));
        for i in 0..10000 {
            a.push(i).unwrap();
        }
        for (i, elem) in (0..10000).zip(a.drain()) {
            assert_eq!(i, elem);
        }
    }
}
