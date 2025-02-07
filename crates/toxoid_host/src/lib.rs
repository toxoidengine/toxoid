#![allow(non_camel_case_types)]
#![allow(warnings)]

pub mod bindings;
use bindings::exports::toxoid::engine::ecs::{EcsEntityT, GuestIter, GuestObserver, ObserverDesc, Phases, PointerT, Relationship};
use bindings::exports::toxoid::engine::ecs::{self, ComponentDesc, EntityDesc, Guest, GuestCallback, GuestComponent, GuestComponentType, GuestEntity, GuestQuery, GuestSystem, GuestPhase, GuestPipeline, QueryDesc, SystemDesc, PipelineDesc, Event};
pub use toxoid_flecs::bindings::{ecs_add_id, ecs_entity_desc_t, ecs_entity_init, ecs_fini, ecs_get_mut_id, ecs_init, ecs_iter_t, ecs_lookup, ecs_make_pair, ecs_member_t, ecs_progress, ecs_query_desc_t, ecs_query_init, ecs_query_iter, ecs_query_next, ecs_query_t, ecs_struct_desc_t, ecs_struct_init, ecs_system_desc_t, ecs_system_init, ecs_system_t, ecs_world_t, EcsDependsOn, EcsOnUpdate, ecs_set_rate, ecs_get_id, ecs_remove_id};
use toxoid_flecs::{ecs_children, ecs_children_next, ecs_delete, ecs_enable, ecs_ensure_id, ecs_field_size, ecs_field_w_size, ecs_get_name, ecs_get_parent, ecs_has_id, ecs_modified_id, ecs_new_w_id, ecs_observer_desc_t, ecs_observer_init, ecs_observer_t, ecs_pipeline_desc_t, ecs_pipeline_init, ecs_set_name, EcsChildOf, EcsInherit, EcsIsA, EcsOnInstantiate, EcsOnLoad, EcsOnStart, EcsOnStore, EcsOnValidate, EcsPhase, EcsPostLoad, EcsPostUpdate, EcsPreStore, EcsPreUpdate, EcsPrefab};
use std::ffi::CStr;
use std::{borrow::BorrowMut, mem::MaybeUninit};
use core::ffi::c_void;
use core::ffi::c_char;
use std::cell::RefCell;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::Mutex;
type ecs_entity_t = u64;

pub struct ToxoidApi;

pub struct ComponentType { 
    pub id: ecs_entity_t
}

pub struct Component {
    pub ptr: *const c_void,
    pub field_offsets: Vec<u8>,
    pub entity_added: ecs_entity_t,
    pub component_type_id: ecs_entity_t
}

pub struct Entity { 
    pub id: ecs_entity_t
}

pub struct Query {
    pub desc: RefCell<ecs_query_desc_t>,
    pub query: RefCell<ecs_query_t>,
    pub iter: RefCell<ecs_iter_t>
}

pub struct System {
    pub desc: RefCell<ecs_system_desc_t>,
    pub entity: RefCell<ecs_entity_t>,
    pub system: RefCell<ecs_system_t>,
    pub callback: RefCell<Callback>
}

pub struct Observer {
    pub desc: RefCell<ecs_observer_desc_t>,
    pub entity: RefCell<ecs_entity_t>,
    pub observer: RefCell<ecs_observer_t>,
    pub callback: RefCell<Callback>
}

pub struct Callback {
    pub handle: u64
}

pub struct Iter {
    pub ptr: *mut c_void
}

pub struct Phase {
    pub name: String,
    pub entity: RefCell<ecs_entity_t>
}

pub struct Pipeline {
    pub desc: RefCell<ecs_pipeline_desc_t>,
    pub entity: RefCell<ecs_entity_t>
}

pub struct EcsWorldPtr(pub *mut ecs_world_t);
unsafe impl Send for EcsWorldPtr {}
unsafe impl Sync for EcsWorldPtr {}

#[repr(u8)]
enum FieldType {
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
    U8List,
    U16List,
    U32List,
    U64List,
    I8List,
    I16List,
    I32List,
    I64List,
    F32List,
    F64List,
    Pointer
}

pub static mut WORLD: Lazy<EcsWorldPtr> = Lazy::new(|| 
    EcsWorldPtr(unsafe { 
        let world = ecs_init();
        // toxoid_flecs::bindings::ecs_set_threads(world, 4);
        world
    })
);

// Create a wrapper type for the pointer that implements Send
#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct ThreadSafePtr(*const c_void);
unsafe impl Send for ThreadSafePtr {}
unsafe impl Sync for ThreadSafePtr {}

// Global cache for array lengths
static ARRAY_LENGTH_CACHE: Lazy<Mutex<HashMap<(ThreadSafePtr, u32), usize>>> = 
    Lazy::new(|| Mutex::new(HashMap::new()));

// Progress the world - game loop tick
pub fn toxoid_progress(delta_time: f32) -> bool {
    unsafe { ecs_progress(WORLD.0, delta_time) }
}

// Reset the world - delete all entities
pub fn toxoid_reset() {
    unsafe {
        ecs_fini(WORLD.0);
        WORLD = Lazy::new(|| EcsWorldPtr(unsafe { ecs_init() }));
    }
}

fn map_event(event: Event) -> ecs_entity_t {
    unsafe {
        match event {
            Event::OnAdd => toxoid_flecs::EcsOnAdd,
            Event::OnRemove => toxoid_flecs::EcsOnRemove,
            Event::OnSet => toxoid_flecs::EcsOnSet,
            Event::OnDelete => toxoid_flecs::EcsOnDelete,
            Event::OnDeleteTarget => toxoid_flecs::EcsOnDeleteTarget,
            Event::OnTableCreate => toxoid_flecs::EcsOnTableCreate,
            Event::OnTableDelete => toxoid_flecs::EcsOnTableDelete,
            Event::OnTableEmpty => toxoid_flecs::EcsOnTableEmpty,
            Event::OnTableFill => toxoid_flecs::EcsOnTableFill,
            _ => 0
        }
    }
}

