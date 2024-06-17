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

#[cfg(all(target_arch="wasm32", target_os="emscripten"))]
pub fn split_u64(v: u64) -> SplitU64 {
    SplitU64 {
        high: (v >> 32) as u32,
        low: v as u32,
    }
}

#[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))]
pub fn split_u64(v: u64) -> u64 {
    v
}

#[cfg(all(target_arch="wasm32", target_os="emscripten"))]
pub fn combine_u32(split_u64: SplitU64) -> u64 {
    ((split_u64.high as u64) << 32) | (split_u64.low as u64)
}

#[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))]
pub fn combine_u32(split_u64: u64) -> u64 {
    split_u64
}

#[derive(Debug)]
#[repr(C)]
pub struct SplitF64 {
    pub high: f32,
    pub low: f32,
}

pub fn split_f64(value: f64) -> SplitF64 {
    let bits = value.to_bits();           // Get the binary representation of f64 as u64
    let high = ((bits >> 32) & 0xFFFFFFFF) as u32; // Extract the high 32 bits
    let low = (bits & 0xFFFFFFFF) as u32;         // Extract the low 32 bits
    SplitF64 {
        high: f32::from_bits(high), // Convert the high bits back to f32
        low: f32::from_bits(low),   // Convert the low bits back to f32
    }
}

pub fn combine_f32(split_f64: SplitF64) -> f64 {
    let bits1 = split_f64.high.to_bits() as u64;  // Convert the first f32 to u32 and then to u64
    let bits2 = split_f64.high.to_bits() as u64;  // Convert the second f32 to u32 and then to u64
    let combined = (bits1 << 32) | bits2; // Shift the first 32 bits left and combine with the second 32 bits
    f64::from_bits(combined)              // Convert the u64 to f64
}

