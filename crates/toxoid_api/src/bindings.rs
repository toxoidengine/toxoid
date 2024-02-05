#![allow(non_camel_case_types)]
#![allow(improper_ctypes)]
use crate::ecs::*;
use core::ffi::c_void;

pub type ecs_id_t = u64;
pub type ecs_entity_t = ecs_id_t;
pub type c_char = i8;

#[repr(C)]
pub struct SplitU64 {
    pub high: u32,
    pub low: u32,
}

pub fn split_u64(v: u64) -> SplitU64 {
    SplitU64 {
        high: (v >> 32) as u32,
        low: v as u32,
    }
}

pub fn combine_u32(split_u64: SplitU64) -> u64 {
    ((split_u64.high as u64) << 32) | (split_u64.low as u64)
}

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
    pub fn register_component_ecs(
        name: &str,
        member_names: &[&str],
        member_types: &[u8],
    ) -> SplitU64;
    pub fn toxoid_register_component(
        component_name: *const c_char,
        component_name_len: u8,
        member_names: *const *const c_char,
        member_names_count: u32,
        member_names_len: *const u8,
        member_types: *const u8,
        member_types_count: u32,
    ) -> ecs_entity_t;
    #[cfg(target_arch="wasm32")]
    pub fn toxoid_entity_create() -> SplitU64;
    #[cfg(not(target_arch="wasm32"))]
    pub fn toxoid_entity_create() -> ecs_entity_t;
    pub fn toxoid_entity_add_component(entity: ecs_entity_t, component: ecs_entity_t);
    pub fn toxoid_entity_add_tag(entity: ecs_entity_t, tag: ecs_entity_t);
    pub fn toxoid_entity_get_component(entity: ecs_entity_t, component: ecs_entity_t) -> *mut c_void;
    pub fn toxoid_entity_child_of(entity: ecs_entity_t, parent: ecs_entity_t);
    pub fn toxoid_entity_children(parent: ecs_entity_t) -> *mut c_void;
    pub fn toxoid_child_entities(iter: *mut c_void) -> *mut ecs_entity_t;
    pub fn toxoid_term_next(iter: *mut c_void) -> bool;
    pub fn toxoid_query_create() -> *mut c_void;
    pub fn toxoid_query_with(query_desc: *mut c_void, filter_index: u8, ids: *mut ecs_entity_t, components_count: i32) -> u8;
    pub fn toxoid_query_without(query_desc: *mut c_void, filter_index: u8, ids: *mut ecs_entity_t, components_count: i32) -> u8;
    pub fn toxoid_query_with_or(query_desc: *mut c_void, filter_index: u8, ids: *mut ecs_entity_t, components_count: i32) -> u8;
    pub fn toxoid_query_build(query_desc: *mut c_void) -> *mut c_void;
    pub fn toxoid_query_iter(query: *mut c_void) -> *mut c_void;
    pub fn toxoid_query_next(iter: *mut c_void) -> bool;
    pub fn toxoid_query_count(iter: *mut c_void) -> i32;
    pub fn toxoid_query_field(
        iter: *mut c_void,
        term_index: i32,
        count: u32,
        index: u32,
    ) -> *const c_void;
    pub fn toxoid_query_entity_list(iter: *mut c_void) -> &'static mut [Entity];
    pub fn toxoid_query_field_size(
        iter: *mut c_void,
        term_index: i32
    ) -> usize;
    pub fn toxoid_query_field_list(
        iter: *mut c_void,
        term_index: i32,
        count: u32,
    ) -> *mut [*const c_void];
    pub fn toxoid_filter_create() -> *mut c_void;
    pub fn toxoid_filter_with(filter: *mut c_void, filter_index: u8, ids: *mut ecs_entity_t, components_count: i32) -> u8;
    pub fn toxoid_filter_without(filter: *mut c_void, filter_index: u8, ids: *mut ecs_entity_t, components_count: i32) -> u8;
    pub fn toxoid_filter_with_or(filter: *mut c_void, filter_index: u8, ids: *mut ecs_entity_t, components_count: i32) -> u8;
    pub fn toxoid_filter_build(filter: *mut c_void) -> *mut c_void;
    pub fn toxoid_filter_term_count(filter: *mut c_void) -> i32;
    pub fn toxoid_filter_term_size(filter: *mut c_void, term_index: i32) -> usize;
    pub fn toxoid_filter_term_list(filter: *mut c_void, term_index: i32, count: u32) -> &'static mut [*const c_void];
    pub fn toxoid_filter_term_iter(filter: *mut c_void, term_index: i32) -> *mut c_void;
    pub fn toxoid_filter_term_next(iter: *mut c_void) -> bool;
    pub fn toxoid_filter_term_entity(iter: *mut c_void, count: u32, index: u32) -> ecs_entity_t;
    pub fn toxoid_filter_term_entity_list(iter: *mut c_void) -> *mut ecs_entity_t;
    pub fn toxoid_iter_count(iter: *mut c_void) -> i32;
    pub fn toxoid_component_cache_insert(type_id: SplitU64, component_id: SplitU64);
    pub fn toxoid_component_cache_get(type_id: SplitU64) -> SplitU64;
    pub fn toxoid_network_entity_cache_insert(network_id: SplitU64, entity_id: SplitU64);
    pub fn toxoid_network_entity_cache_get(network_id: SplitU64) -> SplitU64;
    pub fn toxoid_add_system(system: System);
    pub fn toxoid_component_get_member_u8(component_ptr: *mut c_void, offset: u32) -> u8;
    pub fn toxoid_component_get_member_u16(component_ptr: *mut c_void, offset: u32) -> u8;
    pub fn toxoid_component_get_member_u32(component_ptr: *mut c_void, offset: u32) -> u32;
    pub fn toxoid_component_get_member_u64(component_ptr: *mut c_void, offset: u32) -> u64;
    pub fn toxoid_component_get_member_i8(component_ptr: *mut c_void, offset: u8) -> i8;
    pub fn toxoid_component_get_member_i16(component_ptr: *mut c_void, offset: u8) -> i16;
    pub fn toxoid_component_get_member_i32(component_ptr: *mut c_void, offset: u32) -> i32;
    pub fn toxoid_component_get_member_i64(component_ptr: *mut c_void, offset: u32) -> i64;
    pub fn toxoid_component_get_member_f32(component_ptr: *mut c_void, offset: u32) -> f32;
    pub fn toxoid_component_get_member_f64(component_ptr: *mut c_void, offset: u32) -> f64;
    pub fn toxoid_component_get_member_bool(component_ptr: *mut c_void, offset: u32) -> bool;
    pub fn toxoid_component_get_member_u32array(component_ptr: *mut c_void, offset: u32) -> *mut u32;
    pub fn toxoid_component_get_member_f32array(component_ptr: *mut c_void, offset: u32) -> *mut f32;
    pub fn toxoid_component_get_member_string(
        component_ptr: *mut c_void,
        offset: u32,
        len: u32,
    ) -> *mut c_char;
    pub fn toxoid_component_set_member_u8(component_ptr: *mut c_void, offset: u32, value: u8);
    pub fn toxoid_component_set_member_u16(component_ptr: *mut c_void, offset: u32, value: u16);
    pub fn toxoid_component_set_member_u32(component_ptr: *mut c_void, offset: u32, value: u32);
    pub fn toxoid_component_set_member_u64(component_ptr: *mut c_void, offset: u32, value: u64);
    pub fn toxoid_component_set_member_i8(component_ptr: *mut c_void, offset: u32, value: i8);
    pub fn toxoid_component_set_member_i16(component_ptr: *mut c_void, offset: u32, value: i16);
    pub fn toxoid_component_set_member_i32(component_ptr: *mut c_void, offset: u32, value: i32);
    pub fn toxoid_component_set_member_i64(component_ptr: *mut c_void, offset: u32, value: i64);
    pub fn toxoid_component_set_member_f32(component_ptr: *mut c_void, offset: u32, value: f32);
    pub fn toxoid_component_set_member_f64(component_ptr: *mut c_void, offset: u32, value: f64);
    pub fn toxoid_component_set_member_bool(component_ptr: *mut c_void, offset: u32, value: bool);
    pub fn toxoid_component_set_member_u32array(component_ptr: *mut c_void, offset: u32, value: *mut u32);
    pub fn toxoid_component_set_member_f32array(component_ptr: *mut c_void, offset: u32, value: *mut f32);
    pub fn toxoid_component_set_member_string(
        component_ptr: *mut c_void,
        offset: u32,
        len: u32,
        value: *mut c_char,
    );   
    pub fn toxoid_component_set_member_ptr(
        component_ptr: *mut c_void,
        offset: u32,
        value: *mut c_void,
    );
    pub fn toxoid_component_get_member_ptr(
        component_ptr: *mut c_void,
        offset: u32,
    ) -> *mut c_void;
    pub fn toxoid_progress(delta_time: f32) -> bool;
    pub fn toxoid_filter_children_init(parent: ecs_entity_t) -> *mut c_void;
    pub fn toxoid_filter_iter(filter: *mut c_void) -> *mut c_void;
    pub fn toxoid_filter_next(iter: *mut c_void) -> bool;
    pub fn toxoid_iter_entities(iter: *mut c_void) -> &'static [u64];
    pub fn toxoid_delete_entity(entity: ecs_entity_t);
    pub fn toxoid_entity_remove_component(entity: ecs_entity_t, component: ecs_entity_t);
    pub fn toxoid_is_valid(entity: ecs_entity_t) -> bool;
    pub fn toxoid_entity_has_component(
        entity: ecs_entity_t,
        component: ecs_entity_t
    ) -> bool;
    pub fn toxoid_singleton_get(component: ecs_entity_t) -> *mut c_void;
    pub fn toxoid_singleton_add(component: ecs_entity_t);
    pub fn toxoid_singleton_remove(component: ecs_entity_t);
    pub fn gen_rng_grid_pos() -> (i32, i32);
    pub fn toxoid_systems_init(system_name: *const c_char,
        ids: [ecs_id_t; 16],
        callback: unsafe extern "C" fn(*mut c_void)
    ) -> ecs_entity_t;
    pub fn toxoid_prefab_create() -> SplitU64;
    pub fn toxoid_prefab_instance(prefab_high: u32, prefab_low: u32) -> SplitU64;
    pub fn make_c_string(string: &str) -> *mut i8;
}
