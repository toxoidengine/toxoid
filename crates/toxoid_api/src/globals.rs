#![allow(improper_ctypes)]
// Allocator
extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
// use crate::HashMap;

extern "C" {
    fn host_alloc(layout: Layout) -> *mut u8;
    fn host_dealloc(ptr: *mut u8, layout: Layout);
}

pub struct GuestAllocator;

unsafe impl GlobalAlloc for GuestAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        host_alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        host_dealloc(ptr, layout)
    }
}

#[global_allocator]
pub static ALLOCATOR: GuestAllocator = GuestAllocator;