unsafe fn map_member_type(member_type: u8) -> ecs_entity_t {
    match member_type {
        0 => toxoid_flecs::bindings::FLECS_IDecs_u8_tID_,
        1 => toxoid_flecs::bindings::FLECS_IDecs_u16_tID_,
        2 => toxoid_flecs::bindings::FLECS_IDecs_u32_tID_,
        3 => toxoid_flecs::bindings::FLECS_IDecs_u64_tID_,
        4 => toxoid_flecs::bindings::FLECS_IDecs_i8_tID_,
        5 => toxoid_flecs::bindings::FLECS_IDecs_i16_tID_,
        6 => toxoid_flecs::bindings::FLECS_IDecs_i32_tID_,
        7 => toxoid_flecs::bindings::FLECS_IDecs_i64_tID_,
        8 => toxoid_flecs::bindings::FLECS_IDecs_f32_tID_,
        9 => toxoid_flecs::bindings::FLECS_IDecs_f64_tID_,
        10 => toxoid_flecs::bindings::FLECS_IDecs_bool_tID_,
        11 => toxoid_flecs::bindings::FLECS_IDecs_string_tID_,
        _ => toxoid_flecs::bindings::FLECS_IDecs_uptr_tID_,
    }
}

fn c_string(rust_str: &str) -> *const i8 {
    let c_string = std::ffi::CString::new(rust_str).expect("CString::new failed");
    let c_ptr = c_string.as_ptr();
    std::mem::forget(c_string); // Prevent CString from being deallocated
    c_ptr
}

impl GuestComponentType for ComponentType {
    fn new(desc: ComponentDesc) -> ComponentType {
        unsafe {
            let is_tag = desc.member_names.len() == 0 || desc.member_types.len() == 0;
            if is_tag {
                let mut ent_desc: ecs_entity_desc_t = MaybeUninit::zeroed().assume_init();
                ent_desc.name = c_string(&desc.name);
                let mut tag_entity = ecs_entity_init(WORLD.0, &ent_desc);
                // TODO: Maybe there is some Flecs compiler flag to optimize out debug
                // errors for duplicate component and tag names.
                let lookup = ecs_lookup(WORLD.0, ent_desc.name);
                if lookup == 0 {
                    tag_entity = ecs_entity_init(WORLD.0, &ent_desc);
                } else {
                    tag_entity = ecs_lookup(WORLD.0, ent_desc.name);
                }
                ComponentType {
                    id: tag_entity
                }
            } else {
                // Create component entity
                let mut ent_desc: ecs_entity_desc_t = MaybeUninit::zeroed().assume_init();
                ent_desc.name = c_string(&desc.name);
                // TODO: Maybe there is some Flecs compiler flag to optimize out debug
                // errors for duplicate component and tag names.
                let lookup = ecs_lookup(WORLD.0, ent_desc.name);
                let mut component_entity: ecs_entity_t;
                if lookup == 0 {
                    component_entity = ecs_entity_init(WORLD.0, &ent_desc);
                } else {
                    component_entity = ecs_lookup(WORLD.0, ent_desc.name);
                }

                // Create runtime component description
                let mut struct_desc: ecs_struct_desc_t = MaybeUninit::zeroed().assume_init();
                struct_desc.entity = component_entity;
                let member: ecs_member_t = MaybeUninit::zeroed().assume_init();
                struct_desc.members = [member; 32usize];
                
                // Iterate through member names
                for (index, member_name) in desc.member_names.iter().enumerate() {
                    // Create component member
                    let mut member: ecs_member_t = MaybeUninit::zeroed().assume_init();
                    member.name = member_name.as_ptr() as *const i8;
                    member.type_ = map_member_type(desc.member_types[index]);
                    struct_desc.members[index] = member;
                }

                // Initialize component
                if lookup == 0 {
                    ecs_struct_init(WORLD.0, &struct_desc);
                }

                // Return component 
                ComponentType {
                    id: component_entity
                }
            }
        }
    }

    fn get_id(&self) -> ecs_entity_t {
        self.id
    }
}

impl Component {
    // TODO: Make all functions in host that pass back pointers
    // use this style of function implementation
    // Unfortunately it goes against the WASM component model
    // generated trait. Look into this in the future.
    // But it's a better than leaking memory with boxed
    // allocations on the heap and managing their lifetimes
    // manually.
    pub fn from_ptr_host(ptr: u64) -> Component {
        Component {
            ptr: ptr as *const c_void,
            field_offsets: vec![],
            entity_added: 0,
            component_type_id: 0
        }
    }
}

impl GuestComponent for Component {
    // This is a component instance so it will need a the entity it belongs to and the component type
    // This is required for observers / events to work
    fn new(ptr: u64, entity_added: ecs_entity_t, component_type_id: ecs_entity_t) -> Component {
        Component { 
            ptr: ptr as *const c_void, 
            field_offsets: vec![], 
            entity_added, 
            component_type_id
        }
    }

    fn from_ptr(ptr: u64) -> PointerT {
        unimplemented!("Use from_ptr_host instead");
        // unsafe {
        //     Box::into_raw(Box::new(Component {
        //         ptr: ptr as *const c_void,
        //         field_offsets: vec![],
        //         entity_added: 0,
        //         component_type_id: 0
        //     })) as PointerT
        // }
    }

