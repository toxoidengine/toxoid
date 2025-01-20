#![allow(non_camel_case_types)]
#![allow(warnings)]

pub mod bindings;
use bindings::exports::toxoid::engine::ecs::{EcsEntityT, GuestIter};
use bindings::exports::toxoid::engine::ecs::{self, ComponentDesc, EntityDesc, Guest, GuestCallback, GuestComponent, GuestComponentType, GuestEntity, GuestQuery, GuestSystem, QueryDesc, SystemDesc};
pub use toxoid_flecs::bindings::{ecs_add_id, ecs_entity_desc_t, ecs_entity_init, ecs_fini, ecs_get_mut_id, ecs_init, ecs_iter_t, ecs_lookup, ecs_make_pair, ecs_member_t, ecs_progress, ecs_query_desc_t, ecs_query_init, ecs_query_iter, ecs_query_next, ecs_query_t, ecs_struct_desc_t, ecs_struct_init, ecs_system_desc_t, ecs_system_init, ecs_system_t, ecs_world_t, EcsDependsOn, EcsOnUpdate, ecs_set_rate, ecs_get_id, ecs_remove_id};
use toxoid_flecs::{ecs_delete, ecs_ensure_id};
use std::{borrow::BorrowMut, mem::MaybeUninit};
use core::ffi::c_void;
use core::ffi::c_char;
use std::cell::RefCell;
use std::collections::HashMap;
use once_cell::sync::Lazy;
type ecs_entity_t = u64;

pub struct ToxoidApi;

pub struct ComponentType { 
    pub id: ecs_entity_t
}

pub struct Component {
    pub ptr: *const c_void,
    pub field_offsets: Vec<u8>
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

pub struct Callback {
    pub handle: i64
}

pub struct Iter {
    pub ptr: *mut c_void
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
    U32List,
    U64List,
    F32List,
    Pointer
}

pub static mut WORLD: Lazy<EcsWorldPtr> = Lazy::new(|| 
    EcsWorldPtr(unsafe { 
        let world = ecs_init();
        // toxoid_flecs::bindings::ecs_set_threads(world, 4);
        world
    })
);

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

unsafe fn get_member_type(member_type: u8) -> ecs_entity_t {
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
            // Create component entity
            let mut ent_desc: ecs_entity_desc_t = MaybeUninit::zeroed().assume_init();
            ent_desc.name = c_string(&desc.name);
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
                member.type_ = get_member_type(desc.member_types[index]);
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

    fn get_id(&self) -> ecs_entity_t {
        self.id
    }
}

impl GuestComponent for Component {
    // This is a component instance so it will need a the entity it belongs to and the component type
    // This is required for observers / events to work
    fn new(ptr: i64, entity: ecs_entity_t, component: ecs_entity_t) -> Component {
        Component { ptr: ptr as *const c_void, field_offsets: vec![] }
    }

    fn set_member_u8(&self, offset: u32, value: u8) {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut u8;
            *member_ptr = value;
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
            let member_ptr = self.ptr.offset(offset as isize) as *mut String;
            *member_ptr = value;
        }
    }

    fn get_member_string(&self, offset: u32) -> String {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut String;
            let member_value: String = (*member_ptr).clone();
            member_value
        }
    }

    fn set_member_u32list(&self, offset: u32, value: Vec<u32>) {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut Vec<u32>;
            *member_ptr = value;
        }
    }

    fn get_member_u32list(&self, offset: u32) -> Vec<u32> {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut Vec<u32>;
            let member_value: Vec<u32> = (*member_ptr).clone();
            member_value
        }
    }   

    fn set_member_u64list(&self, offset: u32, value: Vec<u64>) {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut Vec<u64>;
            *member_ptr = value;
        }
    }

    fn get_member_u64list(&self, offset: u32) -> Vec<u64> {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut Vec<u64>;
            let member_value: Vec<u64> = (*member_ptr).clone();
            member_value
        }
    }

    fn set_member_f32list(&self, offset: u32, value: Vec<f32>) {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut Vec<f32>;
            *member_ptr = value;
        }
    }

    fn get_member_f32list(&self, offset: u32) -> Vec<f32> {
        unsafe {
            let member_ptr = self.ptr.offset(offset as isize) as *mut Vec<f32>;
            let member_value: Vec<f32> = (*member_ptr).clone();
            member_value
        }
    }
}

impl GuestEntity for Entity {
    fn new(desc: EntityDesc) -> Entity {
        unsafe {
            let mut ent_desc: ecs_entity_desc_t = MaybeUninit::zeroed().assume_init();
            if let Some(name) = desc.name {
                ent_desc.name = c_string(&name);
            }
            let entity = ecs_entity_init(WORLD.0, &ent_desc);
            Entity { id: entity }
        }
    }
    
