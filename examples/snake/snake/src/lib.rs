#![allow(non_camel_case_types)]
#![allow(improper_ctypes)]
// #![no_std]
// #![feature(allocator_api)]
extern crate toxoid_ffi_macro;
extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};

extern "C" {
    fn host_alloc(size: usize) -> *mut u8;
    fn host_dealloc(ptr: *mut u8, size: usize);
}

struct GuestAllocator;

unsafe impl GlobalAlloc for GuestAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        host_alloc(layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        host_dealloc(ptr, layout.size())
    }
}

#[global_allocator]
static ALLOCATOR: GuestAllocator = GuestAllocator;

use toxoid_ffi_macro::Components;
use core::ffi::c_void;

pub type ecs_id_t = i32;
pub type ecs_entity_t = ecs_id_t;
pub type c_char = i8;
pub const MAX_ELEMENTS: usize = 100;

extern "C" {
    pub fn toxoid_print_i32(v: i32);
    pub fn toxoid_print_string(v: *const i8, v_len: usize);
    pub fn toxoid_entity_get_name(id: i32);
    pub fn toxoid_register_tag(name: *const i8, name_len: usize) -> ecs_entity_t;
    pub fn toxoid_register_component(
        component_name: *const c_char,
        component_name_len: u8,
        member_names: *const *const c_char,
        member_names_count: u32,
        member_names_len: *const u8,
        member_types: *const *const u8,
        member_types_count: u32
    ) -> ecs_entity_t;
    pub fn toxoid_component_set_member_u32(component_ptr: *mut c_void, offset: u32, value: u32);
    pub fn toxoid_entity_create() -> ecs_entity_t;
    pub fn toxoid_entity_add_component(entity: u32, component: u32) -> *mut c_void;
    pub fn toxoid_entity_add_tag(entity: u32, tag: u32);
    pub fn toxoid_query_create(ids: *mut i32, components_count: i32) -> *mut c_void;
    pub fn toxoid_query_iter(query: *mut c_void) -> *mut c_void;
    pub fn toxoid_query_next(iter: *mut c_void) -> bool;
    pub fn toxoid_query_count(iter: *mut c_void) -> i32;
    pub fn toxoid_query_field(iter: *mut c_void, term_index: i32, count: u32, index: u32) -> *const c_void;
    pub fn toxoid_query_entity_list(iter: *mut c_void) -> &'static [Entity];
    pub fn toxoid_iter_count(iter: *mut c_void) -> i32;
}

pub struct Query {
    query: *mut c_void,
    iter: *mut c_void,
    _indexes: [ecs_id_t; MAX_ELEMENTS],
}

impl Query {
    pub fn new(ids: &mut [ecs_id_t]) -> Self {
        unsafe {
            Query {
                query: toxoid_query_create(ids.as_mut_ptr() as *mut i32, ids.len() as i32),
                iter: core::ptr::null_mut(),
                _indexes: [0; MAX_ELEMENTS],
            }
        }
        
    }

    pub fn iter(&mut self) -> &mut Query {
        self.iter = unsafe { 
            toxoid_query_iter(self.query)
        };
        self
    }

    pub fn count(&self) -> i32 {
        unsafe {
            toxoid_iter_count(self.iter)
        }
    }

    pub fn next(&self) -> bool {
        unsafe {
            toxoid_query_next(self.iter)
        }
    }

    pub fn field(&self) -> *const c_void {
        unsafe {
            toxoid_query_field(self.iter, 0, 1, 0)
        }
    }