    fn set_member_u8(&self, offset: u32, value: u8) {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut u8;
            *member_ptr = value;
            ecs_modified_id(WORLD.0, self.entity_added, self.component_type_id);
        }
    }

    fn get_member_u8(&self, offset: u32) -> u8 {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut u8;
            let member_value: u8 = *member_ptr;
            member_value
        }
    }

    fn set_member_u16(&self, offset: u32, value: u16) {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut u16;
            *member_ptr = value;
            ecs_modified_id(WORLD.0, self.entity_added, self.component_type_id);
        }
    }

    fn get_member_u16(&self, offset: u32) -> u16 {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut u16;
            let member_value: u16 = *member_ptr;
            member_value
        }
    }

    fn set_member_u32(&self, offset: u32, value: u32) {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut u32;
            *member_ptr = value;
            ecs_modified_id(WORLD.0, self.entity_added, self.component_type_id);
        }
    }

    fn get_member_u32(&self, offset: u32) -> u32 {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut u32;
            let member_value: u32 = *member_ptr;
            member_value
        }
    }

    fn set_member_u64(&self, offset: u32, value: u64) {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut u64;
            *member_ptr = value;
            ecs_modified_id(WORLD.0, self.entity_added, self.component_type_id);
        }
    }

    fn get_member_u64(&self, offset: u32) -> u64 {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut u64;
            let member_value: u64 = *member_ptr;
            member_value
        }
    }

    fn set_member_i8(&self, offset: u32, value: i8) {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut i8;
            *member_ptr = value;
            ecs_modified_id(WORLD.0, self.entity_added, self.component_type_id);
        }
    }

    fn get_member_i8(&self, offset: u32) -> i8 {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut i8;
            let member_value: i8 = *member_ptr;
            member_value
        }
    }

    fn set_member_i16(&self, offset: u32, value: i16) {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut i16;
            *member_ptr = value;
            ecs_modified_id(WORLD.0, self.entity_added, self.component_type_id);
        }
    }

    fn get_member_i16(&self, offset: u32) -> i16 {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut i16;
            let member_value: i16 = *member_ptr;
            member_value
        }
    }

    fn set_member_i32(&self, offset: u32, value: i32) {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut i32;
            *member_ptr = value;
            ecs_modified_id(WORLD.0, self.entity_added, self.component_type_id);
        }
    }

    fn get_member_i32(&self, offset: u32) -> i32 {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut i32;
            let member_value: i32 = *member_ptr;
            member_value
        }
    }

    fn set_member_i64(&self, offset: u32, value: i64) {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut i64;
            *member_ptr = value;
            ecs_modified_id(WORLD.0, self.entity_added, self.component_type_id);
        }
    }

    fn get_member_i64(&self, offset: u32) -> i64 {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut i64;
            let member_value: i64 = *member_ptr;
            member_value
        }
    }   

    fn set_member_f32(&self, offset: u32, value: f32) {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut f32;
            *member_ptr = value;
            ecs_modified_id(WORLD.0, self.entity_added, self.component_type_id);
        }
    }

    fn get_member_f32(&self, offset: u32) -> f32 {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut f32;
            let member_value: f32 = *member_ptr;
            member_value
        }
    }

    fn set_member_f64(&self, offset: u32, value: f64) {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut f64;
            *member_ptr = value;
            ecs_modified_id(WORLD.0, self.entity_added, self.component_type_id);
        }
    }

    fn get_member_f64(&self, offset: u32) -> f64 {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut f64;
            let member_value: f64 = *member_ptr;
            member_value
        }
    }

    fn set_member_bool(&self, offset: u32, value: bool) {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut bool;
            *member_ptr = value;
            ecs_modified_id(WORLD.0, self.entity_added, self.component_type_id);
        }
    }

    fn get_member_bool(&self, offset: u32) -> bool {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut bool;
            let member_value: bool = *member_ptr;
            member_value
        }
    }

    fn set_member_string(&self, offset: u32, value: String) {
        unsafe {
            // Convert to c_string
            let c_string = std::ffi::CString::new(value).expect("CString::new failed");
            let c_ptr = c_string.as_ptr();
            std::mem::forget(c_string); // Prevent CString from being deallocated
            let member_ptr = self.ptr.offset(offset as isize) as *mut *const i8;
            *member_ptr = c_ptr;
            ecs_modified_id(WORLD.0, self.entity_added, self.component_type_id);
        }
    }

    fn get_member_string(&self, offset: u32) -> String {
        unsafe {        
            let member_ptr = self.ptr.offset(offset as isize) as *mut *const i8;
            let member_value: String = unsafe { CStr::from_ptr(*member_ptr).to_string_lossy().into_owned() };
            member_value
        }
    }

    fn set_member_u8list(&self, offset: u32, value: Vec<u8>) {
        unsafe {
            // Get pointer to member
            let member_ptr = self.ptr.offset(offset as isize) as *mut *mut u8;
            // Allocate memory for array data
            let layout = std::alloc::Layout::array::<u8>(value.len()).unwrap();
            // Allocate memory for length (usize) + array data
            let ptr = std::alloc::alloc(layout) as *mut u8;
            // Copy array data to allocated memory
            std::ptr::copy_nonoverlapping(value.as_ptr(), ptr, value.len());
            
            // Store pointer to allocated memory
            *member_ptr = ptr;
            // Store length of array
            // Use the actual array pointer as the key
            ARRAY_LENGTH_CACHE.lock().unwrap().insert((ThreadSafePtr(ptr as *const c_void), offset), value.len());
            
            // Mark component as modified
            ecs_modified_id(WORLD.0, self.entity_added, self.component_type_id);
        }
    }

    fn get_member_u8list(&self, offset: u32) -> Vec<u8> {
        unsafe {
            // Get pointer to member
            let member_ptr = self.ptr.offset(offset as isize) as *mut *mut u8;
            // Get pointer to allocated memory
            let ptr = *member_ptr;
            
            // Use the actual array pointer as the key
            let length = ARRAY_LENGTH_CACHE.lock().unwrap()
                .get(&(ThreadSafePtr(ptr as *const c_void), offset))
                .copied()
                .unwrap_or(0);

            // Create slice from allocated memory
            let slice = std::slice::from_raw_parts(ptr, length);
            // Return slice as vector
            slice.to_vec()
        }
    }

    fn set_member_u16list(&self, offset: u32, value: Vec<u16>) {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut *mut u16;
            let layout = std::alloc::Layout::array::<u16>(value.len()).unwrap();
            let ptr = std::alloc::alloc(layout) as *mut u16;
            
            std::ptr::copy_nonoverlapping(value.as_ptr(), ptr, value.len());
            
            *member_ptr = ptr;
            // Use the actual array pointer as the key
            ARRAY_LENGTH_CACHE.lock().unwrap().insert((ThreadSafePtr(ptr as *const c_void), offset), value.len());
            
            ecs_modified_id(WORLD.0, self.entity_added, self.component_type_id);
        }
    }

    fn get_member_u16list(&self, offset: u32) -> Vec<u16> {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut *mut u16;
            let ptr = *member_ptr;
            
            // Use the actual array pointer as the key
            let length = ARRAY_LENGTH_CACHE.lock().unwrap()
                .get(&(ThreadSafePtr(ptr as *const c_void), offset))
                .copied()
                .unwrap_or(0);
            let slice = std::slice::from_raw_parts(ptr, length);
            slice.to_vec()
        }
    }

    fn set_member_u32list(&self, offset: u32, value: Vec<u32>) {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut *mut u32;
            let layout = std::alloc::Layout::array::<u32>(value.len()).unwrap();
            let ptr = std::alloc::alloc(layout) as *mut u32;
            
            std::ptr::copy_nonoverlapping(value.as_ptr(), ptr, value.len());
            
            *member_ptr = ptr;
            ARRAY_LENGTH_CACHE.lock().unwrap().insert((ThreadSafePtr(ptr as *const c_void), offset), value.len());
            
            ecs_modified_id(WORLD.0, self.entity_added, self.component_type_id);
        }
    }

    fn get_member_u32list(&self, offset: u32) -> Vec<u32> {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut *mut u32;
            let ptr = *member_ptr;
            
            let length = ARRAY_LENGTH_CACHE.lock().unwrap()
                .get(&(ThreadSafePtr(ptr as *const c_void), offset))
                .copied()
                .unwrap_or(0);
            let slice = std::slice::from_raw_parts(ptr, length);
            slice.to_vec()
        }
    }

    fn set_member_u64list(&self, offset: u32, value: Vec<u64>) {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut *mut u64;
            let layout = std::alloc::Layout::array::<u64>(value.len()).unwrap();
            let ptr = std::alloc::alloc(layout) as *mut u64;
            
            std::ptr::copy_nonoverlapping(value.as_ptr(), ptr, value.len());
            
            *member_ptr = ptr;
            ARRAY_LENGTH_CACHE.lock().unwrap().insert((ThreadSafePtr(ptr as *const c_void), offset), value.len());
            
            ecs_modified_id(WORLD.0, self.entity_added, self.component_type_id);
        }
    }

    fn get_member_u64list(&self, offset: u32) -> Vec<u64> {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut *mut u64;
            let ptr = *member_ptr;
            
            let length = ARRAY_LENGTH_CACHE.lock().unwrap()
                .get(&(ThreadSafePtr(ptr as *const c_void), offset))
                .copied()
                .unwrap_or(0);
            let slice = std::slice::from_raw_parts(ptr, length);
            slice.to_vec()
        }
    }

    fn set_member_i8list(&self, offset: u32, value: Vec<i8>) {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut *mut i8;
            let layout = std::alloc::Layout::array::<i8>(value.len()).unwrap();
            let ptr = std::alloc::alloc(layout) as *mut i8;
            
            std::ptr::copy_nonoverlapping(value.as_ptr(), ptr, value.len());
            
            *member_ptr = ptr;
            ARRAY_LENGTH_CACHE.lock().unwrap().insert((ThreadSafePtr(ptr as *const c_void), offset), value.len());
            
            ecs_modified_id(WORLD.0, self.entity_added, self.component_type_id);
        }
    }

    fn get_member_i8list(&self, offset: u32) -> Vec<i8> {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut *mut i8;
            let ptr = *member_ptr;
            
            let length = ARRAY_LENGTH_CACHE.lock().unwrap()
                .get(&(ThreadSafePtr(ptr as *const c_void), offset))
                .copied()
                .unwrap_or(0);
            let slice = std::slice::from_raw_parts(ptr, length);
            slice.to_vec()
        }
    }

    fn set_member_i16list(&self, offset: u32, value: Vec<i16>) {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut *mut i16;
            let layout = std::alloc::Layout::array::<i16>(value.len()).unwrap();
            let ptr = std::alloc::alloc(layout) as *mut i16;
            
            std::ptr::copy_nonoverlapping(value.as_ptr(), ptr, value.len());
            
            *member_ptr = ptr;
            ARRAY_LENGTH_CACHE.lock().unwrap().insert((ThreadSafePtr(ptr as *const c_void), offset), value.len());
            
            ecs_modified_id(WORLD.0, self.entity_added, self.component_type_id);
        }
    }

    fn get_member_i16list(&self, offset: u32) -> Vec<i16> {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut *mut i16;
            let ptr = *member_ptr;
            
            let length = ARRAY_LENGTH_CACHE.lock().unwrap()
                .get(&(ThreadSafePtr(ptr as *const c_void), offset))
                .copied()
                .unwrap_or(0);
            let slice = std::slice::from_raw_parts(ptr, length);
            slice.to_vec()
        }
    }

    fn set_member_i32list(&self, offset: u32, value: Vec<i32>) {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut *mut i32;
            let layout = std::alloc::Layout::array::<i32>(value.len()).unwrap();
            let ptr = std::alloc::alloc(layout) as *mut i32;
            
            std::ptr::copy_nonoverlapping(value.as_ptr(), ptr, value.len());
            
            *member_ptr = ptr;
            ARRAY_LENGTH_CACHE.lock().unwrap().insert((ThreadSafePtr(ptr as *const c_void), offset), value.len());
            
            ecs_modified_id(WORLD.0, self.entity_added, self.component_type_id);
        }
    }

    fn get_member_i32list(&self, offset: u32) -> Vec<i32> {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut *mut i32;
            let ptr = *member_ptr;
            
            let length = ARRAY_LENGTH_CACHE.lock().unwrap()
                .get(&(ThreadSafePtr(ptr as *const c_void), offset))
                .copied()
                .unwrap_or(0);
            let slice = std::slice::from_raw_parts(ptr, length);
            slice.to_vec()
        }
    }

    fn set_member_i64list(&self, offset: u32, value: Vec<i64>) {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut *mut i64;
            let layout = std::alloc::Layout::array::<i64>(value.len()).unwrap();
            let ptr = std::alloc::alloc(layout) as *mut i64;
            
            std::ptr::copy_nonoverlapping(value.as_ptr(), ptr, value.len());
            
            *member_ptr = ptr;
            ARRAY_LENGTH_CACHE.lock().unwrap().insert((ThreadSafePtr(ptr as *const c_void), offset), value.len());
            
            ecs_modified_id(WORLD.0, self.entity_added, self.component_type_id);
        }
    }

    fn get_member_i64list(&self, offset: u32) -> Vec<i64> {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut *mut i64;
            let ptr = *member_ptr;
            
            let length = ARRAY_LENGTH_CACHE.lock().unwrap()
                .get(&(ThreadSafePtr(ptr as *const c_void), offset))
                .copied()
                .unwrap_or(0);
            let slice = std::slice::from_raw_parts(ptr, length);
            slice.to_vec()
        }
    }

    fn set_member_f32list(&self, offset: u32, value: Vec<f32>) {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut *mut f32;
            let layout = std::alloc::Layout::array::<f32>(value.len()).unwrap();
            let ptr = std::alloc::alloc(layout) as *mut f32;
            
            std::ptr::copy_nonoverlapping(value.as_ptr(), ptr, value.len());
            
            *member_ptr = ptr;
            ARRAY_LENGTH_CACHE.lock().unwrap().insert((ThreadSafePtr(ptr as *const c_void), offset), value.len());
            
            ecs_modified_id(WORLD.0, self.entity_added, self.component_type_id);
        }
    }

    fn get_member_f32list(&self, offset: u32) -> Vec<f32> {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut *mut f32;
            let ptr = *member_ptr;
            
            let length = ARRAY_LENGTH_CACHE.lock().unwrap()
                .get(&(ThreadSafePtr(ptr as *const c_void), offset))
                .copied()
                .unwrap_or(0);
            let slice = std::slice::from_raw_parts(ptr, length);
            slice.to_vec()
        }
    }

    fn set_member_f64list(&self, offset: u32, value: Vec<f64>) {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut *mut f64;
            let layout = std::alloc::Layout::array::<f64>(value.len()).unwrap();
            let ptr = std::alloc::alloc(layout) as *mut f64;
            
            std::ptr::copy_nonoverlapping(value.as_ptr(), ptr, value.len());
            
            *member_ptr = ptr;
            ARRAY_LENGTH_CACHE.lock().unwrap().insert((ThreadSafePtr(ptr as *const c_void), offset), value.len());
            
            ecs_modified_id(WORLD.0, self.entity_added, self.component_type_id);
        }
    }

    fn get_member_f64list(&self, offset: u32) -> Vec<f64> {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut *mut f64;
            let ptr = *member_ptr;
            
            let length = ARRAY_LENGTH_CACHE.lock().unwrap()
                .get(&(ThreadSafePtr(ptr as *const c_void), offset))
                .copied()
                .unwrap_or(0);
            let slice = std::slice::from_raw_parts(ptr, length);
            slice.to_vec()
        }
    }

    fn set_member_pointer(&self, offset: u32, value: u64) {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut *mut u64;
            *member_ptr = value as *mut u64;
        }
    }

    fn get_member_pointer(&self, offset: u32) -> u64 {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut *mut u64;
            *member_ptr as u64
        }
    }
}

