use super::allocator::{BuddyAllocator, VirtualPageAllocator};
use crate::memory::mmu::{Entry, PageDirectory};
use crate::memory::tools::*;
pub use crate::taskmaster::{CString, CStringArray};
use alloc::vec::Vec;
use core::convert::Into;
use core::mem::size_of;
use fallible_collections::{try_vec, FallibleVec};
use libc_binding::c_char;

#[derive(Debug)]
/// Virtual Allocator Specialized for processus
pub struct AddressSpace(VirtualPageAllocator);

impl AddressSpace {
    pub unsafe fn try_new() -> Result<Self> {
        let mut buddy = BuddyAllocator::new(Page::new(0x0), NbrPages::_3GB)?;
        buddy
            .reserve_exact(Page::new(0x0), NbrPages::_4MB)
            .expect("User Buddy won't collaborate");

        let pd = PageDirectory::new_for_process()?;

        Ok(Self(VirtualPageAllocator::new(buddy, pd)))
    }

    /// the process forker must be the current cr3
    pub fn fork(&self) -> Result<Self> {
        Ok(Self(self.0.fork()?))
    }

    /// Check if a pointer given by user process is not bullshit
    fn check_user_ptr_predicate<T, P>(&self, ptr: *const T, predicate: P) -> Result<()>
    where
        P: Fn(Entry) -> bool,
    {
        let start_ptr = Virt(ptr as usize);
        let end_ptr = Virt(
            (ptr as usize)
                .checked_add(size_of::<T>() - 1)
                .ok_or(MemoryError::BadAddr)?,
        );

        Ok(self
            .0
            .check_page_range(start_ptr.into(), end_ptr.into(), predicate)
            .map_err(|_| MemoryError::BadAddr)?)
    }

    /// Check if a pointer given by user process is not bullshit
    /// length is in number of T
    fn check_user_ptr_predicate_with_len<T, P>(
        &self,
        ptr: *const T,
        length: usize,
        predicate: P,
    ) -> Result<()>
    where
        P: Fn(Entry) -> bool,
    {
        if length == 0 {
            return Ok(());
        }
        let start_ptr = Virt(ptr as usize);
        let end_ptr = Virt(
            (ptr as usize)
                .checked_add(length * size_of::<T>() - 1)
                .ok_or(MemoryError::BadAddr)?,
        );

        Ok(self
            .0
            .check_page_range(start_ptr.into(), end_ptr.into(), predicate)
            .map_err(|_| MemoryError::BadAddr)?)
    }

    /// check is a user ptr with len is valid for READING
    pub fn check_user_ptr_with_len<T>(&self, ptr: *const T, length: usize) -> Result<()> {
        self.check_user_ptr_predicate_with_len(ptr, length, |entry| {
            entry.contains(Entry::from(AllocFlags::USER_MEMORY) | Entry::PRESENT)
        })
    }

