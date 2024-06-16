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

#[cfg(all(target_arch="wasm32", target_os="emscripten"))]
#[global_allocator]
pub static ALLOCATOR: GuestAllocator = GuestAllocator;

#[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))]
#[global_allocator]
pub static ALLOCATOR: std::alloc::System = std::alloc::System;


#[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))]
pub extern "C" fn allocate(size: usize) -> *mut u8 {
    let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();
    unsafe { std::alloc::alloc(layout) }
}

#[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))]
pub extern "C" fn deallocate(ptr: *mut u8, size: usize) {
    let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();
    unsafe { std::alloc::dealloc(ptr, layout) }
}