impl GuestEntity for Entity {
    fn new(desc: EntityDesc, inherits: Option<ecs_entity_t>) -> Entity {
        unsafe {
            let mut ent_desc: ecs_entity_desc_t = MaybeUninit::zeroed().assume_init();
            if let Some(name) = desc.name {
                ent_desc.name = c_string(&name);
            }
            if let Some(mut add) = desc.add {
                if desc.prefab {
                    add.push(EcsPrefab);
                }
                let add_array = add.iter().map(|&x| x).collect::<Vec<_>>();
                ent_desc.add = add_array.as_ptr();
                std::mem::forget(add_array); // Prevent deallocation
            }
            let entity = if let Some(inherits) = inherits {
                let pair = ecs_make_pair(EcsIsA, inherits);
                ecs_new_w_id(WORLD.0, pair)
            } else {
                ecs_entity_init(WORLD.0, &ent_desc)
            };
            Entity { id: entity }
        }
    }
    
    fn get_id(&self) -> ecs_entity_t {
        self.id
    }

    fn get_name(&self) -> String {
        unsafe {
            let name = ecs_get_name(WORLD.0, self.id);
            let name_str = unsafe { CStr::from_ptr(name) };
            name_str.to_str().unwrap().to_string()
        }
    }

    fn set_name(&self, name: String) {
        unsafe { ecs_set_name(WORLD.0, self.id, c_string(&name)) };
    }