    fn get_id(&self) -> ecs_entity_t {
        self.id
    }

    fn add(&self, component: ecs_entity_t) {
        unsafe {
            // println!("Adding component {:?} to entity {:?}", component, self.id);
            // ecs_emplace_id(WORLD.0, self.id, component, &mut true as *mut bool);
            ecs_add_id(WORLD.0, self.id, component);
        }
    }

    fn remove(&self, component: ecs_entity_t) {
        unsafe {
            ecs_remove_id(WORLD.0, self.id, component);
        }
    }

    fn from_id(id: u64) -> i64 {
        Box::into_raw(Box::new(Entity { id })) as i64
    }

    fn get(&self, component: ecs_entity_t) -> i64 {
        unsafe {
            // println!("Getting component {:?} from entity {:?}", component, self.id);
            ecs_ensure_id(WORLD.0, self.id, component) as i64
        }
    }
}

impl GuestQuery for Query {
    fn new(query_desc: QueryDesc) -> Query {
        let mut desc: ecs_query_desc_t = unsafe { MaybeUninit::zeroed().assume_init() };
        desc.expr = c_string(&query_desc.expr);
        Query { desc: RefCell::new(desc), query: unsafe { MaybeUninit::zeroed().assume_init() }, iter: unsafe { MaybeUninit::zeroed().assume_init() } }
    }

    fn expr(&self, expr: String) {
        self.desc.borrow_mut().expr = expr.as_ptr() as *const i8;
    }

    fn build(&self) { 
        *self.query.borrow_mut() = unsafe { 
            *ecs_query_init(WORLD.0, self.desc.as_ptr()) 
        };
    }

    fn iter(&self) {
        *self.iter.borrow_mut() = unsafe { ecs_query_iter(WORLD.0, self.query.as_ptr()) };
    }

    fn next(&self) -> bool {
        unsafe { ecs_query_next(self.iter.as_ptr()) }
    }

    fn count(&self) -> i32 {
        self.iter.borrow().count
    }

    fn entities(&self) -> Vec<EcsEntityT> {
        let entities = self.iter.borrow().entities;
        let entities_slice = unsafe { std::slice::from_raw_parts(entities, self.count() as usize) };
        entities_slice.to_vec()
    }

    // fn field(&self, index: u32) -> *const c_void {
    //     let iter

    //     let size = ecs_field_size(iter, term_index);
    //     let field = ecs_field_w_size(iter, size, term_index);

    //     let ptrs_slice = std::slice::from_raw_parts(field, count as usize * size);
    //     let mut component_ptrs: Vec<*const c_void> = Vec::new();

    //     for i in 0..count {
    //         let ptr = &ptrs_slice[i as usize * size];
    //         component_ptrs.push(ptr as *const c_void);
    //     }

    //     let boxed_slice = component_ptrs.into_boxed_slice();
    //     let raw_ptr = Box::into_raw(boxed_slice);

    //     raw_ptr
    // }
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

    fn callback(&self) -> i64 {
       self.callback.borrow().handle
    }

    fn build(&self) {
        *self.entity.borrow_mut() = unsafe { ecs_system_init(WORLD.0, self.desc.as_ptr()) };
        // Set tickrate using world, system id, description, and 0 (to use frames as a source)
        unsafe { ecs_set_rate(WORLD.0, *self.entity.borrow(), self.desc.borrow().rate, 0) };
        
    }
}

impl GuestCallback for Callback {
    fn new(handle: i64) -> Callback {
        Callback { handle }
    }

    fn cb_handle(&self) -> i64 {
        self.handle
    }

    fn run(&self, query: ecs::Iter) {
        println!("Callback run");
    }
}

impl GuestIter for Iter {
    fn new(ptr: i64) -> Iter {
        Iter { ptr: ptr as *mut c_void }
    }

    fn next(&self) -> bool {
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
}

impl Guest for ToxoidApi {
    type ComponentType = ComponentType;
    type Component = Component;
    type Entity = Entity;
    type Query = Query;
    type System = System;
    type Callback = Callback;
    type Iter = Iter;

    fn add_singleton(component: ecs_entity_t) {
        unsafe { ecs_add_id(WORLD.0, component, component) };   
    }

    fn get_singleton(component: ecs_entity_t) -> i64 {
        // unsafe { ecs_get_mut_id(WORLD.0, component, component) as i64 }
        unsafe { ecs_ensure_id(WORLD.0, component, component) as i64 }  
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
}