//! Rust Global Alloc stuff

use kernel_modules::ForeignAllocMethods;

use core::alloc::{GlobalAlloc, Layout};

/// Main GlobalAlloc structure
pub struct RustGlobalAlloc {
    tools: Option<ForeignAllocMethods>,
}

impl RustGlobalAlloc {
    pub const fn new() -> Self {
        Self { tools: None }
    }

    pub fn set_methods(&mut self, alloc_methods: ForeignAllocMethods) {
        self.tools = Some(alloc_methods);
    }
}

unsafe impl GlobalAlloc for RustGlobalAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        assert!(layout.align() <= 16);
        (self.tools.as_ref().expect("no alloc set").kmalloc)(layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        assert!(layout.align() <= 16);
        (self.tools.as_ref().expect("no alloc set").kfree)(ptr)
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        assert!(layout.align() <= 16);
        (self.tools.as_ref().expect("no alloc set").kcalloc)(1, layout.size())
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        assert!(layout.align() <= 16);
        (self.tools.as_ref().expect("no alloc set").krealloc)(ptr, new_size)
    }
}

#[alloc_error_handler]
#[cfg(not(test))]
fn out_of_memory(_: core::alloc::Layout) -> ! {
    panic!("Out of memory: Failed to allocate a rust data structure");
}