    /// check is a user ptr with len is valid and READ WRITE
    pub fn check_user_mut_ptr_with_len<T>(&self, ptr: *mut T, length: usize) -> Result<()> {
        self.check_user_ptr_predicate_with_len(ptr, length, |entry| {
            entry
                .contains(Entry::from(AllocFlags::USER_MEMORY) | Entry::READ_WRITE | Entry::PRESENT)
        })
    }
    /// Creates a slice of T, from `ptr`, of `elem_number` elements.
    /// It checks against the Bullshitship of the ptr, asserting it's a valid userland pointer.
    ///
    /// We need to specify some lifetime that is not related to self, as of lifetime ellision,
    /// in order to make the compiler understand that ultimately the slice returned is not
    /// some form of borrow of self.
    pub fn make_checked_slice<'unbound, T>(
        &self,
        ptr: *const T,
        elem_number: usize,
    ) -> Result<&'unbound [T]> {
        self.check_user_ptr_with_len(ptr, elem_number)?;
        Ok(unsafe { core::slice::from_raw_parts(ptr, elem_number) })
    }

    pub fn make_checked_str<'unbound>(&self, ptr: *const c_char) -> Result<&'unbound str> {
        let mut string_len = 0;

        let mut curr_ptr = ptr;
        loop {
            // Check if the pointer exists in address space
            self.check_user_ptr::<c_char>(curr_ptr)?;
            // Set the remaining length, relative to page size
            let limit = PAGE_SIZE - ((curr_ptr as usize) & PAGE_SIZE_MASK);
            let res = safe_strlen(curr_ptr, limit);
            if let Some(len) = res {
                // In case of success, set the final string_len and break
                string_len += len;
                break;
            } else {
                // In case of failure, advance string_len & curr_ptr
                string_len += limit;
                curr_ptr = (curr_ptr as usize + limit) as _;
            }
        }
        let slice = unsafe { core::slice::from_raw_parts(ptr as *const u8, string_len) };
        Ok(core::str::from_utf8(slice).map_err(|_e| MemoryError::BadAddr)?)
    }
    /// check is a user ptr is valid and READ WRITE
    pub fn check_user_ptr<T>(&self, ptr: *const T) -> Result<()> {
        self.check_user_ptr_predicate(ptr, |entry| {
            entry
                .contains(Entry::from(AllocFlags::USER_MEMORY) | Entry::READ_WRITE | Entry::PRESENT)
        })
    }

    /// create a safe ref from a raw pointer
    pub fn make_checked_ref<'unbound, T>(&self, ptr: *const T) -> Result<&'unbound T> {
        self.check_user_ptr_predicate(ptr, |entry| {
            entry.contains(Entry::from(AllocFlags::USER_MEMORY) | Entry::PRESENT)
        })?;
        unsafe { Ok(&*ptr) }
    }

    /// create a safe mut ref from a raw mut pointer
    pub fn make_checked_ref_mut<'unbound, T>(&self, ptr: *mut T) -> Result<&'unbound mut T> {
        self.check_user_ptr_predicate(ptr, |entry| {
            entry
                .contains(Entry::from(AllocFlags::USER_MEMORY) | Entry::READ_WRITE | Entry::PRESENT)
        })?;
        unsafe { Ok(&mut *ptr) }
    }

    /// Creates a mutable slice of T, from `ptr`, of `elem_number` elements.
    /// It checks against the Bullshitship of the ptr, asserting it's a valid userland pointer.
    ///
    /// We need to specify some lifetime that is not related to self, as of lifetime ellision,
    /// in order to make the compiler understand that ultimately the slice returned is not
    /// some form of borrow of self..
    pub fn make_checked_mut_slice<'unbound, T>(
        &self,
        ptr: *mut T,
        elem_number: usize,
    ) -> Result<&'unbound mut [T]> {
        self.check_user_mut_ptr_with_len(ptr, elem_number)?;
        Ok(unsafe { core::slice::from_raw_parts_mut(ptr, elem_number) })
    }

    pub fn make_checked_cstring(&self, s: *const c_char) -> Result<CString> {
        let s = self.make_checked_str(s)?;
        let mut v: Vec<c_char> = try_vec![0 as c_char; s.len() + 1]?;

        unsafe {
            (v.as_mut_ptr() as *mut u8).copy_from(s.as_ptr() as _, s.len());
        }
        Ok(CString(v))
    }

    pub fn make_checked_cstring_array(&self, s: *const *const c_char) -> Result<CStringArray> {
        // tips: Constructs a new, empty Vec<T>. The vector will not allocate until elements are pushed onto it.
        let mut c_pointer: Vec<*const c_char> = Vec::new();
        let mut owned_content: Vec<CString> = Vec::new();

        // Direct NULL pointer case
        if s != 0x0 as _ {
            unsafe {
                let mut curr_ptr = s;
                let pointer_size = core::mem::size_of::<*const char>();
                loop {
                    // Check if the pointer exists in address space
                    self.check_user_ptr::<*const c_char>(curr_ptr)?;
                    // Set the remaining length, relative to page size
                    let limit = PAGE_SIZE - (curr_ptr as usize) % PAGE_SIZE;

                    let mut i = 0;
                    while i != limit && *(curr_ptr.add(i / pointer_size)) != core::ptr::null() {
                        let string: CString =
                            self.make_checked_cstring(*(curr_ptr.add(i / pointer_size)))?;
                        c_pointer.try_push(string.as_ptr())?;
                        owned_content.try_push(string)?;
                        i += pointer_size;
                    }
                    if i != limit {
                        break;
                    }
                    curr_ptr = (curr_ptr as usize + limit) as _;
                }
            }
        }

        // nullptr to terminate the array
        c_pointer.try_push(0x0 as _)?;
        Ok(CStringArray {
            c_pointer,
            owned_content,
        })
    }

    pub fn alloc<N>(&mut self, length: N, alloc_flags: AllocFlags) -> Result<*mut u8>
    where
        N: Into<NbrPages>,
    {
        Ok(self
            .0
            .alloc(length.into(), alloc_flags | AllocFlags::USER_MEMORY)?
            .to_addr()
            .0 as *mut u8)
    }

    pub unsafe fn context_switch(&self) {
        self.0.context_switch()
    }

    pub fn change_range_page_entry<U>(
        &mut self,
        start_page: Page<Virt>,
        nbr_pages: NbrPages,
        update: &mut U,
    ) -> Result<()>
    where
        U: FnMut(&mut Entry),
    {
        self.0
            .change_range_page_entry(start_page, nbr_pages, update)
    }

    pub fn change_flags_range_page_entry(
        &mut self,
        start_page: Page<Virt>,
        nbr_pages: NbrPages,
        flags: AllocFlags,
    ) {
        // URGENT TODO: check if range is in user_memory
        self.0.change_flags_range_page_entry(
            start_page,
            nbr_pages,
            flags | AllocFlags::USER_MEMORY,
        );
    }

    #[inline(always)]
    pub fn change_flags_page_entry(&mut self, page: Page<Virt>, flags: AllocFlags) {
        // URGENT TODO: check if range is in user_memory
        self.0
            .change_flags_page_entry(page, flags | AllocFlags::USER_MEMORY);
    }

    pub fn alloc_on(&mut self, vaddr: *mut u8, size: usize, flags: AllocFlags) -> Result<*mut u8> {
        let vaddr = Virt(vaddr as usize);
        let size =
            NbrPages::from((vaddr + size).align_next(PAGE_SIZE) - vaddr.align_prev(PAGE_SIZE));
        let page = Page::from(vaddr);
        Ok(self
            .0
            .alloc_on(page, size, flags | AllocFlags::USER_MEMORY)?
            .to_addr()
            .0 as *mut u8)
    }
    pub fn unmap_addr(&mut self, vaddr: Page<Virt>, size: NbrPages) -> Result<()> {
        self.0.unmap_addr(vaddr, size)
    }
}

/// Get the len of a C style *const c_char. Operate in a limited area
fn safe_strlen(ptr: *const c_char, limit: usize) -> Option<usize> {
    let mut i = 0;
    while i != limit && unsafe { (*ptr.add(i)) } != 0 {
        i += 1;
    }
    if i == limit {
        None
    } else {
        Some(i)
    }
}