    fn add(&self, component: ecs_entity_t) {
        unsafe {
            // println!("Adding component {:?} to entity {:?}", component, self.id);
            // ecs_emplace_id(WORLD.0, self.id, component, &mut true as *mut bool);
            ecs_add_id(WORLD.0, self.id, component);
        }
    }

    fn has(&self, component: ecs_entity_t) -> bool {
        unsafe {
            ecs_has_id(WORLD.0, self.id, component)
        }
    }

    fn remove(&self, component: ecs_entity_t) {
        unsafe {
            ecs_remove_id(WORLD.0, self.id, component);
        }
    }

    fn from_id(id: u64) -> PointerT {
        Box::into_raw(Box::new(Entity { id })) as PointerT
    }

    fn get(&self, component: ecs_entity_t) -> PointerT {
        unsafe {
            // println!("Getting component {:?} from entity {:?}", component, self.id);
            ecs_ensure_id(WORLD.0, self.id, component) as PointerT
        }
    }

    fn child_of(&self, target: ecs_entity_t) {
        self.add_relationship(Relationship::ChildOf, target);
    }

    fn parent_of(&self, target: ecs_entity_t) {
        unsafe {
            let pair = ecs_make_pair(EcsChildOf, self.id);
            ecs_add_id(WORLD.0, target, pair); 
        }
    }

