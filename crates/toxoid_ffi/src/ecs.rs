#![allow(improper_ctypes)]
#![allow(improper_ctypes_definitions)]

use core::ffi::{c_char, c_void};
use std::collections::HashMap;
use core::alloc::Layout;
use flecs_core::*;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::utils::*;
use crate::allocator::*;

#[derive(Debug)]
pub enum DynamicType {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Bool(bool),
}
// TODO: Replace this with ecs_lookup
pub static COMPONENT_ID_CACHE: Lazy<Mutex<HashMap<u64, ecs_entity_t>>> = Lazy::new(|| Mutex::new(HashMap::new()));
// TODO: Maybe we can replace this with something in the ECS as well?
pub static NETWORK_ENTITY_CACHE: Lazy<Mutex<HashMap<u64, ecs_entity_t>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[no_mangle]
pub unsafe extern "C" fn toxoid_print_i32(v: i32) {
    println!("Printing from Toxoid Engine: {}", v);
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_print_u64(v: SplitU64) {
    println!("Printing from Toxoid Engine: {}", combine_u32(v));
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_print_f32(v: f32) {
    println!("Printing from Toxoid Engine: {}", v);
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_print_f64(v: SplitF64) {
    println!("Printing from Toxoid Engine: {}", combine_f32(v));
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_print_string(v: *const i8, v_len: usize) {
    // Convert the C String to a Rust string
    let slice = std::slice::from_raw_parts(v as *mut u8, v_len);
    let rust_string = std::str::from_utf8_unchecked(slice);
    println!("Printing from Toxoid Engine: {}", rust_string);
}

#[no_mangle]
pub fn toxoid_entity_get_name(id: i32) {
    unsafe {
        let world = *flecs_core::WORLD;
        let tag_name = flecs_core::bindings::ecs_get_name(world, id as u64);

        // Convert to Rust string
        let tag_name = std::ffi::CStr::from_ptr(tag_name as *const i8);
        let tag_name = tag_name.to_string_lossy().into_owned();
        println!("Found entity name: {:?}", tag_name);
    }
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_register_tag(name: *const i8, name_len: usize) -> ecs_entity_t {
    // Convert the C String to a Rust string using a specific length
    // to deal with FFI memory issues
    let slice = std::slice::from_raw_parts(name as *mut u8, name_len);
    let rust_string = std::str::from_utf8_unchecked(slice);

    // Convert back to C string with specific length
    let c_string = std::ffi::CString::new(rust_string).expect("Failed to convert to CString");
    flecs_core::flecs_tag_create(c_string.as_ptr())
}

// Have to define high level workaround to maintain context in toxoid_api_macro
#[no_mangle]
pub unsafe extern "C" fn register_component_ecs(
    name: &str,
    member_names: &[&str],
    member_types: &[u8],
) -> SplitU64 {
    unsafe {
        if member_names.len() == 0 {
            let entity = toxoid_register_tag(
                name.as_bytes().as_ptr() as *const c_char, 
                name.len() as usize
            );
            split_u64(entity)
        } else {
            let member_names_layout = Layout::array::<*mut c_char>(member_names.len() as usize).unwrap();
            let member_names_ptr = host_alloc(member_names_layout) as *mut *mut c_char;
            let member_names_len_layout = Layout::array::<u8>(member_names.len() as usize).unwrap();
            let member_names_len_ptr = host_alloc(member_names_len_layout) as *mut u8;
            member_names
                .iter()
                .enumerate()
                .for_each(|(i, &member_name)| {
                    member_names_ptr.add(i).write(member_name.as_ptr() as *mut i8);
                    member_names_len_ptr.add(i).write(member_name.len() as u8);
                });

            let member_types_layout = Layout::array::<u8>(member_types.len() as usize).unwrap();
            let member_types_ptr = host_alloc(member_types_layout) as *mut u8;
            member_types
                .iter()
                .enumerate()
                .for_each(|(i, &member_type)| {
                    member_types_ptr.add(i).write(member_type);
                });

            let entity = toxoid_register_component(
                name.as_bytes().as_ptr() as *const c_char,
                name.len() as u8,
                member_names_ptr as *const *const c_char,
                member_names.len() as u32,
                member_names_len_ptr,
                member_types_ptr,
                member_types.len() as u32,
            );
            split_u64(entity)
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_register_component(
    component_name: *const c_char,
    component_name_len: u8,
    member_names: *const *const c_char,
    member_names_count: u32,
    member_names_len: *const u8,
    member_types: *const u8,
    member_types_count: u32,
) -> u64 {
    // Convert the C String to a Rust string using a specific length
    // to deal with FFI memory issues
    let component_name_slice =
        std::slice::from_raw_parts(component_name as *mut u8, component_name_len as usize);
    let component_name = std::str::from_utf8_unchecked(component_name_slice);
    // Convert back to C string with specific length
    let component_name =
        std::ffi::CString::new(component_name).expect("Failed to convert to CString");

    // Convert the C String to a Rust string using a specific length
    // for all member names
    let member_names_slice = std::slice::from_raw_parts(member_names as *mut *mut c_char, member_names_count as usize);
    let member_names_len_slice = std::slice::from_raw_parts(member_names_len as *mut u8, member_names_count as usize);
    let member_names = member_names_slice
        .iter()
        .enumerate()
        .map(|(i, member_name_ptr)| {
            // Convert the C String to a Rust string using a specific length
            let member_name_slice = std::slice::from_raw_parts(*member_name_ptr as *mut u8, member_names_len_slice[i] as usize);
            let member_name_str = std::str::from_utf8_unchecked(member_name_slice);
            // Convert back to C string with specific length and collect CStrings to avoid dropping
            std::ffi::CString::new(member_name_str).expect("Failed to convert to CString")
        })
        .collect::<Vec<std::ffi::CString>>();
    let member_names_pointers: Vec<_> = member_names.iter().map(|c_string| c_string.as_ptr()).collect();

    let created = flecs_core::flecs_component_create(
        component_name.as_ptr(),
        member_names_pointers.as_ptr(),
        member_names_count,
        member_types,
        member_types_count,
    );
    created
}

#[cfg(target_arch="wasm32")]
#[no_mangle]
pub unsafe extern "C" fn toxoid_entity_create() -> SplitU64 {
    let entity = flecs_core::flecs_entity_create();
    split_u64(entity)
}

#[cfg(not(target_arch="wasm32"))]
#[no_mangle]
pub unsafe extern "C" fn toxoid_entity_create() -> u64 {
    flecs_core::flecs_entity_create()
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_entity_add_component(entity: ecs_entity_t, component: ecs_entity_t) {
    flecs_core::flecs_entity_add_component(entity, component)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_entity_add_tag(entity: ecs_entity_t, tag: ecs_entity_t) {
    flecs_core::flecs_entity_add_tag(entity, tag)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_entity_child_of(parent: ecs_entity_t, child: ecs_entity_t) {
    flecs_core::flecs_entity_child_of(parent, child)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_entity_children(parent: ecs_entity_t) -> *mut ecs_iter_t {
    flecs_core::flecs_entity_children(parent)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_child_entities(iter: *mut ecs_iter_t) -> *mut u64 {
    flecs_core::flecs_child_entities(iter)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_term_next(iter: *mut flecs_core::ecs_iter_t) -> bool {
    flecs_core::flecs_term_next(iter)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_query_create() -> *mut flecs_core::ecs_query_desc_t {
    flecs_core::flecs_query_create()
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_query_with(query_desc: *mut flecs_core::ecs_query_desc_t, filter_index: u8, ids: *mut ecs_entity_t, components_count: i32) -> u8 {
    flecs_core::flecs_query_with(query_desc, filter_index, ids, components_count)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_query_without(query_desc: *mut flecs_core::ecs_query_desc_t, filter_index: u8, ids: *mut ecs_entity_t, components_count: i32) -> u8 {
    flecs_core::flecs_query_without(query_desc, filter_index, ids, components_count)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_query_with_or(query_desc: *mut flecs_core::ecs_query_desc_t, filter_index: u8, ids: *mut ecs_entity_t, components_count: i32) -> u8 {
    flecs_core::flecs_query_with_or(query_desc, filter_index, ids, components_count)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_query_build(query_desc: *mut flecs_core::ecs_query_desc_t) -> *mut flecs_core::ecs_query_t {
    flecs_core::flecs_query_build(query_desc)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_query_iter(
    query: *mut flecs_core::ecs_query_t,
) -> *mut flecs_core::ecs_iter_t {
    flecs_core::flecs_query_iter(query)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_query_next(iter: *mut flecs_core::ecs_iter_t) -> bool {
    flecs_core::flecs_query_next(iter)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_iter_count(iter: *mut flecs_core::ecs_iter_t) -> i32 {
    flecs_core::flecs_iter_count(iter)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_iter_entity_id(iter: *mut flecs_core::ecs_iter_t) -> i32 {
    flecs_core::flecs_iter_count(iter)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_query_field(
    iter: *mut flecs_core::ecs_iter_t,
    term_index: i32,
    count: u32,
    index: u32,
) -> *const c_void {
    flecs_core::flecs_query_field(iter, term_index, count, index)
}

// Function to convert your *mut u64 to a &[u64]
pub unsafe extern "C" fn to_u64_slice(ptr: *mut u64, len: usize) -> &'static [u64] {
    let slice = core::slice::from_raw_parts(ptr, len);
    slice
}

#[repr(C)]
pub struct EntityId {
    id: ecs_entity_t,
    children: &'static mut [EntityId]
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_query_entity_list(iter: *mut flecs_core::ecs_iter_t) -> &'static [EntityId] {
    let count = toxoid_iter_count(iter) as usize;
    let ptr = flecs_core::flecs_query_entity_list(iter) as *mut ecs_entity_t;
    let slice: &[ecs_entity_t] = core::slice::from_raw_parts(ptr, count);

    // Create a Vec<Entity> from the slice of entity IDs
    // grabbed raw from the flecs API
    let mut entities_vec: Vec<EntityId> = Vec::with_capacity(count);
    for &id in slice.iter() {
        entities_vec.push(EntityId { id, children: &mut [] });
    }
    // Here, Box::leak(entities_vec.into_boxed_slice()) creates a leak, intentionally not freeing the memory.
    // This is generally a bad practice, but sometimes it can be useful when interfacing with C or for certain kinds of low-level programming.
    Box::leak(entities_vec.into_boxed_slice()) as &'static [EntityId]
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_entity_get_component(entity: ecs_entity_t, component: ecs_entity_t) -> *mut c_void {
    flecs_core::flecs_entity_get_component(entity, component)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_cache_insert(
    type_hash: SplitU64,
    component_id: SplitU64
) {
    let mut cache = COMPONENT_ID_CACHE.lock().unwrap();
    let type_hash = combine_u32(type_hash);
    let component_id = combine_u32(component_id);
    cache.insert(type_hash, component_id);
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_cache_get(type_hash: SplitU64) -> SplitU64 {
    let cache = COMPONENT_ID_CACHE.lock().unwrap();
    let type_hash = combine_u32(type_hash);
    let component_id = *cache.get(&type_hash).unwrap_or(&0);
    split_u64(component_id)
}

#[cfg(target_arch="wasm32")]
#[no_mangle]
pub unsafe extern "C" fn toxoid_network_entity_cache_insert(
    entity_id: SplitU64,
    network_id: SplitU64
) {
    let mut cache = NETWORK_ENTITY_CACHE.lock().unwrap();
    let entity_id = combine_u32(entity_id);
    let network_id = combine_u32(network_id);
    cache.insert(entity_id, network_id);
}

#[cfg(not(target_arch="wasm32"))]
#[no_mangle]
pub unsafe extern "C" fn toxoid_network_entity_cache_insert(
    entity_id: u64,
    network_id: u64
) {
    let mut cache = NETWORK_ENTITY_CACHE.lock().unwrap();
    cache.insert(entity_id, network_id);
}

#[cfg(target_arch="wasm32")]
#[no_mangle]
pub unsafe extern "C" fn toxoid_network_entity_cache_get(entity_id: SplitU64) -> SplitU64 {
    let cache = NETWORK_ENTITY_CACHE.lock().unwrap();
    let entity_id = combine_u32(entity_id);
    let network_id = *cache.get(&entity_id).unwrap_or_else(|| {
        println!("Entity ID not found in network entity cache: {:?}", entity_id);
        println!("Cache contents: {:?}", cache);
        &0
    });
    split_u64(network_id)
}

#[cfg(not(target_arch="wasm32"))]
#[no_mangle]
pub unsafe extern "C" fn toxoid_network_entity_cache_get(entity_id: u64) -> u64 {
    let cache = NETWORK_ENTITY_CACHE.lock().unwrap();
    let network_id = *cache.get(&entity_id).unwrap_or_else(|| {
        println!("Entity ID not found in network entity cache: {:?}", entity_id);
        &0
    });
    network_id
}

#[cfg(target_arch="wasm32")]
#[no_mangle]
pub unsafe extern "C" fn toxoid_network_entity_cache_remove(entity_id: SplitU64) {
    let mut cache = NETWORK_ENTITY_CACHE.lock().unwrap();
    let entity_id = combine_u32(entity_id);
    cache.remove(&entity_id);
}

#[cfg(not(target_arch="wasm32"))]
#[no_mangle]
pub unsafe extern "C" fn toxoid_network_entity_cache_remove(entity_id: u64) {
    let mut cache = NETWORK_ENTITY_CACHE.lock().unwrap();
    cache.remove(&entity_id);
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_create_vec() -> *mut c_void {
    Box::into_raw(Box::new(Vec::<*mut c_void>::new())) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_vec_push(ptr: *mut c_void, value: *mut c_void) {
    let vec: &mut Vec<*mut c_void> = &mut *(ptr as *mut Vec<*mut c_void>);
    vec.push(value);
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_vec_drop(ptr: *mut c_void) {
    let _box: Box<Vec<*mut c_void>> = Box::from_raw(ptr as *mut Vec<*mut c_void>);
    // Dropping the box, and hence the Vec.
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_vec_as_slice(ptr: *mut c_void) -> (*const *mut c_void, i32) {
    let vec: &mut Vec<*mut c_void> = &mut *(ptr as *mut Vec<*mut c_void>);
    (vec.as_ptr(), vec.len() as i32)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_query_field_size(
    iter: *mut flecs_core::ecs_iter_t,
    term_index: i32
) -> usize {
    flecs_core::flecs_query_field_size(iter, term_index)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_query_field_list(
    iter: *mut flecs_core::ecs_iter_t,
    term_index: i32,
    count: u32,
) -> *mut [*const c_void] {
    flecs_core::flecs_query_field_list(iter, term_index, count)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_get_member_u8(
    component_ptr: *mut c_void,
    offset: u32
) -> u8 {
    flecs_core::flecs_component_get_member_u8(component_ptr, offset)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_get_member_u16(
    component_ptr: *mut c_void,
    offset: u32
) -> u16 {
    flecs_core::flecs_component_get_member_u16(component_ptr, offset)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_get_member_u32(
    component_ptr: *mut c_void,
    offset: u32
) -> u32 {
    flecs_core::flecs_component_get_member_u32(component_ptr, offset)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_get_member_u64(
    component_ptr: *mut c_void,
    offset: u32
) -> SplitU64 {
    split_u64(flecs_core::flecs_component_get_member_u64(component_ptr, offset))
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_get_member_i8(
    component_ptr: *mut c_void,
    offset: u32
) -> i8 {
    flecs_core::flecs_component_get_member_i8(component_ptr, offset)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_get_member_i16(
    component_ptr: *mut c_void,
    offset: u32
) -> i16 {
    flecs_core::flecs_component_get_member_i16(component_ptr, offset)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_get_member_i32(
    component_ptr: *mut c_void,
    offset: u32
) -> i32 {
    flecs_core::flecs_component_get_member_i32(component_ptr, offset)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_get_member_i64(
    component_ptr: *mut c_void,
    offset: u32
) -> i64 {
    flecs_core::flecs_component_get_member_i64(component_ptr, offset)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_get_member_f32(
    component_ptr: *mut c_void,
    offset: u32
) -> f32 {
    flecs_core::flecs_component_get_member_f32(component_ptr, offset)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_get_member_f64(
    component_ptr: *mut c_void,
    offset: u32
) -> f64 {
    flecs_core::flecs_component_get_member_f64(component_ptr, offset)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_get_member_bool(
    component_ptr: *mut c_void,
    offset: u32
) -> bool {
    flecs_core::flecs_component_get_member_bool(component_ptr, offset)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_get_member_string(
    component_ptr: *mut c_void,
    offset: u32
) -> *mut i8 {
    flecs_core::flecs_component_get_member_string(component_ptr, offset)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_get_member_u32array(
    component_ptr: *mut c_void,
    offset: u32
) -> *mut u32 {
    flecs_core::flecs_component_get_member_u32array(component_ptr, offset)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_get_member_f32array(
    component_ptr: *mut c_void,
    offset: u32
) -> *mut f32 {
    flecs_core::flecs_component_get_member_f32array(component_ptr, offset)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_set_member_u8(
    component_ptr: *mut c_void,
    offset: u32,
    value: u8,
) {
    flecs_core::flecs_component_set_member_u8(component_ptr, offset, value);
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_set_member_u16(
    component_ptr: *mut c_void,
    offset: u32,
    value: u16,
) {
    flecs_core::flecs_component_set_member_u16(component_ptr, offset, value);
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_set_member_u32(
    component_ptr: *mut c_void,
    offset: u32,
    value: u32,
) {
    flecs_core::flecs_component_set_member_u32(component_ptr, offset, value);
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_set_member_u64(
    component_ptr: *mut c_void,
    offset: u32,
    value: SplitU64,
) {
    flecs_core::flecs_component_set_member_u64(component_ptr, offset, combine_u32(value));
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_set_member_i8(
    component_ptr: *mut c_void,
    offset: u32,
    value: i8,
) {
    flecs_core::flecs_component_set_member_i8(component_ptr, offset, value);
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_set_member_i16(
    component_ptr: *mut c_void,
    offset: u32,
    value: i16,
) {
    flecs_core::flecs_component_set_member_i16(component_ptr, offset, value);
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_set_member_i32(
    component_ptr: *mut c_void,
    offset: u32,
    value: i32,
) {
    flecs_core::flecs_component_set_member_i32(component_ptr, offset, value);
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_set_member_i64(
    component_ptr: *mut c_void,
    offset: u32,
    value: i64,
) {
    flecs_core::flecs_component_set_member_i64(component_ptr, offset, value);
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_set_member_f32(
    component_ptr: *mut c_void,
    offset: u32,
    value: f32,
) {
    flecs_core::flecs_component_set_member_f32(component_ptr, offset, value);
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_set_member_f64(
    component_ptr: *mut c_void,
    offset: u32,
    value: f64,
) {
    flecs_core::flecs_component_set_member_f64(component_ptr, offset, value);
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_set_member_bool(
    component_ptr: *mut c_void,
    offset: u32,
    value: bool,
) {
    flecs_core::flecs_component_set_member_bool(component_ptr, offset, value);
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_set_member_string(
    component_ptr: *mut c_void,
    offset: u32,
    value: *mut c_char,
) {
    flecs_core::flecs_component_set_member_string(component_ptr, offset, value);
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_set_member_u32array(
    component_ptr: *mut c_void,
    offset: u32,
    value: *mut u32,
) {
    flecs_core::flecs_component_set_member_u32array(component_ptr, offset, value);
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_set_member_f32array(
    component_ptr: *mut c_void,
    offset: u32,
    value: *mut f32,
) {
    flecs_core::flecs_component_set_member_f32array(component_ptr, offset, value);
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_progress(delta_time: f32) -> bool {
    flecs_core::flecs_progress(delta_time)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_filter_children_init(
    parent: ecs_entity_t
) -> *mut flecs_core::ecs_filter_t {
    flecs_core::flecs_filter_children_init(parent.into())
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_filter_iter(
    filter: *mut flecs_core::ecs_filter_t
) -> *mut flecs_core::ecs_iter_t {
    flecs_core::flecs_filter_iter(filter)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_iter_entities(
    iter: *mut flecs_core::ecs_iter_t
) -> &'static [u64] {
    flecs_core::flecs_iter_entities(iter)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_delete_entity(
    entity: ecs_entity_t
) {
    flecs_core::flecs_delete_entity(entity)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_entity_remove_component(
    entity: ecs_entity_t,
    component: ecs_entity_t
) {
    flecs_core::flecs_entity_remove_component(entity, component)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_filter_next(
    iter: *mut flecs_core::ecs_iter_t
) -> bool {
    flecs_core::flecs_filter_next(iter)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_is_valid(
    entity: ecs_entity_t
) -> bool {
    flecs_core::flecs_is_valid(entity)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_entity_has_component(
    entity: ecs_entity_t,
    component: ecs_entity_t
) -> bool {
    flecs_core::flecs_entity_has_component(entity, component)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_singleton_add(
    component: ecs_entity_t
) {
    flecs_core::flecs_singleton_add(component)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_singleton_get(
    component: ecs_entity_t
) -> *mut c_void {
    flecs_core::flecs_singleton_get(component)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_singleton_remove(
    component: ecs_entity_t
) {
    flecs_core::flecs_singleton_remove(component)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_set_member_ptr(
    component_ptr: *mut c_void,
    offset: u32,
    value: *mut c_void,
) {
    flecs_core::flecs_component_set_member_ptr(component_ptr, offset, value as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_get_member_ptr(
    component_ptr: *mut c_void,
    offset: u32,
) -> *mut c_void {
    flecs_core::flecs_component_get_member_ptr(component_ptr, offset)
}


#[no_mangle]
pub unsafe extern "C" fn toxoid_filter_create() -> *mut flecs_core::ecs_filter_desc_t {
    flecs_core::flecs_filter_create()    
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_filter_with(filter_desc: *mut flecs_core::ecs_filter_desc_t, filter_index: u8, ids: *mut ecs_entity_t, components_count: i32) -> u8 {
    flecs_core::flecs_filter_with(filter_desc, filter_index, ids, components_count)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_filter_without(filter_desc: *mut flecs_core::ecs_filter_desc_t, filter_index: u8, ids: *mut ecs_entity_t, components_count: i32) -> u8 {
    flecs_core::flecs_filter_without(filter_desc, filter_index, ids, components_count)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_filter_with_or(filter_desc: *mut flecs_core::ecs_filter_desc_t, filter_index: u8, ids: *mut ecs_entity_t, components_count: i32) -> u8 {
    flecs_core::flecs_filter_with_or(filter_desc, filter_index, ids, components_count)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_filter_build(desc: *mut flecs_core::ecs_filter_desc_t) -> *mut flecs_core::ecs_filter_t {
    flecs_core::flecs_filter_build(desc)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_filter_iter_component(
    component_array_ptr: *mut u8,
    component_index: u32,
    count: u32,
    component_id: ecs_entity_t,
) -> *const u8 {
    flecs_core::flecs_filter_iter_component(component_array_ptr, component_index, count, component_id)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_filter_field(
    iter: *mut flecs_core::ecs_iter_t,
    term_index: i32,
    count: u32,
    index: u32,
) -> *const c_void {
    flecs_core::flecs_filter_field(iter, term_index, count, index)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_filter_field_size(
    iter: *mut flecs_core::ecs_iter_t,
    term_index: i32,
) -> usize {
    flecs_core::flecs_filter_field_size(iter, term_index)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_filter_field_list(
    iter: *mut flecs_core::ecs_iter_t,
    term_index: i32,
    count: u32
) -> *mut [*const c_void] {
    flecs_core::flecs_filter_field_list(iter, term_index, count)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_filter_entity(iter: *mut flecs_core::ecs_iter_t, count: u32, index: u32) -> ecs_entity_t {
    flecs_core::flecs_filter_entity(iter, count, index)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_filter_entity_list(iter: *mut flecs_core::ecs_iter_t) -> *mut ecs_entity_t {
    flecs_core::flecs_filter_entity_list(iter)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_prefab_create() -> SplitU64 {
    split_u64(flecs_core::flecs_prefab_create())
}

// Does not accept SplitU64 -> SplitU64 on Emscripten because of weird linker bug
// Use tools like nm, objdump, or readelf to inspect the symbols and their signatures in the compiled object files.
// Look at the generated WebAssembly text format (.wat file) or bytecode (.wasm file) to verify the exported function signatures.
#[no_mangle]
pub unsafe extern "C" fn toxoid_prefab_instance(prefab_high: u32, prefab_low: u32) -> SplitU64 {
    split_u64(flecs_core::flecs_prefab_instance(combine_u32(SplitU64 { high: prefab_high, low: prefab_low })))
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_system_create(
    callback_closure: fn(*mut c_void)
) -> *mut flecs_core::ecs_system_desc_t {
    flecs_core::flecs_system_create(callback_closure)
}

#[cfg(target_arch="wasm32")]
#[no_mangle]
pub unsafe extern "C" fn toxoid_system_build(
    system_desc: *mut flecs_core::ecs_system_desc_t,
) -> SplitU64 {
    split_u64(flecs_core::flecs_system_build(system_desc))
}

#[cfg(not(target_arch="wasm32"))]
#[no_mangle]
pub unsafe extern "C" fn toxoid_system_build(
    system_desc: *mut flecs_core::ecs_system_desc_t,
) -> ecs_entity_t {
    flecs_core::flecs_system_build(system_desc)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_query_from_system_desc(
    system_desc: *mut flecs_core::ecs_system_desc_t
) -> *mut flecs_core::ecs_query_desc_t {
    flecs_core::flecs_query_from_system_desc(system_desc)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_lookup(name: *mut c_char) -> SplitU64 {
    split_u64(flecs_core::flecs_component_lookup(name))
}

// Map deserialized entities to local entity component system and synchronize entities
#[no_mangle]
pub unsafe extern "C" fn toxoid_deserialize_entity_sync(entity_id: ecs_entity_t, components_serialized: &[toxoid_api::MessageComponent]) {
    let world = *WORLD;
    components_serialized
        .iter()
        .for_each(|component_serialized| {
            let component_name = std::ffi::CString::new(component_serialized.name).unwrap();
            let component_id: ecs_entity_t = ecs_lookup(world, component_name.as_ptr());
            // let component_struct_ptr = ecs_get_mut_id(world, FLECS_IDEcsStructID_, component_id);
            let ecs_struct = ecs_get_id(world, component_id, FLECS_IDEcsStructID_) as *const EcsStruct;
            let members = (*ecs_struct).members;
            let component_data = component_serialized.data;
            let component_deserialized = flexbuffers::Reader::get_root(component_data).unwrap();
            let component_map = component_deserialized.as_map();
            // let keys: Vec<&str> = component_map.iter_keys().collect();
            let component_ptr = ecs_get_mut_id(world, entity_id, component_id);
            ecs_vector_each::<ecs_member_t, _>(&members, |item| {
                let name = core::ffi::CStr::from_ptr(item.name as *const i8).to_str().unwrap();
                let value = component_map.idx(name);
                match item.type_ {
                    type_id if type_id == FLECS_IDecs_u8_tID_ => {
                        let value = value.as_u8();
                        // TODO: This is wrong, component_ptr is not the right pointer
                        // It needs to point to entity, not the component struct type
                        flecs_component_set_member_u8(component_ptr, item.offset as u32, value);
                    },
                    type_id if type_id == FLECS_IDecs_u16_tID_ => {
                        let value = value.as_u16();
                        flecs_component_set_member_u16(component_ptr, item.offset as u32, value);
                    },
                    type_id if type_id == FLECS_IDecs_u32_tID_ => {
                        let value = value.as_u32();
                        flecs_component_set_member_u32(component_ptr, item.offset as u32, value);
                    },
                    type_id if type_id == FLECS_IDecs_u64_tID_ => {
                        let value = value.as_u64();
                        flecs_component_set_member_u64(component_ptr, item.offset as u32, value);
                    },
                    type_id if type_id == FLECS_IDecs_i8_tID_ => {
                        let value = value.as_i8();
                        flecs_component_set_member_i8(component_ptr, item.offset as u32, value);
                    },
                    type_id if type_id == FLECS_IDecs_i16_tID_ => {
                        let value = value.as_i16();
                        flecs_component_set_member_i16(component_ptr, item.offset as u32, value);
                    },
                    type_id if type_id == FLECS_IDecs_i32_tID_ => {
                        let value = value.as_i32();
                        flecs_component_set_member_i32(component_ptr, item.offset as u32, value);
                    },
                    type_id if type_id == FLECS_IDecs_i64_tID_ => {
                        let value = value.as_i64();
                        flecs_component_set_member_i64(component_ptr, item.offset as u32, value);
                    },
                    type_id if type_id == FLECS_IDecs_f32_tID_ => {
                        let value = value.as_f32();
                        flecs_component_set_member_f32(component_ptr, item.offset as u32, value);
                    },
                    type_id if type_id == FLECS_IDecs_f64_tID_ => {
                        let value = value.as_f64();
                        flecs_component_set_member_f64(component_ptr, item.offset as u32, value);
                    },
                    type_id if type_id == FLECS_IDecs_bool_tID_ => {
                        let value = value.as_bool();
                        flecs_component_set_member_bool(component_ptr, item.offset as u32, value);
                    },
                    _ => eprintln!("Type not supported {:?}", item.type_),
                }
            });
        });
}

// Deserialize entities and return a hashmap of entities and components
#[no_mangle]
pub unsafe extern "C" fn toxoid_deserialize_entity(components_serialized: &[toxoid_api::MessageComponent]) -> HashMap<std::string::String, HashMap<std::string::String, DynamicType>> {
    let world = *WORLD;
    let mut components_hashmap: HashMap<String, HashMap<String, DynamicType>> = HashMap::new();
    components_serialized
        .iter()
        .for_each(|component_serialized| {
            let component_name = std::ffi::CString::new(component_serialized.name.clone()).unwrap();
            let component_id: ecs_entity_t = ecs_lookup(world, component_name.as_ptr());
            // let component_struct_ptr = ecs_get_mut_id(world, FLECS_IDEcsStructID_, component_id);
            let ecs_struct = ecs_get_id(world, component_id, FLECS_IDEcsStructID_) as *const EcsStruct;
            let members = (*ecs_struct).members;
            let component_data = component_serialized.data.clone();
            let component_deserialized = flexbuffers::Reader::get_root(component_data).unwrap();
            let component_map = component_deserialized.as_map();
            // let keys: Vec<&str> = component_map.iter_keys().collect();
            let mut component_hashmap: HashMap<String, DynamicType> = HashMap::new();
            ecs_vector_each::<ecs_member_t, _>(&members, |item| {
                let name = core::ffi::CStr::from_ptr(item.name as *const i8).to_str().unwrap();
                let value = component_map.idx(name);
                let name = name.to_string();
                match item.type_ {
                    type_id if type_id == FLECS_IDecs_u8_tID_ => {
                        let value = DynamicType::U8(value.as_u8());
                        component_hashmap.insert(name, value);
                    },
                    type_id if type_id == FLECS_IDecs_u16_tID_ => {
                        let value = DynamicType::U16(value.as_u16());
                        component_hashmap.insert(name, value);
                    },
                    type_id if type_id == FLECS_IDecs_u32_tID_ => {
                        let value = DynamicType::U32(value.as_u32());
                        component_hashmap.insert(name, value);
                    },
                    type_id if type_id == FLECS_IDecs_u64_tID_ => {
                        let value = DynamicType::U64(value.as_u64());
                        component_hashmap.insert(name, value);
                    },
                    type_id if type_id == FLECS_IDecs_i8_tID_ => {
                        let value = DynamicType::I8(value.as_i8());
                        component_hashmap.insert(name, value);
                    },
                    type_id if type_id == FLECS_IDecs_i16_tID_ => {
                        let value = DynamicType::I16(value.as_i16());
                        component_hashmap.insert(name, value);
                    },
                    type_id if type_id == FLECS_IDecs_i32_tID_ => {
                        let value = DynamicType::I32(value.as_i32());
                        component_hashmap.insert(name, value);
                    },
                    type_id if type_id == FLECS_IDecs_i64_tID_ => {
                        let value = DynamicType::I64(value.as_i64());
                        component_hashmap.insert(name, value);
                    },
                    type_id if type_id == FLECS_IDecs_f32_tID_ => {
                        let value = DynamicType::F32(value.as_f32());
                        component_hashmap.insert(name, value);
                    },
                    type_id if type_id == FLECS_IDecs_f64_tID_ => {
                        let value = DynamicType::F64(value.as_f64());
                        component_hashmap.insert(name, value);
                    },
                    type_id if type_id == FLECS_IDecs_bool_tID_ => {
                        let value = DynamicType::Bool(value.as_bool());
                        component_hashmap.insert(name, value);
                    },
                    _ => eprintln!("Type not supported {:?}", item.type_),
                }
            });
            components_hashmap.insert(component_serialized.name.to_string(), component_hashmap);
        });
    components_hashmap
}

#[no_mangle]
pub unsafe fn toxoid_serialize_component(entity_id: ecs_entity_t, component_id: ecs_entity_t) -> toxoid_serialize::NetworkMessageComponent {
    let world = *WORLD;
    // Get the component name
    let component_name = ecs_get_name(world, component_id);
    let component_name = core::ffi::CStr::from_ptr(component_name).to_str().unwrap();
    // Get the component struct pointer
    // let component_struct_ptr = ecs_get_mut_id(world, FLECS_IDEcsStructID_, component_id);
    // Get the component type
    let ecs_struct = ecs_get_id(world, component_id, FLECS_IDEcsStructID_) as *const EcsStruct;
    // Get the members of the component type
    let members = (*ecs_struct).members;
    // Create a new Flexbuffer builder.
    let mut builder = flexbuffers::Builder::default();
    // Start a map
    let mut component_serialized = builder.start_map();
    let component_ptr = ecs_get_mut_id(world, entity_id, component_id);
    ecs_vector_each::<ecs_member_t, _>(&members, |item| {
        // Convert from *const i8 to &str
        let name = core::ffi::CStr::from_ptr(item.name as *const i8).to_str().unwrap();
        match item.type_ {
            type_id if type_id == FLECS_IDecs_u8_tID_ => {
                let value = flecs_component_get_member_u8(component_ptr, item.offset as u32);
                component_serialized.push(name, value);
            },
            type_id if type_id == FLECS_IDecs_u16_tID_ => {
                let value = flecs_component_get_member_u16(component_ptr, item.offset as u32);
                component_serialized.push(name, value);
            },
            type_id if type_id == FLECS_IDecs_u32_tID_ => {
                let value = flecs_component_get_member_u32(component_ptr, item.offset as u32);
                component_serialized.push(name, value);
            },
            type_id if type_id == FLECS_IDecs_u64_tID_ => {
                let value = flecs_component_get_member_u64(component_ptr, item.offset as u32);
                component_serialized.push(name, value);
            },
            type_id if type_id == FLECS_IDecs_i8_tID_ => {
                let value = flecs_component_get_member_i8(component_ptr, item.offset as u32);
                component_serialized.push(name, value);
            },
            type_id if type_id == FLECS_IDecs_i16_tID_ => {
                let value = flecs_component_get_member_i16(component_ptr, item.offset as u32);
                component_serialized.push(name, value);
            },
            type_id if type_id == FLECS_IDecs_i32_tID_ => {
                let value = flecs_component_get_member_i32(component_ptr, item.offset as u32);
                component_serialized.push(name, value);
            },
            type_id if type_id == FLECS_IDecs_i64_tID_ => {
                let value = flecs_component_get_member_i64(component_ptr, item.offset as u32);
                component_serialized.push(name, value);
            },
            type_id if type_id == FLECS_IDecs_f32_tID_ => {
                let value = flecs_component_get_member_f32(component_ptr, item.offset as u32);
                component_serialized.push(name, value);
            },
            type_id if type_id == FLECS_IDecs_f64_tID_ => {
                let value = flecs_component_get_member_f64(component_ptr, item.offset as u32);
                component_serialized.push(name, value);
            },
            type_id if type_id == FLECS_IDecs_bool_tID_ => {
                let value = flecs_component_get_member_bool(component_ptr, item.offset as u32);
                component_serialized.push(name, value);
            },
            _ => eprintln!("Type not supported {:?}", item.type_),
        }
    });
    component_serialized.end_map();
    let component_data = builder.view().to_vec();
    toxoid_serialize::NetworkMessageComponent {
        name: component_name.to_string(),
        data: component_data,
    }
}

#[no_mangle]
pub unsafe fn toxoid_serialize_entity(entity_id: ecs_entity_t) -> Vec<toxoid_serialize::NetworkMessageComponent> {
    let world = *WORLD;
    // Network components
    let mut network_components = Vec::new();
    // Get the entity type
    let entity_type: *const ecs_type_t  = ecs_get_type(world, entity_id);
    // Get the type ids
    let type_ids = (*entity_type).array;
    let type_ids = std::slice::from_raw_parts(type_ids, (*entity_type).count as usize);
    // Iterate over the type ids
    for i in 0..(*entity_type).count {
        // Create a new Flexbuffer builder.
        let mut builder = flexbuffers::Builder::default();
        // Start a map
        builder.start_map();

        // Get the component id
        let component_id = type_ids[i as usize];
        let network_message_component = toxoid_serialize_component(entity_id, component_id);
        network_components.push(network_message_component);
    }
    network_components
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_entity_to_json(entity: ecs_entity_t) -> *mut c_char {
    flecs_entity_to_json(entity)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_json_to_entity(json: *mut c_char) {
    let world = *flecs_core::WORLD;
    let json = core::ffi::CStr::from_ptr(json).to_str().unwrap();
    let json_entity = toxoid_json::parse_entity(json);
    let entity = toxoid_api::Entity::new();
    entity.set_name(json_entity.label.as_str());
    json_entity
        .ids
        .iter()
        .zip(json_entity.values)
        .for_each(|(id, value)| {
            let id = id.get(0).unwrap();
            let component_name = std::ffi::CString::new(id.clone()).unwrap();
            let component: ecs_entity_t = ecs_lookup(world, component_name.as_ptr());
            let ecs_struct = ecs_get_id(world, component, FLECS_IDEcsStructID_) as *const EcsStruct;
            let members = (*ecs_struct).members;
            ecs_add_id(world, entity.get_id(), component);
            if value.is_number() {
                return
            } else {
                let component_ptr = ecs_get_mut_id(world, entity.get_id(), component);
                ecs_vector_each::<ecs_member_t, _>(&members, |item| {
                    // println!("Member name: {:?}", core::ffi::CStr::from_ptr(item.name).to_str().unwrap());
                    // println!("Member value from JSON {:?}", value);
                    let member_name = core::ffi::CStr::from_ptr(item.name).to_str().unwrap();
                    let value = value.get(member_name).unwrap();
                    match item.type_ {
                        type_id if type_id == FLECS_IDecs_u8_tID_ => {
                            let value = value.as_u64().unwrap() as u8;
                            flecs_component_set_member_u8(component_ptr, item.offset as u32, value);
                        },
                        type_id if type_id == FLECS_IDecs_u16_tID_ => {
                            let value = value.as_u64().unwrap() as u16;
                            flecs_component_set_member_u16(component_ptr, item.offset as u32, value);
                        },
                        type_id if type_id == FLECS_IDecs_u32_tID_ => {
                            let value = value.as_u64().unwrap() as u32;
                            flecs_component_set_member_u32(component_ptr, item.offset as u32, value);
                        },
                        type_id if type_id == FLECS_IDecs_u64_tID_ => {
                            let value = value.as_u64().unwrap();
                            flecs_component_set_member_u64(component_ptr, item.offset as u32, value);
                        },
                        type_id if type_id == FLECS_IDecs_i8_tID_ => {
                            let value = value.as_i64().unwrap() as i8;
                            flecs_component_set_member_i8(component_ptr, item.offset as u32, value);
                        },
                        type_id if type_id == FLECS_IDecs_i16_tID_ => {
                            let value = value.as_i64().unwrap() as i16;
                            flecs_component_set_member_i16(component_ptr, item.offset as u32, value);
                        },
                        type_id if type_id == FLECS_IDecs_i32_tID_ => {
                            let value = value.as_i64().unwrap() as i32;
                            flecs_component_set_member_i32(component_ptr, item.offset as u32, value);
                        },
                        type_id if type_id == FLECS_IDecs_i64_tID_ => {
                            let value = value.as_i64().unwrap();
                            flecs_component_set_member_i64(component_ptr, item.offset as u32, value);
                        },
                        type_id if type_id == FLECS_IDecs_f32_tID_ => {
                            let value = value.as_f64().unwrap() as f32;
                            flecs_component_set_member_f32(component_ptr, item.offset as u32, value);
                        },
                        type_id if type_id == FLECS_IDecs_f64_tID_ => {
                            let value = value.as_f64().unwrap();
                            flecs_component_set_member_f64(component_ptr, item.offset as u32, value);
                        },
                        type_id if type_id == FLECS_IDecs_bool_tID_ => {
                            let value = value.as_bool().unwrap();
                            flecs_component_set_member_bool(component_ptr, item.offset as u32, value);
                        },
                        type_id if type_id == FLECS_IDecs_string_tID_ => {
                            let value = value.as_str().unwrap();
                            let value = std::ffi::CString::new(value).unwrap();
                            flecs_component_set_member_string(component_ptr, item.offset as u32, value.as_ptr() as *mut i8);
                        },
                        _ => eprintln!("Type not supported {:?}", item.type_),
                    }
                });
            }
        });
}

 #[no_mangle]
 pub unsafe extern "C" fn toxoid_entity_set_name(entity: ecs_entity_t, name: *mut c_char) {
    // Convert name to string
    let name = core::ffi::CStr::from_ptr(name).to_str().unwrap();
    // String interpolate uuid after name
    if name.is_empty() {
        let name = format!("{}", uuid::Uuid::new_v4());
        let name = std::ffi::CString::new(name).unwrap();
        flecs_entity_set_name(entity, name.as_ptr() as *mut i8);
    } else {
        let name = format!("{}-{}", name, uuid::Uuid::new_v4());
        let name = std::ffi::CString::new(name).unwrap();
        flecs_entity_set_name(entity, name.as_ptr() as *mut i8);
    }
 }

#[no_mangle]
pub extern "C" fn gen_uuid() -> *mut c_char {
    let uuid = uuid::Uuid::new_v4();
    let uuid_str = uuid.to_string();
    let c_str = std::ffi::CString::new(uuid_str).unwrap();
    c_str.into_raw()
}