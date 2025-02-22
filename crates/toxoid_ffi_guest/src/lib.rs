mod ffi;

use std::ffi::{CStr, CString};
use flexbuffers::{Builder, MapBuilder};

#[derive(Clone, Copy)]
pub struct ComponentType {
    id: u64
}
unsafe impl Send for ComponentType {}
unsafe impl Sync for ComponentType {}

#[derive(Clone, Copy)]
pub struct Component {
    ptr: u64,
    entity_id: u64,
    component_type_id: u64
}
unsafe impl Send for Component {}
unsafe impl Sync for Component {}

#[derive(Clone, Copy)]
pub struct Entity {
    id: u64
}

#[derive(Clone, Copy)]
pub struct Query {
    id: u64
}

#[derive(Clone, Copy)]
pub struct System {
    id: u64
}

impl ComponentType {
    pub fn new(name: &str, member_names: Vec<String>, member_types: Vec<u8>) -> Self {
        let mut builder = Builder::default();
        {
            let mut map = builder.start_map();
            map.push("name", name);
            map.push("member_names", &member_names);
            map.push("member_types", &member_types);
            map.end_map();
        }
        
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

    pub fn set_member_u8(&mut self, offset: u32, value: u8) {
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
        let mut builder = Builder::default();
        {
            let mut map = builder.start_map();
            map.end_map();
        }
        let data = builder.build();
        let id = unsafe {
            ffi::toxoid_entity_new(data.as_ptr(), data.len())
        };
        Self { id }
    }

    pub fn named(name: &str) -> Self {
        let mut builder = Builder::default();
        {
            let mut map = builder.start_map();
            map.push("name", name);
            map.end_map();
        }
        let data = builder.build();
        let id = unsafe {
            ffi::toxoid_entity_new(data.as_ptr(), data.len())
        };
        Self { id }
    }

    pub fn get_id(&self) -> u64 {
        unsafe { ffi::toxoid_entity_get_id(self.id) }
    }

    pub fn add(&mut self, component_id: u64) {
        unsafe { ffi::toxoid_entity_add(self.id, component_id) }
    }

    pub fn has(&self, component_id: u64) -> bool {
        unsafe { ffi::toxoid_entity_has(self.id, component_id) }
    }

    pub fn remove(&mut self, component_id: u64) {
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
    pub fn add_singleton<T: Component + ComponentType + 'static>() {
        let component_id = std::any::TypeId::of::<T>().as_u64();
        unsafe { ffi::toxoid_world_add_singleton(component_id) }
    }

    pub fn get_singleton<T: Component + ComponentType + 'static>() -> *mut T {
        let component_id = std::any::TypeId::of::<T>().as_u64();
        unsafe { ffi::toxoid_world_get_singleton(component_id) as *mut T }
    }

    pub fn remove_singleton<T: Component + ComponentType + 'static>() {
        let component_id = std::any::TypeId::of::<T>().as_u64();
        unsafe { ffi::toxoid_world_remove_singleton(component_id) }
    }

    pub fn add_entity(entity_id: u64) {
        unsafe { ffi::toxoid_world_add_entity(entity_id) }
    }

    pub fn remove_entity(entity_id: u64) {
        unsafe { ffi::toxoid_world_remove_entity(entity_id) }
    }
}

// Traits
pub trait ComponentTrait: Send + Sync {
    fn get_id(&self) -> u64;
}

pub trait EntityTrait: Send + Sync {
    fn get_id(&self) -> u64;
    fn add<T: ComponentTrait>(&mut self, component: T);
    fn remove<T: ComponentTrait>(&mut self, component: T);
    fn has<T: ComponentTrait>(&self, component: T) -> bool;
}

impl ComponentTrait for ComponentType {
    fn get_id(&self) -> u64 {
        self.get_id()
    }
}

impl EntityTrait for Entity {
    fn get_id(&self) -> u64 {
        self.get_id()
    }

    fn add<T: ComponentTrait>(&mut self, component: T) {
        self.add(component.get_id())
    }

    fn remove<T: ComponentTrait>(&mut self, component: T) {
        self.remove(component.get_id())
    }

    fn has<T: ComponentTrait>(&self, component: T) -> bool {
        self.has(component.get_id())
    }
}