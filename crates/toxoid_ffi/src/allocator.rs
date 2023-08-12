#![allow(improper_ctypes)]
#![allow(improper_ctypes_definitions)]

use std::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;

extern "C" {
    pub fn malloc(size: usize) -> *mut c_void;
    pub fn free(p: *mut c_void);
}

struct HostAllocator;

unsafe impl GlobalAlloc for HostAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr as *mut c_void)
    }
}

#[no_mangle]
pub unsafe extern "C" fn host_alloc(layout: Layout) -> *mut u8 {
    let allocator = HostAllocator;
    allocator.alloc(layout)
}

#[no_mangle]
pub unsafe extern "C" fn host_dealloc(ptr: *mut u8, layout: Layout) {
    let allocator = HostAllocator;
    allocator.dealloc(ptr, layout)
}