    pub fn entities(&self) -> &[Entity] {
        unsafe {
            toxoid_query_entity_list(self.iter)
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Entity {
    id: ecs_id_t
}

impl Entity {
    pub fn new() -> Self {
        unsafe {
            Entity {
                id: toxoid_entity_create()
            }
        }
    }

    pub fn add(&mut self, component: ecs_id_t) {
        unsafe {
            toxoid_entity_add_component(self.id as u32, component as u32);
        }
    }

    pub fn add_tag(&mut self, tag: ecs_entity_t) {
        unsafe {
            toxoid_entity_add_tag(self.id as u32, tag as u32);
        }
    }

    pub fn get_id(&self) -> ecs_entity_t {
        self.id
    }

    // pub fn get_component<T: Default>(&self) -> Option<&T> {
    //     unsafe {
    //         // let ptr = T::default();
    //         //Some(&ptr)
    //         alloc::boxed::Box::new(42)
    //         // if ptr.is_null() {
    //         //     None
    //         // } else {
    //         //     Some(&*(ptr as *const T))
    //         // }
    //     }
    // }
}

#[repr(u8)]
pub enum Type {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Bool,
    String,
    Array,
    U32Array,
    F32Array,
}

#[derive(Components)]
pub struct Position {
    x: u32,
    y: u32,
}

#[derive(Components)]
pub struct Velocity {
    dx: f32,
    dy: f32,
}

#[no_mangle]
pub unsafe extern "C" fn app_main() {
    let ptr = ALLOCATOR.alloc(Layout::from_size_align(1, 1).unwrap());
    ALLOCATOR.dealloc(ptr, Layout::from_size_align(1, 1).unwrap());

    let mut vec = Vec::new();

    for i in 1..10 {
        vec.push(i);
    }

    for item in vec.iter() {
        print_string("HELLO ITEMS!");
    }

    let mut map = HashMap::new();
    map.insert(1, "one");
    print_string(map.get(&1).unwrap());

    // Create a new tag.
    let tag = register_tag("LocalPlayer");
    // Print the name of the tag.
    toxoid_entity_get_name(tag);

    // Create a new component.
    let mut position = Position { x: 0, y: 0 };
    // Set the values of the component.
    position.set_x(77);
    position.set_y(99);
    // Print the values of the component.
    print_string("X:");
    print_i32(position.x as i32);
    print_string("Y:");
    print_i32(position.y as i32);

    // Register the component.
    let pos_id = Position::register();
    let vel_id = Velocity::register();

    // Print the name of the component.
    toxoid_entity_get_name(pos_id);
    toxoid_entity_get_name(vel_id);

    // Create a new entity.
    let mut player = Entity::new();
    // Add the component to the entity.
    player.add(pos_id);
    player.add(vel_id);
    player.add_tag(tag);

    let mut player_2 = Entity::new();
    // Add the component to the entity.
    player_2.add(pos_id);
    player_2.add(vel_id);
    player_2.add_tag(tag);

    let mut player_3 = Entity::new();
    // Add the component to the entity.
    player_3.add(pos_id);
    player_3.add(vel_id);
    player_3.add_tag(tag);

    let mut query = Query::new(&mut [pos_id, vel_id]);
    let query = query.iter();
    while query.next() {
        let _field = query.field();
        let entities = query.entities();
        for entity in entities.iter() {
            print_i32(entity.get_id());
        }
    }
}

pub fn print_i32(v: i32) {
    unsafe {
        toxoid_print_i32(v);
    }
}

pub fn print_string(v: &str) {
    unsafe {
        toxoid_print_string(v.as_bytes().as_ptr() as *const i8, v.len());
    }
}

pub fn register_tag(name: &str) -> ecs_entity_t {
    unsafe {
        toxoid_register_tag(name.as_bytes().as_ptr() as *const i8, name.len())
    }
}

pub fn register_component(name: &str, member_names: &[&str], member_types: &[u8]) -> ecs_entity_t {
    unsafe {
        let mut c_member_names: [*const c_char; MAX_ELEMENTS] = [core::ptr::null(); MAX_ELEMENTS]; 
        let mut c_member_names_len: [u8; MAX_ELEMENTS] = [0; MAX_ELEMENTS];
        for (i, &s) in member_names.iter().enumerate() {
            c_member_names[i] = s.as_ptr() as *const c_char;
            c_member_names_len[i] = s.len() as u8;
        }

        // TODO: Since this is Rust, it can probably just be the u8 value
        // instead of a pointer to the value.
        let mut c_member_types: [*const u8; MAX_ELEMENTS] = [core::ptr::null(); MAX_ELEMENTS];
        for (i, &t) in member_types.iter().enumerate() {
            c_member_types[i] = &t as *const u8;
        }

        toxoid_register_component(
            name.as_bytes().as_ptr() as *const c_char,
            name.len() as u8,
            c_member_names.as_ptr(),
            member_names.len() as u32,
            c_member_names_len.as_ptr(),
            c_member_types.as_ptr(),
            c_member_types.len() as u32
        )
    }
}

use core::mem;
use core::ptr::NonNull;

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