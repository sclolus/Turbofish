use core::alloc::{GlobalAlloc, Layout};

pub struct RustGlobalAlloc;

unsafe impl GlobalAlloc for RustGlobalAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        assert!(layout.align() <= 16);
        malloc(layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        assert!(layout.align() <= 16);
        free(ptr)
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        assert!(layout.align() <= 16);
        calloc(layout.size())
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        assert!(layout.align() <= 16);
        realloc(ptr, new_size)
    }
}

#[alloc_error_handler]
#[cfg(not(test))]
fn out_of_memory(_: core::alloc::Layout) -> ! {
    panic!("Out of memory: Failed to allocate a rust data structure");
}

extern "C" {
    fn malloc(len: usize) -> *mut u8;
    fn calloc(len: usize) -> *mut u8;
    fn realloc(addr: *mut u8, new_size: usize) -> *mut u8;
    fn free(addr: *mut u8);
}
