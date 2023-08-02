// use toxoid_ffi_macro::Component;
use crate::*;
use core::ffi::c_void;
use core::alloc::{GlobalAlloc, Layout};

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

pub trait IsComponent {
    fn register() -> i32;
    fn set_ptr(&mut self, ptr: *mut c_void);
    fn get_ptr(&self) -> *mut c_void;
}

pub struct Query {
    query: *mut c_void,
    iter: *mut c_void,
    indexes: [ecs_id_t; MAX_ELEMENTS],
}

impl Query {
    pub fn new(ids: &mut [ecs_id_t]) -> Self {
        unsafe {
            Query {
                query: toxoid_query_create(ids.as_mut_ptr() as *mut i32, ids.len() as i32),
                iter: core::ptr::null_mut(),
                indexes: [0; MAX_ELEMENTS],
            }
        }
    }

    pub fn iter(&mut self) -> &mut Query {
        self.iter = unsafe { toxoid_query_iter(self.query) };
        self
    }

    pub fn count(&self) -> i32 {
        unsafe { toxoid_iter_count(self.iter) }
    }

    pub fn next(&self) -> bool {
        unsafe { toxoid_query_next(self.iter) }
    }

    pub fn entities(&self) -> &[Entity] {
        unsafe { toxoid_query_entity_list(self.iter) }
    }

    // TODO: Solve alignment issue passing vector slices back and forth
    pub fn field<T: Default + IsComponent + 'static>(&self) -> (*mut *mut c_void, i32) {
        unsafe {
            let count = toxoid_iter_count(self.iter);
            let component_id = toxoid_component_cache_get(core::any::TypeId::of::<T>());
            // + 1 because of 1-based indexing for term index
            // let term_index = self.indexes.iter().find(|&&x| x == component_id).unwrap() + 1;
    
            // Create a new vector in the host environment.
            let vec_ptr = toxoid_create_vec();
    
            for i in 0..count {
                let layout = Layout::new::<T>();
                let component_ptr = ALLOCATOR.alloc(layout) as *mut T;
                if component_ptr.is_null() {
                    // Handle allocation error, e.g., by returning or breaking the loop
                    break;
                }
            
                // let ptr = toxoid_query_field(self.iter, term_index, count as u32, i as u32);
                let ptr = toxoid_query_field(self.iter, 1, count as u32, i as u32);
                
                // Use ptr::write to write T::default() to the allocated memory.
                component_ptr.write(T::default());
                (*component_ptr).set_ptr(ptr as *mut c_void);  // Assuming T has a set_ptr method
               
                // Push the component to the vector in the host environment.
                toxoid_vec_push(vec_ptr, component_ptr as *mut _ as *mut c_void);
            }

            let (field_ptr, count) = toxoid_vec_as_slice(vec_ptr);

            let field = core::slice::from_raw_parts(*field_ptr as *mut T, count as usize);
            for i in 0..count {
                print_i32(field[i as usize].get_ptr() as i32);
            }

            toxoid_vec_as_slice(vec_ptr)
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Entity {
    id: ecs_id_t,
}

impl Entity {
    pub fn new() -> Self {
        unsafe {
            Entity {
                id: toxoid_entity_create(),
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

    pub fn get_component<T: Default + IsComponent + 'static>(&self) -> T {
        unsafe {
            let mut component = T::default();
            let component_id = toxoid_component_cache_get(core::any::TypeId::of::<T>());
            let ptr = toxoid_entity_get_component(self.id as u32, component_id as u32);
            component.set_ptr(ptr);
            component
        }
    }
}

pub fn register_tag(name: &str) -> ecs_entity_t {
    unsafe { toxoid_register_tag(name.as_bytes().as_ptr() as *const i8, name.len()) }
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
            c_member_types.len() as u32,
        )
    }
}