    fn parent(&self) -> EcsEntityT {
        let id = unsafe { ecs_get_parent(WORLD.0, self.id) };
        id
    }

    fn children(&self) -> Vec<EcsEntityT> {
        unsafe {
            let mut iter = ecs_children(WORLD.0, self.id);
            let mut children = Vec::new();
            while ecs_children_next(&mut iter) {
                if !iter.entities.is_null() && iter.count > 0 {
                    let entities_slice = std::slice::from_raw_parts(iter.entities, iter.count as usize);
                    // Simply return the raw entity IDs
                    children.extend_from_slice(entities_slice);
                }
            }
            children
        }
    }

    fn relationships(&self) -> Vec<EcsEntityT> {
        // TODO: Needs Flecs filter
        unimplemented!()
    }

    fn add_relationship(&self, relationship: Relationship, target: ecs_entity_t) {
        unsafe { 
            let relationship_entity = match relationship {
                Relationship::IsA => EcsIsA,
                Relationship::ChildOf => EcsChildOf,
                Relationship::Custom(entity) => entity
            };
            let pair = ecs_make_pair(EcsChildOf, target);
            ecs_add_id(WORLD.0, self.id, pair); 
        };
    }

    fn remove_relationship(&self, relationship: Relationship, target: ecs_entity_t) {
        unsafe {
            let relationship_entity = match relationship {
                Relationship::IsA => EcsIsA,
                Relationship::ChildOf => EcsChildOf,
                Relationship::Custom(entity) => entity
            };
            let pair = ecs_make_pair(relationship_entity, target);
            ecs_remove_id(WORLD.0, self.id, pair);
        }
    }

    fn disable(&self) {
        unsafe { ecs_enable(WORLD.0, self.id, false) };
    }

    fn enable(&self) {
        unsafe { ecs_enable(WORLD.0, self.id, true) };
    }
}

impl GuestQuery for Query {
    fn new(query_desc: QueryDesc) -> Query {
        let mut desc: ecs_query_desc_t = unsafe { MaybeUninit::zeroed().assume_init() };
        desc.expr = c_string(&query_desc.expr);
        Query { 
            desc: RefCell::new(desc), 
            query: RefCell::new(unsafe { MaybeUninit::zeroed().assume_init() }), 
            iter: RefCell::new(unsafe { MaybeUninit::zeroed().assume_init() }) 
        }
    }

    fn build(&self) { 
        let query = unsafe { ecs_query_init(WORLD.0, self.desc.as_ptr()) };
        *self.query.borrow_mut() = unsafe { *query };
    }

    fn iter(&self) -> PointerT {
        // Create new iterator
        let iter = unsafe { ecs_query_iter(WORLD.0, self.query.as_ptr()) };
        
        // Store it in our RefCell
        *self.iter.borrow_mut() = iter;

        // Create a new Iter that points to our stored iterator
        let iter_ptr = self.iter.as_ptr();
        
        // Return a boxed Iter
        Box::into_raw(Box::new(Iter { ptr: iter_ptr as *mut c_void })) as PointerT
    }

    fn next(&self) -> bool {
        let iter_ptr = self.iter.as_ptr();
        if iter_ptr.is_null() {
            return false;
        }
        unsafe { ecs_query_next(iter_ptr) }
    }

    fn count(&self) -> i32 {
        self.iter.borrow().count
    }

    fn entities(&self) -> Vec<EcsEntityT> {
        let entities = self.iter.borrow().entities;
        let entities_slice = unsafe { std::slice::from_raw_parts(entities, self.count() as usize) };
        entities_slice.to_vec()
    }

