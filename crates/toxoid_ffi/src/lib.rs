#![allow(non_snake_case)]
extern crate once_cell;
extern crate flexbuffers;
extern crate toxoid_host;

use flexbuffers::Reader;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::ffi::{c_void, CStr, CString};
use std::os::raw::c_char;
use toxoid_host::bindings::exports::toxoid::engine::ecs::{
    EntityDesc, GuestEntity, GuestQuery, GuestSystem, GuestObserver,
    QueryDesc, SystemDesc, ObserverDesc, Guest as WorldGuest
};
use toxoid_host::{
    Component as ToxoidComponent, 
    ComponentType as ToxoidComponentType,
    Entity as ToxoidEntity, 
    Query as ToxoidQuery, 
    System as ToxoidSystem,
    Observer as ToxoidObserver, 
    Iter as ToxoidIter,
    ToxoidApi
};

// FFI Struct Definitions
#[repr(C)]
#[derive(Clone)]
pub struct Component {
    id: u32,
    ptr: u64,
    entity_id: u64,
    component_type_id: u64
}

#[repr(C)]
#[derive(Clone)]
pub struct Entity {
    id: u32,
    entity_id: u64
}

#[repr(C)]
#[derive(Clone)]
pub struct Query {
    id: u32,
    query: std::sync::Arc<std::sync::Mutex<ToxoidQuery>>,
}
unsafe impl Send for Query {}
unsafe impl Sync for Query {}

#[repr(C)]
#[derive(Clone)]
pub struct System {
    id: u32,
    system_id: u64
}

#[repr(C)]
#[derive(Clone)]
pub struct Observer {
    id: u32,
    observer_id: u64
}

// Global registries now come after struct definitions
static COMPONENTS: Lazy<Mutex<HashMap<u32, Component>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static ENTITIES: Lazy<Mutex<HashMap<u32, Entity>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static SYSTEMS: Lazy<Mutex<HashMap<u32, System>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static QUERIES: Lazy<Mutex<HashMap<u32, Query>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static OBSERVERS: Lazy<Mutex<HashMap<u32, Observer>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static CALLBACKS: Lazy<Mutex<HashMap<u32, Box<dyn Fn(*const c_void) + Send + Sync>>>> = 
    Lazy::new(|| Mutex::new(HashMap::new()));

// Counter for generating unique IDs
static NEXT_ID: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(1));

fn get_next_id() -> u32 {
    let mut id = NEXT_ID.lock().unwrap();
    let current = *id;
    *id += 1;
    current
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
    COMPONENTS.lock().unwrap().insert(id, component.as_ref().clone());
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
    let root = reader.as_map();
    
    let name = root.idx("name").as_str().to_string();
    let add = if let v = root.idx("add").as_vector() {
        let mut ids = Vec::new();
        for i in 0..v.len() {
            if let val = v.idx(i).as_u64() {
                ids.push(val);
            }
        }
        Some(ids)
    } else {
        None
    };
    let prefab = root.idx("prefab").as_bool();

    let entity = ToxoidEntity::new(EntityDesc {
        name: Some(name),
        add,
        prefab
    }, None);

    let id = get_next_id();
    let entity_wrapper = Box::new(Entity {
        id,
        entity_id: entity.get_id()
    });
    
    ENTITIES.lock().unwrap().insert(id, (*entity_wrapper).clone());
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
            ToxoidApi::add_entity(component_id);
        }
    }
}

#[no_mangle]
pub extern "C" fn toxoid_entity_remove(entity: *const Entity, component_id: u64) {
    unsafe {
        if let Some(entity) = ENTITIES.lock().unwrap().get(&(*entity).id) {
            let mut api_entity = ToxoidEntity::new(EntityDesc {
                name: None,
                add: None,
                prefab: false
            }, None);
            api_entity.remove(component_id);
        }
    }
}

#[no_mangle]
pub extern "C" fn toxoid_entity_has(entity: *const Entity, component_id: u64) -> bool {
    unsafe {
        if let Some(entity) = ENTITIES.lock().unwrap().get(&(*entity).id) {
            let api_entity = ToxoidEntity::from_id(entity.entity_id);
            // ecs_has_id(WORLD.0, entity.entity_id, component_id)
            true
        } else {
            false
        }
    }
}

// Query FFI functions
#[no_mangle]
pub extern "C" fn toxoid_query_new(expr: *const c_char) -> *mut Query {
    let expr_str = unsafe { CStr::from_ptr(expr).to_str().unwrap() };
    let query = ToxoidQuery::new(QueryDesc {
        expr: expr_str.to_string()
    });
    let id = get_next_id();
    let query_wrapper = Box::new(Query {
        id,
        query: std::sync::Arc::new(std::sync::Mutex::new(query)),
    });
    
    QUERIES.lock().unwrap().insert(id, (*query_wrapper).clone());
    Box::into_raw(query_wrapper)
}

#[no_mangle]
pub extern "C" fn toxoid_query_build(query: *mut Query) {
    unsafe {
        if let Some(query) = QUERIES.lock().unwrap().get_mut(&(*query).id) {
            query.query.lock().unwrap().build();
        }
    }
}

#[no_mangle]
pub extern "C" fn toxoid_query_next(query: *const Query) -> bool {
    unsafe {
        if let Some(query) = QUERIES.lock().unwrap().get(&(*query).id) {
            query.query.lock().unwrap().next()
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
    
    let callback_id = get_next_id() as u64;
    
    let callback_box = Box::new(move |_: *const c_void| {
        callback(std::ptr::null());
    }) as Box<dyn Fn(*const c_void) + Send + Sync>;
    
    CALLBACKS.lock().unwrap().insert(callback_id as u32, callback_box);

    let system = ToxoidSystem::new(SystemDesc {
        name: None,
        callback: callback_id,
        query_desc: QueryDesc { expr: "".to_string() },
        is_guest: false,
        tick_rate: None
    });

    let id = get_next_id();
    let system_wrapper = Box::new(System {
        id,
        system_id: system.get_id()
    });
    SYSTEMS.lock().unwrap().insert(id, *(system_wrapper).clone());
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
    
    let callback_id = get_next_id() as u64;
    let callback_box = Box::new(move |data: *const c_void| {
        callback(data);
    });
    
    CALLBACKS.lock().unwrap().insert(callback_id as u32, callback_box);

    let observer = ToxoidObserver::new(ObserverDesc {
        name: None,
        query_desc: QueryDesc { expr: "".to_string() },
        events: vec![],
        callback: callback_id,
        is_guest: false
    });

    let id = get_next_id();
    let observer_wrapper = Box::new(Observer {
        id,
        observer_id: unsafe { *observer.entity.borrow() } // Access entity ID directly from RefCell
    });
    
    OBSERVERS.lock().unwrap().insert(id, (*observer_wrapper).clone());
    Box::into_raw(observer_wrapper)
}

// World FFI functions
#[no_mangle]
pub extern "C" fn toxoid_world_add_singleton(component_id: u64) {
    ToxoidApi::add_singleton(component_id);
}

#[no_mangle]
pub extern "C" fn toxoid_world_get_singleton(component_id: u64) -> u64 {
    ToxoidApi::get_singleton(component_id)
}

#[no_mangle]
pub extern "C" fn toxoid_world_remove_singleton(component_id: u64) {
    ToxoidApi::remove_singleton(component_id);
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