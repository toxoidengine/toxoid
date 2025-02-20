#![allow(non_snake_case)]
use std::os::raw::{c_char, c_void};

// FFI function declarations
extern "C" {
    // Component Type
    pub fn toxoid_component_type_new(desc_data: *const u8, desc_len: usize) -> u64;
    pub fn toxoid_component_type_get_id(component_type: u64) -> u64;

    // Component
    pub fn toxoid_component_new(ptr: u64, entity_id: u64, component_type_id: u64) -> u64;
    pub fn toxoid_component_set_member_u8(component: u64, offset: u32, value: u8);
    pub fn toxoid_component_get_member_u8(component: u64, offset: u32) -> u8;
    pub fn toxoid_component_set_member_u16(component: u64, offset: u32, value: u16);
    pub fn toxoid_component_get_member_u16(component: u64, offset: u32) -> u16;
    // ... other member types

    // Entity
    pub fn toxoid_entity_new(desc_data: *const u8, desc_len: usize) -> u64;
    pub fn toxoid_entity_get_id(entity: u64) -> u64;
    pub fn toxoid_entity_get_name(entity: u64) -> *const c_char;
    pub fn toxoid_entity_set_name(entity: u64, name: *const c_char);
    pub fn toxoid_entity_get(entity: u64, component: u64) -> u64;
    pub fn toxoid_entity_add(entity: u64, component: u64);
    pub fn toxoid_entity_has(entity: u64, component: u64) -> bool;
    pub fn toxoid_entity_remove(entity: u64, component: u64);

    // Query
    pub fn toxoid_query_new(expr: *const c_char) -> u64;
    pub fn toxoid_query_build(query: u64);
    pub fn toxoid_query_next(query: u64) -> bool;
    pub fn toxoid_query_count(query: u64) -> i32;
    pub fn toxoid_query_entities(query: u64, out_entities: *mut u64, max_count: usize) -> usize;
    pub fn toxoid_query_components(query: u64, index: i8, out_components: *mut u64, max_count: usize) -> usize;

    // System
    pub fn toxoid_system_new(desc_data: *const u8, desc_len: usize, callback: *const c_void) -> u64;
    pub fn toxoid_system_build(system: u64);
    pub fn toxoid_system_get_id(system: u64) -> u64;
    pub fn toxoid_system_named(system: u64, name: *const c_char);

    // World
    pub fn toxoid_world_add_singleton(component_id: u64);
    pub fn toxoid_world_get_singleton(component_id: u64) -> u64;
    pub fn toxoid_world_remove_singleton(component_id: u64);
    pub fn toxoid_world_add_entity(entity_id: u64);
    pub fn toxoid_world_remove_entity(entity_id: u64);
} 