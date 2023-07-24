use toxoid_ffi_macro::Component;
use core::ffi::c_void;
use crate::*;

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

    pub fn get_component<T: Default + IsComponent + 'static>(&self) -> T {
        unsafe {
            let test = core::any::TypeId::of::<T>();
            let mut map = HashMap::new();
            map.insert(test, "Hello TYPEID!");
            let value = map.get(&test).unwrap();
            print_string(value);

            let mut component = T::default();
            // let ptr = toxoid_entity_get_component(self.id as u32, T::register() as u32);
            let ptr = toxoid_entity_get_component(self.id as u32, 0 as u32);
            component.set_ptr(ptr);
            component
        }
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