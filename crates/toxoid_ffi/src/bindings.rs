#![allow(non_camel_case_types)]

use crate::ecs::*;
use core::ffi::c_void;

pub type ecs_id_t = i32;
pub type ecs_entity_t = ecs_id_t;
pub type c_char = i8;

extern "C" {
    pub fn toxoid_print_i32(v: i32);
    pub fn toxoid_print_string(v: *const i8, v_len: usize);
    pub fn toxoid_create_vec() -> *mut c_void;
    pub fn toxoid_vec_push(ptr: *mut c_void, value: *mut c_void);
    pub fn toxoid_vec_drop(ptr: *mut c_void);
    pub fn toxoid_vec_as_slice(ptr: *mut c_void) -> (*mut *mut c_void, i32);
    pub fn toxoid_free_slice(ptr: *mut c_void, len: usize);
    pub fn toxoid_entity_get_name(id: i32);
    pub fn toxoid_register_tag(name: *const i8, name_len: usize) -> ecs_entity_t;
    pub fn toxoid_register_component(
        component_name: *const c_char,
        component_name_len: u8,
        member_names: *const *const c_char,
        member_names_count: u32,
        member_names_len: *const u8,
        member_types: *const *const u8,
        member_types_count: u32,
    ) -> ecs_entity_t;
    pub fn toxoid_entity_create() -> ecs_entity_t;
    pub fn toxoid_entity_add_component(entity: u32, component: u32) -> *mut c_void;
    pub fn toxoid_entity_add_tag(entity: u32, tag: u32);
    pub fn toxoid_entity_get_component(entity: u32, component: u32) -> *mut c_void;
    pub fn toxoid_query_create(ids: *mut i32, components_count: i32) -> *mut c_void;
    pub fn toxoid_query_iter(query: *mut c_void) -> *mut c_void;
    pub fn toxoid_query_next(iter: *mut c_void) -> bool;
    pub fn toxoid_query_count(iter: *mut c_void) -> i32;
    pub fn toxoid_query_field(
        iter: *mut c_void,
        term_index: i32,
        count: u32,
        index: u32,
    ) -> *const c_void;
    pub fn toxoid_query_entity_list(iter: *mut c_void) -> &'static [Entity];
    pub fn toxoid_iter_count(iter: *mut c_void) -> i32;
    pub fn toxoid_component_cache_insert(type_id: core::any::TypeId, component_id: i32);
    pub fn toxoid_component_cache_get(type_id: core::any::TypeId) -> i32;
    pub fn toxoid_component_set_member_u32(component_ptr: *mut c_void, offset: u32, value: u32);
    pub fn toxoid_component_get_member_u32(component_ptr: *mut c_void, offset: u32) -> u32;
    pub fn toxoid_query_field_size(
        iter: *mut c_void,
        term_index: i32
    ) -> usize;
    pub fn toxoid_query_field_list(
        iter: *mut c_void,
        term_index: i32,
        count: u32,
    ) -> &'static mut [*const c_void];
}
