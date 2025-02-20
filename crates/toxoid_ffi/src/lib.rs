#![allow(non_snake_case)]

use flexbuffers::{Builder, Reader};
use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::ffi::{c_void, CStr, CString};
use std::os::raw::{c_char, c_int};

// Store resources in global maps with unique IDs
static COMPONENTS: Lazy<Mutex<HashMap<u32, Component>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static ENTITIES: Lazy<Mutex<HashMap<u32, Entity>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static SYSTEMS: Lazy<Mutex<HashMap<u32, System>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static QUERIES: Lazy<Mutex<HashMap<u32, Query>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static OBSERVERS: Lazy<Mutex<HashMap<u32, Observer>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static CALLBACKS: Lazy<Mutex<HashMap<u32, Box<dyn Fn(*const c_void)>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

// Counter for generating unique IDs
static NEXT_ID: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(1));

fn get_next_id() -> u32 {
    let mut id = NEXT_ID.lock().unwrap();
    let current = *id;
    *id += 1;
    current
}

// Basic FFI structs
#[repr(C)]
pub struct Component {
    id: u32,
    ptr: u64,
    entity_id: u64,
    component_type_id: u64
}

#[repr(C)]
pub struct Entity {
    id: u32,
    entity_id: u64
}

#[repr(C)]
pub struct System {
    id: u32,
    system_id: u64
}

#[repr(C)]
pub struct Query {
    id: u32,
    query: toxoid_api::Query
}

#[repr(C)]
pub struct Observer {
    id: u32,
    observer: toxoid_api::Observer
}

// Component FFI functions
#[no_mangle]
pub extern "C" fn toxoid_component_new(ptr: u64, entity_id: u64, component_type_id: u64) -> *mut Component {
    let id = get_next_id();
    let component = Box::new(Component {
        id,
        ptr,
        entity_id,
        component_type_id
    });
    COMPONENTS.lock().unwrap().insert(id, *component);
    Box::into_raw(component)
}

#[no_mangle]
pub extern "C" fn toxoid_component_set_member_u8(component: *mut Component, offset: u32, value: u8) {
    unsafe {
        let component = &*component;
        let ptr = component.ptr as *mut u8;
        *ptr.add(offset as usize) = value;
    }
}

#[no_mangle]
pub extern "C" fn toxoid_component_get_member_u8(component: *const Component, offset: u32) -> u8 {
    unsafe {
        let component = &*component;
        let ptr = component.ptr as *const u8;
        *ptr.add(offset as usize)
    }
}

// Add other member get/set methods...

// Entity FFI functions
#[no_mangle]
pub extern "C" fn toxoid_entity_new(desc_data: *const u8, desc_len: usize) -> *mut Entity {
    let desc_slice = unsafe { std::slice::from_raw_parts(desc_data, desc_len) };
    let reader = Reader::get_root(desc_slice).unwrap();
    
    let name = reader.get_str("name").ok();
    let add = reader.get_vector("add").ok().map(|v| {
        let mut ids = Vec::new();
        for i in 0..v.length() {
            ids.push(v.get(i).as_u64().unwrap());
        }
        ids
    });
    let prefab = reader.get_bool("prefab").unwrap_or(false);

    let desc = toxoid_api::EntityDesc {
        name: name.map(|s| s.to_string()),
        add,
        prefab
    };

    let entity = toxoid_api::Entity::new(Some(desc));
    let id = get_next_id();
    
    let entity_wrapper = Box::new(Entity {
        id,
        entity_id: entity.get_id()
    });
    
    ENTITIES.lock().unwrap().insert(id, *entity_wrapper);
    Box::into_raw(entity_wrapper)
}

#[no_mangle]
pub extern "C" fn toxoid_entity_get_id(entity: *const Entity) -> u64 {
    unsafe { (*entity).entity_id }
}

#[no_mangle]
pub extern "C" fn toxoid_entity_add(entity: *const Entity, component_id: u64) {
    unsafe {
        if let Some(entity) = ENTITIES.lock().unwrap().get(&(*entity).id) {
            toxoid_api::World::add_entity(component_id);
        }
    }
}

#[no_mangle]
pub extern "C" fn toxoid_entity_remove(entity: *const Entity, component_id: u64) {
    unsafe {
        if let Some(entity) = ENTITIES.lock().unwrap().get(&(*entity).id) {
            let mut api_entity = toxoid_api::Entity::from_id(entity.entity_id);
            api_entity.remove(component_id);
        }
    }
}

#[no_mangle]
pub extern "C" fn toxoid_entity_has(entity: *const Entity, component_id: u64) -> bool {
    unsafe {
        if let Some(entity) = ENTITIES.lock().unwrap().get(&(*entity).id) {
            let api_entity = toxoid_api::Entity::from_id(entity.entity_id);
            api_entity.has(component_id)
        } else {
            false
        }
    }
}

