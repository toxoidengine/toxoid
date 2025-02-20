mod ffi;

use std::ffi::{CStr, CString};
use flexbuffers::Builder;

pub struct ComponentType {
    id: u64
}

pub struct Component {
    ptr: u64,
    entity_id: u64,
    component_type_id: u64
}

pub struct Entity {
    id: u64
}

pub struct Query {
    id: u64
}

pub struct System {
    id: u64
}

impl ComponentType {
    pub fn new(name: &str, member_names: Vec<String>, member_types: Vec<u8>) -> Self {
        let mut builder = Builder::default();
        builder.start_map();
        builder.push("name", name);
        // Serialize member info...
        builder.end_map();
        
        let data = builder.build();
        let id = unsafe { 
            ffi::toxoid_component_type_new(data.as_ptr(), data.len())
        };
        
        Self { id }
    }

    pub fn get_id(&self) -> u64 {
        unsafe { ffi::toxoid_component_type_get_id(self.id) }
    }
}

impl Component {
    pub fn new(ptr: u64, entity_id: u64, component_type_id: u64) -> Self {
        let ptr = unsafe {
            ffi::toxoid_component_new(ptr, entity_id, component_type_id)
        };
        Self {
            ptr,
            entity_id,
            component_type_id
        }
    }

    pub fn set_member_u8(&self, offset: u32, value: u8) {
        unsafe {
            ffi::toxoid_component_set_member_u8(self.ptr, offset, value)
        }
    }

    pub fn get_member_u8(&self, offset: u32) -> u8 {
        unsafe {
            ffi::toxoid_component_get_member_u8(self.ptr, offset)
        }
    }

    // Add other member methods...
}

impl Entity {
    pub fn new() -> Self {
        let desc_data = Builder::default().build();
        let id = unsafe {
            ffi::toxoid_entity_new(desc_data.as_ptr(), desc_data.len())
        };
        Self { id }
    }

    pub fn named(name: &str) -> Self {
        let mut builder = Builder::default();
        builder.start_map();
        builder.push("name", name);
        builder.end_map();
        
        let data = builder.build();
        let id = unsafe {
            ffi::toxoid_entity_new(data.as_ptr(), data.len())
        };
        Self { id }
    }

    pub fn get_id(&self) -> u64 {
        unsafe { ffi::toxoid_entity_get_id(self.id) }
    }

    pub fn add<T: ComponentType>(&mut self, component_id: u64) {
        unsafe { ffi::toxoid_entity_add(self.id, component_id) }
    }

    pub fn has<T: ComponentType>(&self, component_id: u64) -> bool {
        unsafe { ffi::toxoid_entity_has(self.id, component_id) }
    }

    pub fn remove<T: ComponentType>(&mut self, component_id: u64) {
        unsafe { ffi::toxoid_entity_remove(self.id, component_id) }
    }
}

impl Query {
    pub fn new(expr: &str) -> Self {
        let c_expr = CString::new(expr).unwrap();
        let id = unsafe { ffi::toxoid_query_new(c_expr.as_ptr()) };
        Self { id }
    }

    pub fn build(&mut self) {
        unsafe { ffi::toxoid_query_build(self.id) }
    }

    pub fn next(&mut self) -> bool {
        unsafe { ffi::toxoid_query_next(self.id) }
    }

    pub fn count(&self) -> i32 {
        unsafe { ffi::toxoid_query_count(self.id) }
    }

    pub fn entities(&self) -> Vec<Entity> {
        let mut buffer = Vec::with_capacity(32);
        let count = unsafe {
            ffi::toxoid_query_entities(self.id, buffer.as_mut_ptr(), buffer.capacity())
        };
        unsafe { buffer.set_len(count) };
        
        buffer.into_iter()
            .map(|id| Entity { id })
            .collect()
    }
}

pub struct World;

impl World {
    pub fn add_singleton(component_id: u64) {
        unsafe { ffi::toxoid_world_add_singleton(component_id) }
    }

    pub fn get_singleton(component_id: u64) -> u64 {
        unsafe { ffi::toxoid_world_get_singleton(component_id) }
    }

    pub fn remove_singleton(component_id: u64) {
        unsafe { ffi::toxoid_world_remove_singleton(component_id) }
    }

    pub fn add_entity(entity_id: u64) {
        unsafe { ffi::toxoid_world_add_entity(entity_id) }
    }

    pub fn remove_entity(entity_id: u64) {
        unsafe { ffi::toxoid_world_remove_entity(entity_id) }
    }
}