    fn components(&self, index: i8) -> Vec<PointerT> {
        // Get iter as raw pointer
        let iter = self.iter.as_ptr();
        // Get count of components
        let count = unsafe { (*iter).count };
        if count != 0 {
            // Get size of field (list of components of type T mapped by index)
            let size = unsafe { ecs_field_size(iter, index) };
            // Get field at index (list of components of type T mapped by index)
            let field = unsafe { ecs_field_w_size(iter, size, index) };
            // Create a slice of the field data and convert directly to Vec
            unsafe {
                // Create a vector to store component pointers
                let mut components = Vec::with_capacity(count as usize);
                for i in 0..count {
                    // Calculate pointer to each component using size
                    let component_ptr = (field as *const u8).add(i as usize * size as usize);
                    components.push(component_ptr as PointerT);
                }
                components
            }
        } else {
            vec![]
        }
    }
}

impl GuestIter for Iter {
    fn new(ptr: u64) -> Iter {
        Iter { ptr: ptr as *mut c_void }
    }

    fn next(&self) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ecs_query_next(self.ptr as *mut ecs_iter_t) }
    }

    fn count(&self) -> i32 {
        let iter = unsafe { *(self.ptr as *mut ecs_iter_t) };
        iter.count
    }

    fn entities(&self) -> Vec<EcsEntityT> {
        let iter = unsafe { *(self.ptr as *mut ecs_iter_t) };
        let entities = iter.entities;
        let entities_slice = unsafe { std::slice::from_raw_parts(entities, self.count() as usize) };
        entities_slice.to_vec()
    }

    fn components(&self, index: i8) -> Vec<PointerT> {
        // Get iter as raw pointer
        let iter = self.ptr as *mut ecs_iter_t;
        // Get count of components
        let count = unsafe { (*iter).count };
        if count != 0 {
            // Get size of field (list of components of type T mapped by index)
            let size = unsafe { ecs_field_size(iter, index) };
            // Get field at index (list of components of type T mapped by index)
            let field = unsafe { ecs_field_w_size(iter, size, index) };
            // Create a slice of the field data and convert directly to Vec
            unsafe {
                // Create a vector to store component pointers
                let mut components = Vec::with_capacity(count as usize);
                for i in 0..count {
                    // Calculate pointer to each component using size
                    let component_ptr = (field as *const u8).add(i as usize * size as usize);
                    components.push(component_ptr as PointerT);
                }
                components
            }
        } else {
            vec![]
        }
    }
}

pub static mut QUERY_TRAMPOLINE: Option<unsafe extern "C" fn(*mut ecs_iter_t)> = None;

impl GuestSystem for System {
    fn new(desc: SystemDesc) -> System {
        // Create system entity
        let mut entity_desc: ecs_entity_desc_t = unsafe { MaybeUninit::zeroed().assume_init() };
        // We have to add this pair so that the system is part of standard progress stage
        let pair = &[unsafe { ecs_make_pair(EcsDependsOn, EcsOnUpdate) }];
        entity_desc.add = pair.as_ptr() as *const u64;
        if let Some(name) = desc.name.clone() {
            entity_desc.name = c_string(&name);
        }
        let entity = unsafe { ecs_entity_init(WORLD.0, &entity_desc) };
        // Create system descriptor
        let mut system_desc: ecs_system_desc_t = unsafe { MaybeUninit::zeroed().assume_init() };
        system_desc.entity = entity;
        system_desc.rate = desc.tick_rate.unwrap_or(1);
        let mut query_desc: ecs_query_desc_t = unsafe { MaybeUninit::zeroed().assume_init() };
        query_desc.expr = c_string(&desc.query_desc.expr);
        system_desc.query = query_desc;
        if desc.is_guest {
            system_desc.ctx = 1 as *mut c_void;
        } else {
            system_desc.ctx = std::ptr::null_mut();
        }
        system_desc.callback_ctx = desc.callback as *mut c_void;
        system_desc.callback = Some(unsafe { QUERY_TRAMPOLINE.unwrap() });
        System { 
            desc: RefCell::new(system_desc),
            entity: RefCell::new(entity), 
            callback: RefCell::new(Callback::new(desc.callback)),
            system: RefCell::new(
                unsafe { MaybeUninit::zeroed().assume_init() }
            )
        }
    }
    
    fn get_id(&self) -> ecs_entity_t {
        *self.entity.borrow()
    }

    fn callback(&self) -> u64 {
       self.callback.borrow().handle
    }

    fn build(&self) {
        *self.entity.borrow_mut() = unsafe { ecs_system_init(WORLD.0, self.desc.as_ptr()) };
        // Set tickrate using world, system id, description, and 0 (to use frames as a source)
        unsafe { ecs_set_rate(WORLD.0, *self.entity.borrow(), self.desc.borrow().rate, 0) };
    }

    fn disable(&self) {
        unsafe { ecs_enable(WORLD.0, *self.entity.borrow(), false) };
    }

    fn enable(&self) {
        unsafe { ecs_enable(WORLD.0, *self.entity.borrow(), true) };
    }
}

impl GuestObserver for Observer {
    fn new(desc: ObserverDesc) -> Observer {
        let mut observer_desc: ecs_observer_desc_t = unsafe { MaybeUninit::zeroed().assume_init() };
        let mut query_desc: ecs_query_desc_t = unsafe { MaybeUninit::zeroed().assume_init() };
        // Create observer entity
        let mut entity_desc: ecs_entity_desc_t = unsafe { MaybeUninit::zeroed().assume_init() };
        if let Some(name) = desc.name.clone() {
            entity_desc.name = c_string(&name);
        }
        let entity = unsafe { ecs_entity_init(WORLD.0, &entity_desc) };
        observer_desc.entity = entity;
        query_desc.expr = c_string(&desc.query_desc.expr);
        observer_desc.query = query_desc;
        if desc.is_guest {
            observer_desc.ctx = 1 as *mut c_void;
        } else {
            observer_desc.ctx = std::ptr::null_mut();
        }
        observer_desc.callback_ctx = desc.callback as *mut c_void;
        observer_desc.callback = Some(unsafe { QUERY_TRAMPOLINE.unwrap() });
        let mut events = desc
            .events
            .iter()
            .map(|event| map_event(*event))
            .collect::<Vec<u64>>();
        // Pad with zeros until length is 8
        while events.len() < 8 {
            events.push(0);
        }
        // Truncate if longer than 8
        events.truncate(8);
        observer_desc.events = events.try_into().unwrap();
        Observer { 
            desc: RefCell::new(observer_desc),
            entity: RefCell::new(entity),
            observer: RefCell::new(unsafe { MaybeUninit::zeroed().assume_init() }),
            callback: RefCell::new(Callback::new(desc.callback))
        }
    }