// Query FFI functions
#[no_mangle]
pub extern "C" fn toxoid_query_new(expr: *const c_char) -> *mut Query {
    let expr_str = unsafe { CStr::from_ptr(expr).to_string_lossy().into_owned() };
    let query = toxoid_api::Query::dsl(&expr_str);
    let id = get_next_id();
    let query_wrapper = Box::new(Query { id, query });
    QUERIES.lock().unwrap().insert(id, *query_wrapper);
    Box::into_raw(query_wrapper)
}

#[no_mangle]
pub extern "C" fn toxoid_query_build(query: *mut Query) {
    unsafe {
        if let Some(query) = QUERIES.lock().unwrap().get_mut(&(*query).id) {
            query.query.build();
        }
    }
}

#[no_mangle]
pub extern "C" fn toxoid_query_next(query: *const Query) -> bool {
    unsafe {
        if let Some(query) = QUERIES.lock().unwrap().get(&(*query).id) {
            query.query.next()
        } else {
            false
        }
    }
}

// System FFI functions
#[no_mangle]
pub extern "C" fn toxoid_system_new(
    desc_data: *const u8, 
    desc_len: usize,
    callback: extern "C" fn(*const c_void)
) -> *mut System {
    let desc_slice = unsafe { std::slice::from_raw_parts(desc_data, desc_len) };
    let reader = Reader::get_root(desc_slice).unwrap();
    
    let callback_id = get_next_id();
    let callback_box = Box::new(move |data: *const c_void| {
        callback(data);
    });
    
    CALLBACKS.lock().unwrap().insert(callback_id, callback_box);

    let system = toxoid_api::System::new(
        None,
        move |iter| {
            if let Some(callback) = CALLBACKS.lock().unwrap().get(&callback_id) {
                // Convert iter to C data structure
                let iter_data = std::ptr::null(); // TODO: Implement conversion
                callback(iter_data);
            }
        }
    );

    let id = get_next_id();
    let system_wrapper = Box::new(System {
        id,
        system_id: system.get_id()
    });
    
    SYSTEMS.lock().unwrap().insert(id, *system_wrapper);
    Box::into_raw(system_wrapper)
}

// Observer FFI functions
#[no_mangle]
pub extern "C" fn toxoid_observer_new(
    desc_data: *const u8,
    desc_len: usize,
    callback: extern "C" fn(*const c_void)
) -> *mut Observer {
    let desc_slice = unsafe { std::slice::from_raw_parts(desc_data, desc_len) };
    let reader = Reader::get_root(desc_slice).unwrap();
    
    let callback_id = get_next_id();
    let callback_box = Box::new(move |data: *const c_void| {
        callback(data);
    });
    
    CALLBACKS.lock().unwrap().insert(callback_id, callback_box);

    let observer = toxoid_api::Observer::new(
        None,
        move |iter| {
            if let Some(callback) = CALLBACKS.lock().unwrap().get(&callback_id) {
                let iter_data = std::ptr::null(); // TODO: Implement conversion
                callback(iter_data);
            }
        }
    );

    let id = get_next_id();
    let observer_wrapper = Box::new(Observer {
        id,
        observer
    });
    
    OBSERVERS.lock().unwrap().insert(id, *observer_wrapper);
    Box::into_raw(observer_wrapper)
}

// World FFI functions
#[no_mangle]
pub extern "C" fn toxoid_world_add_singleton(component_id: u64) {
    toxoid_api::World::add_singleton(component_id);
}

#[no_mangle]
pub extern "C" fn toxoid_world_get_singleton(component_id: u64) -> u64 {
    toxoid_api::ToxoidApi::get_singleton(component_id)
}

#[no_mangle]
pub extern "C" fn toxoid_world_remove_singleton(component_id: u64) {
    toxoid_api::World::remove_singleton(component_id);
}

// Memory management functions
#[no_mangle]
pub extern "C" fn toxoid_free_component(ptr: *mut Component) {
    unsafe {
        if !ptr.is_null() {
            let component = Box::from_raw(ptr);
            COMPONENTS.lock().unwrap().remove(&component.id);
        }
    }
}

#[no_mangle]
pub extern "C" fn toxoid_free_entity(ptr: *mut Entity) {
    unsafe {
        if !ptr.is_null() {
            let entity = Box::from_raw(ptr);
            ENTITIES.lock().unwrap().remove(&entity.id);
        }
    }
}

#[no_mangle]
pub extern "C" fn toxoid_free_system(ptr: *mut System) {
    unsafe {
        if !ptr.is_null() {
            let system = Box::from_raw(ptr);
            SYSTEMS.lock().unwrap().remove(&system.id);
        }
    }
} 