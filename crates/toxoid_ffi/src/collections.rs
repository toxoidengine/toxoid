use core::mem;
use core::ptr::NonNull;
use core::alloc::{GlobalAlloc, Layout};
use crate::globals::*;

pub struct Vec<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.len == self.cap {
            self.grow();
        }

        unsafe {
            self.ptr.as_ptr().add(self.len).write(item);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;

        unsafe { Some(self.ptr.as_ptr().add(self.len).read()) }
    }
    // We calculate the old layout just before deallocating the old memory block, so we can provide the correct Layout to the dealloc method.
    fn grow(&mut self) {
        let new_cap = if self.cap == 0 { 1 } else { self.cap * 2 };
        let new_size = new_cap * mem::size_of::<T>();
        let new_layout = Layout::from_size_align(new_size, mem::align_of::<T>()).unwrap();
        let new_ptr = unsafe { ALLOCATOR.alloc(new_layout) as *mut T };
    
        if !self.ptr.as_ptr().is_null() {
            unsafe {
                // Copy old data to new space and deallocate old space.
                new_ptr.copy_from_nonoverlapping(self.ptr.as_ptr(), self.len);
                let old_layout = Layout::from_size_align(self.cap * mem::size_of::<T>(), mem::align_of::<T>()).unwrap();
                ALLOCATOR.dealloc(self.ptr.as_ptr() as *mut u8, old_layout);
            }
        }
    
        self.ptr = unsafe { NonNull::new_unchecked(new_ptr) };
        self.cap = new_cap;
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            ptr: self.ptr.as_ptr(),
            len: self.len,
            index: 0,
        }
    }

    pub fn retain<F: FnMut(&T) -> bool>(&mut self, mut f: F) {
        let mut i = 0;
        let mut len = self.len;

        while i != len {
            if !f(unsafe { &*self.ptr.as_ptr().add(i) }) {
                len -= 1;
                if i != len {
                    unsafe {
                        core::ptr::copy(self.ptr.as_ptr().add(i + 1), self.ptr.as_ptr().add(i), len - i);
                    }
                }
            } else {
                i += 1;
            }
        }
        self.len = len;
    }
}

pub struct Iter<T> {
    ptr: *mut T,
    len: usize,
    index: usize,
}

impl<T> Iterator for Iter<T> {
    type Item = *mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let result = unsafe { self.ptr.add(self.index) };
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

#[derive(Clone)]
struct KeyValuePair<K, V> {
    key: K,
    value: V,
}

pub struct HashMap<K: PartialEq + Clone, V: Clone> {
    elements: Vec<KeyValuePair<K, V>>,
}

impl<K: PartialEq + Clone, V: Clone> HashMap<K, V> {
    pub fn new() -> Self {
        HashMap { elements: Vec::new() }
    }

    pub fn insert(&mut self, key: K, value: V) {
        // remove any existing value for this key
        self.elements.retain(|kvp| kvp.key != key);

        // insert the new key-value pair
        self.elements.push(KeyValuePair { key, value });
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.elements.iter().filter_map(|kvp| {
            let kvp_ref = unsafe { &*kvp };
            if kvp_ref.key == *key {
                Some(&kvp_ref.value)
            } else {
                None
            }
        }).next()
    }
}