#[link(name = "engine")]
extern "C" {
    pub fn toxoid_print_i32(v: i32);
    #[cfg(all(target_arch="wasm32", target_os="emscripten"))]
    pub fn toxoid_print_u64(v: SplitU64);
    #[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))]
    pub fn toxoid_print_u64(v: u64);
    pub fn toxoid_print_f32(v: f32);
    #[cfg(all(target_arch="wasm32", target_os="emscripten"))]
    pub fn toxoid_print_f64(v: SplitF64);
    #[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))]
    pub fn toxoid_print_f64(v: f64);
    pub fn toxoid_print_string(v: *const i8, v_len: usize);
    pub fn toxoid_register_tag(name: *const i8, name_len: usize) -> ecs_entity_t;
    #[cfg(all(target_arch="wasm32", target_os="emscripten"))]
    pub fn toxoid_register_component_ecs(
        name: &str,
        member_names: &[&str],
        member_types: &[u8],
    ) -> SplitU64;
    #[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))]
    pub fn toxoid_register_component_ecs(
        name: &str,
        member_names: &[&str],
        member_types: &[u8],
    ) -> ecs_entity_t;
    #[cfg(all(target_arch="wasm32", target_os="emscripten"))]
    pub fn toxoid_entity_create() -> SplitU64;
    #[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))]
    pub fn toxoid_entity_create() -> ecs_entity_t;
    pub fn toxoid_entity_set_name(entity_id: ecs_entity_t, name: &str);
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
    pub fn toxoid_query_order_by(query_desc: *mut c_void, component_id: ecs_entity_t, callback: extern "C" fn(ecs_entity_t, *const c_void, ecs_entity_t, *const c_void) -> i32);
    pub fn toxoid_query_build(query_desc: *mut c_void) -> *mut c_void;
    pub fn toxoid_query_iter(query: *mut c_void) -> *mut c_void;
    pub fn toxoid_query_next(iter: *mut c_void) -> bool;
    // pub fn toxoid_query_count(iter: *mut c_void) -> i32;
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
    // pub fn toxoid_filter_term_count(filter: *mut c_void) -> i32;
    // pub fn toxoid_filter_term_size(filter: *mut c_void, term_index: i32) -> usize;
    // pub fn toxoid_filter_term_list(filter: *mut c_void, term_index: i32, count: u32) -> &'static mut [*const c_void];
    // pub fn toxoid_filter_term_iter(filter: *mut c_void, term_index: i32) -> *mut c_void;
    // pub fn toxoid_filter_term_next(iter: *mut c_void) -> bool;
    // pub fn toxoid_filter_term_entity(iter: *mut c_void, count: u32, index: u32) -> ecs_entity_t;
    // pub fn toxoid_filter_term_entity_list(iter: *mut c_void) -> *mut ecs_entity_t;
    pub fn toxoid_iter_count(iter: *mut c_void) -> i32;
    #[cfg(all(target_arch="wasm32", target_os="emscripten"))]
    pub fn toxoid_component_cache_insert(type_id: SplitU64, component_id: SplitU64);
    #[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))]
    pub fn toxoid_component_cache_insert(type_id: u64, component_id: ecs_entity_t);
    #[cfg(all(target_arch="wasm32", target_os="emscripten"))]
    pub fn toxoid_component_cache_get(type_id: SplitU64) -> SplitU64;
    #[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))]
    pub fn toxoid_component_cache_get(type_id: u64) -> ecs_entity_t;
    #[cfg(all(target_arch="wasm32", target_os="emscripten"))]
    pub fn toxoid_network_entity_cache_insert(network_id: SplitU64, entity_id: SplitU64);
    #[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))]
    pub fn toxoid_network_entity_cache_insert(network_id: u64, entity_id: ecs_entity_t);
    #[cfg(all(target_arch="wasm32", target_os="emscripten"))]
    pub fn toxoid_network_entity_cache_get(network_id: SplitU64) -> SplitU64;
    #[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))]
    pub fn toxoid_network_entity_cache_get(network_id: u64) -> ecs_entity_t;
    #[cfg(all(target_arch="wasm32", target_os="emscripten"))]
    pub fn toxoid_network_entity_cache_remove(network_id: SplitU64);
    #[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))]
    pub fn toxoid_network_entity_cache_remove(network_id: u64);
    pub fn toxoid_component_get_member_u8(component_ptr: *mut c_void, offset: u32) -> u8;
    pub fn toxoid_component_get_member_u16(component_ptr: *mut c_void, offset: u32) -> u8;
    pub fn toxoid_component_get_member_u32(component_ptr: *mut c_void, offset: u32) -> u32;
    #[cfg(all(target_arch="wasm32", target_os="emscripten"))]
    pub fn toxoid_component_get_member_u64(component_ptr: *mut c_void, offset: u32) -> SplitU64;
    #[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))]
    pub fn toxoid_component_get_member_u64(component_ptr: *mut c_void, offset: u32) -> ecs_entity_t;
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
    #[cfg(all(target_arch="wasm32", target_os="emscripten"))]
    pub fn toxoid_component_set_member_u64(component_ptr: *mut c_void, offset: u32, value: SplitU64);
    #[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))]
    pub fn toxoid_component_set_member_u64(component_ptr: *mut c_void, offset: u32, value: ecs_entity_t);
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
        value: &str,
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
    #[cfg(all(target_arch="wasm32", target_os="emscripten"))]
    pub fn toxoid_prefab_create() -> SplitU64;
    #[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))]
    pub fn toxoid_prefab_create() -> ecs_entity_t;
    pub fn toxoid_prefab_instance(prefab_high: u32, prefab_low: u32) -> SplitU64;
    pub fn toxoid_system_create(callback_closure: fn(&mut Iter)) -> *mut c_void;
    #[cfg(all(target_arch="wasm32", target_os="emscripten"))]
    pub fn toxoid_system_build(system_desc: *mut c_void) -> SplitU64;
    #[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))]
    pub fn toxoid_system_build(system_desc: *mut c_void) -> ecs_entity_t;
    pub fn toxoid_query_from_system_desc(query_desc: *mut c_void) -> *mut c_void;
    pub fn toxoid_network_send(network_messages: *mut c_void);
    #[cfg(all(target_arch="wasm32", target_os="emscripten"))]
    pub fn toxoid_net_send_components(
        entity_id: SplitU64,
        components: &[&dyn Component], 
        event: &str
    );
    #[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))]
    pub fn toxoid_net_send_components(
        entity_id: ecs_entity_t,
        components: &[&dyn Component], 
        event: &str
    );
    #[cfg(all(target_arch="wasm32", target_os="emscripten"))]
    pub fn toxoid_component_lookup(name: *mut i8) -> SplitU64;
    #[cfg(any(not(target_arch="wasm32"), all(target_arch="wasm32", target_os="unknown")))]
    pub fn toxoid_component_lookup(name: *mut i8) -> ecs_entity_t;
    #[cfg(all(target_arch="wasm32"))]
    pub fn toxoid_net_add_event(
        event_name: &str,
        callback: extern "C" fn(message: &crate::net::MessageEntity)
    );
    #[cfg(not(target_arch="wasm32"))]
    pub fn toxoid_net_add_event(
        event_name: &str,
        callback: &mut Box<dyn FnMut(&crate::net::MessageEntity) + Send + Sync>
    );
    pub fn toxoid_deserialize_entity_sync(entity_id: ecs_entity_t, components_serialized: &[crate::net::MessageComponent]);
    pub fn toxoid_make_c_string(string: &str) -> *mut i8;
    pub fn toxoid_get_timestamp() -> SplitF64;
    pub fn gen_uuid() -> *mut c_char;
    // pub fn toxoid_engine_load_sprite(filename: &str, callback: extern "C" fn(&mut Entity)) -> *mut Entity;
    // pub fn toxoid_engine_load_worldmap(filename: &str, callback: extern "C" fn(&mut Entity)) -> *mut Entity;
    // pub fn toxoid_deserialize_entity(components_serialized: &[MessageComponent]) -> HashMap<String, HashMap<String, DynamicType>>;
    // TODO: Replace this function with a generic abstraction for rand
    // pub fn gen_rng_grid_pos() -> (i32, i32);
    // pub fn toxoid_serialize_entity(entity_id: ecs_entity_t) -> Vec<crate::net::NetworkMessageComponent>;
    // pub fn toxoid_serialize_component(entity_id: ecs_entity_t, component_id: ecs_entity_t) -> crate::net::NetworkMessageComponent;
    // pub fn toxoid_register_component(
    //     component_name: *const c_char,
    //     component_name_len: u8,
    //     member_names: *const *const c_char,
    //     member_names_count: u32,
    //     member_names_len: *const u8,
    //     member_types: *const u8,
    //     member_types_count: u32,
    // ) -> ecs_entity_t;
    // pub fn toxoid_create_vec() -> *mut c_void;
    // pub fn toxoid_vec_push(ptr: *mut c_void, value: *mut c_void);
    // pub fn toxoid_vec_drop(ptr: *mut c_void);
    // pub fn toxoid_vec_as_slice(ptr: *mut c_void) -> (*mut *mut c_void, i32);
    // pub fn toxoid_free_slice(ptr: *mut c_void, len: usize);
    // pub fn toxoid_entity_get_name(id: i32);
}

