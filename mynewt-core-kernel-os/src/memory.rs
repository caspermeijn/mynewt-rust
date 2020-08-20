use core::alloc::{GlobalAlloc, Layout};

struct MynewtAllocator;

unsafe impl GlobalAlloc for MynewtAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { mynewt_core_kernel_os_bindgen::os_malloc(layout.size() as cty::c_uint) as *mut u8 }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { mynewt_core_kernel_os_bindgen::os_free(ptr as *mut cty::c_void) }
    }
}

#[global_allocator]
static A: MynewtAllocator = MynewtAllocator;

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
