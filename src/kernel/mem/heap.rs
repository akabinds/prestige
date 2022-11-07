use alloc::alloc::{GlobalAlloc, Layout};

pub struct OSAlloc;

unsafe impl GlobalAlloc for OSAlloc {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        core::ptr::null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc should be never called")
    }
}

#[global_allocator]
static OSALLOC: OSAlloc = OSAlloc;

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation erorr: {:?}", layout)
}
