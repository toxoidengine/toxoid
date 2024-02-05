#![allow(improper_ctypes)]
#![allow(improper_ctypes_definitions)]

use std::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;

extern "C" {
    #[cfg(target_os = "linux")]
    fn posix_memalign(memptr: *mut *mut c_void, alignment: usize, size: usize) -> i32;
    #[cfg(target_os = "windows")] 
    fn _aligned_malloc(size: usize, alignment: usize) -> *mut c_void;
    #[cfg(target_os = "windows")] 
    fn _aligned_free(ptr: *mut c_void);
    pub fn malloc(size: usize) -> *mut c_void;
    pub fn free(p: *mut c_void);
}

struct HostAllocator;

/* 
    malloc does not guarantee that the returned pointer will be suitably aligned for all types. The Layout passed to alloc and dealloc contains an alignment field, which you should respect.

    You can use posix_memalign instead of malloc to ensure the returned memory is properly aligned. However, this function is not available on all platforms. On Windows, you can use _aligned_malloc and _aligned_free instead. On Emscripten, you can use malloc directly, as it guarantees that the returned pointer is aligned to 16 bytes.
*/
unsafe impl GlobalAlloc for HostAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        #[cfg(target_os = "emscripten")] {
            malloc(layout.size()) as *mut u8
        }   

        #[cfg(target_os = "linux")] {
            let mut memptr: *mut c_void = std::ptr::null_mut();
            let result = posix_memalign(&mut memptr, layout.align(), layout.size());
            if result != 0 {
                std::ptr::null_mut()
            } else {
                memptr as *mut u8
            }
        }

        #[cfg(target_os = "windows")] {
            _aligned_malloc(layout.size(), layout.align()) as *mut u8
        }
    }

    #[allow(unused_variables)]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        #[cfg(all(target_os = "emscripten", target_os = "linux"))] {
            free(ptr as *mut c_void)
        }

        #[cfg(target_os = "windows")]
        _aligned_free(ptr as *mut c_void)
    }
}

#[no_mangle]
pub unsafe extern "C" fn host_alloc(layout: Layout) -> *mut u8 {
    HostAllocator.alloc(layout)
}

#[no_mangle]
pub unsafe extern "C" fn host_dealloc(ptr: *mut u8, layout: Layout) {
    HostAllocator.dealloc(ptr, layout)
}