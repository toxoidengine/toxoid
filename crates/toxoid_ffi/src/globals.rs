// Allocator
extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
// use crate::HashMap;

extern "C" {
    fn host_alloc(size: usize) -> *mut u8;
    fn host_dealloc(ptr: *mut u8, size: usize);
}

pub struct GuestAllocator;

unsafe impl GlobalAlloc for GuestAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        host_alloc(layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        host_dealloc(ptr, layout.size())
    }
}

#[global_allocator]
pub static ALLOCATOR: GuestAllocator = GuestAllocator;

// Global constants
pub const MAX_ELEMENTS: usize = 100;