    fn callback(&self) -> u64 {
        self.callback.borrow().handle
    }

    fn build(&self) {
        *self.entity.borrow_mut() = unsafe { 
            ecs_observer_init(WORLD.0, self.desc.as_ptr()) 
        };
    }
}

impl GuestCallback for Callback {
    fn new(handle: u64) -> Callback {
        Callback { handle }
    }

    fn cb_handle(&self) -> u64 {
        self.handle
    }

    fn run(&self, query: ecs::Iter) {
        println!("Callback run");
    }
}

impl GuestPhase for Phase {
    fn new(name: String) -> Phase {
        let entity = unsafe { ecs_new_w_id(WORLD.0, EcsPhase) };
        Phase { name, entity: RefCell::new(entity) }
    }

    fn depends_on(&self, phase: Phases) {
       let phase = unsafe { 
            match phase {
                Phases::OnStart => EcsOnStart,
                Phases::OnLoad => EcsOnLoad,
                Phases::PostLoad => EcsPostLoad,
                Phases::PreUpdate => EcsPreUpdate,
                Phases::OnUpdate => EcsOnUpdate,
                Phases::OnValidate => EcsOnValidate,
                Phases::PostUpdate => EcsPostUpdate,
                Phases::PreStore => EcsPreStore,
                Phases::OnStore => EcsOnStore,
                Phases::Custom(entity) => entity
            } 
        };
        let pair = unsafe { ecs_make_pair(EcsDependsOn, phase) };
        unsafe { ecs_add_id(WORLD.0, *self.entity.borrow(), pair) };
    }

    fn get_id(&self) -> ecs_entity_t {
        *self.entity.borrow()
    }
}

impl GuestPipeline for Pipeline {
    fn new(desc: PipelineDesc) -> Pipeline {
        let mut entity_desc: ecs_entity_desc_t = unsafe { MaybeUninit::zeroed().assume_init() };
        entity_desc.name = c_string(&desc.name);
        let entity = unsafe { ecs_entity_init(WORLD.0, &entity_desc) };
        let mut pipeline_desc: ecs_pipeline_desc_t = unsafe { MaybeUninit::zeroed().assume_init() };
        pipeline_desc.entity = entity;
        let mut query_desc: ecs_query_desc_t = unsafe { MaybeUninit::zeroed().assume_init() };
        query_desc.expr = c_string(&desc.query_desc.expr);
        pipeline_desc.query = query_desc;
        Pipeline { desc: RefCell::new(pipeline_desc), entity: RefCell::new(entity) }
    }

    fn build(&self) {
        *self.entity.borrow_mut() = unsafe { ecs_pipeline_init(WORLD.0, self.desc.as_ptr()) };
    }

    fn add_phase(&self, phase: ecs_entity_t) {
        unsafe { ecs_add_id(WORLD.0, *self.entity.borrow(), phase) };
    }

    fn get_id(&self) -> ecs_entity_t {
        *self.entity.borrow()
    }

    fn disable(&self) {
        unsafe { ecs_enable(WORLD.0, *self.entity.borrow(), false) };
    }

    fn enable(&self) {
        unsafe { ecs_enable(WORLD.0, *self.entity.borrow(), true) };
    }
}


impl Guest for ToxoidApi {
    type ComponentType = ComponentType;
    type Component = Component;
    type Entity = Entity;
    type Query = Query;
    type Iter = Iter;
    type System = System;
    type Observer = Observer;
    type Callback = Callback;
    type Phase = Phase;
    type Pipeline = Pipeline;
    
    fn add_singleton(component: ecs_entity_t) {
        unsafe { ecs_add_id(WORLD.0, component, component) };   
    }

    fn get_singleton(component: ecs_entity_t) -> u64 {
        // unsafe { ecs_get_mut_id(WORLD.0, component, component) as u64 }
        unsafe { ecs_ensure_id(WORLD.0, component, component) as u64 }  
    }

    fn remove_singleton(component: ecs_entity_t) {
        unsafe { ecs_remove_id(WORLD.0, component, component) };
    }

    fn add_entity(entity: ecs_entity_t) {
        unsafe { ecs_add_id(WORLD.0, entity, entity) };
    }

    fn remove_entity(entity: ecs_entity_t) {
        unsafe { ecs_delete(WORLD.0, entity); }
    }

    fn has_entity_named(name: String) -> bool {
        // Convert name to c_string
        let c_name = c_string(&name);
        unsafe { ecs_lookup(WORLD.0, c_name) != 0 }
    }

    fn get_component_id(component_name: String) -> ecs_entity_t {
        let c_name = c_string(&component_name);
        unsafe { ecs_lookup(WORLD.0, c_name) }
    }
}

// TODO: Don't forget to clean up the cache when components are deleted!
// I'll need to hook this into your component lifecycle management
fn cleanup_array_lengths(ptr: *const c_void) {
    ARRAY_LENGTH_CACHE.lock().unwrap().retain(|&(array_ptr, _), _| array_ptr.0 != ptr);
}