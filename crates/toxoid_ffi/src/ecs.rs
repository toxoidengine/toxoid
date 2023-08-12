#![allow(improper_ctypes)]
#![allow(improper_ctypes_definitions)]

use core::ffi::{c_char, c_void};
use std::{collections::HashMap, cell::RefCell, any::TypeId};

thread_local! {
    pub static SYSTEMS: RefCell<Vec<toxoid_api::System>> = {
        let systems = Vec::new();
        RefCell::new(systems)
    };
    pub static COMPONENT_ID_CACHE: RefCell<HashMap<core::any::TypeId, i32>> = {
        let cache = HashMap::new();
        RefCell::new(cache)
    };
}

pub fn cache_component_ecs(type_id: TypeId, component_id: i32) {
    unsafe {
        toxoid_component_cache_insert(type_id, component_id);
    }
}

pub fn register_component_ecs(
    name: &str,
    member_names: &[&str],
    member_types: &[u8],
) -> toxoid_api::ecs_entity_t {
    println!("Register Component ECS from toxoid_ffi/src/ecs.rs");
    unsafe {
        let mut c_member_names: [*const c_char; 100] = [core::ptr::null(); 100];
        let mut c_member_names_len: [u8; 100] = [0; 100];
        for (i, &s) in member_names.iter().enumerate() {
            c_member_names[i] = s.as_ptr() as *const c_char;
            c_member_names_len[i] = s.len() as u8;
        }

        let mut c_member_types: [*const u8; 100] = [core::ptr::null(); 100];
        for (i, &t) in member_types.iter().enumerate() {
            c_member_types[i] = &t as *const u8;
        }

        toxoid_register_component(
            name.as_bytes().as_ptr() as *const c_char,
            name.len() as u8,
            c_member_names.as_ptr(),
            member_names.len() as u32,
            c_member_names_len.as_ptr(),
            c_member_types.as_ptr(),
            c_member_types.len() as u32,
        )
    }
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_print_i32(v: i32) {
    println!("Printing from Toxoid Engine: {}", v);
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
        let world = *flecs_core::WORLD.as_mut().unwrap_unchecked();
        let tag_name = flecs_core::bindings::ecs_get_name(world, id as u64);

        // Convert to Rust string
        let tag_name = std::ffi::CStr::from_ptr(tag_name as *const i8);
        let tag_name = tag_name.to_string_lossy().into_owned();
        println!("Found entity name: {:?}", tag_name);
    }
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_register_tag(name: *const i8, name_len: usize) -> i32 {
    // Convert the C String to a Rust string using a specific length
    // to deal with FFI memory issues
    let slice = std::slice::from_raw_parts(name as *mut u8, name_len);
    let rust_string = std::str::from_utf8_unchecked(slice);

    println!("Created tag named: {}", rust_string);

    // Convert back to C string with specific length
    let c_string = std::ffi::CString::new(rust_string).expect("Failed to convert to CString");
    flecs_core::flecs_tag_create(c_string.as_ptr()) as i32
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_register_component(
    component_name: *const c_char,
    component_name_len: u8,
    member_names: *const *const c_char,
    member_names_count: u32,
    member_names_len: *const u8,
    member_types: *const *const u8,
    member_types_count: u32,
) -> i32 {
    // Convert the C String to a Rust string using a specific length
    // to deal with FFI memory issues
    let component_name_slice =
        std::slice::from_raw_parts(component_name as *mut u8, component_name_len as usize);
    let component_name = std::str::from_utf8_unchecked(component_name_slice);
    // println!("Component Name: {}", component_name);

    // Convert back to C string with specific length
    let component_name =
        std::ffi::CString::new(component_name).expect("Failed to convert to CString");

    // Iterate over the member names 
    for i in 0..member_names_count {
        let _member_name_ptr = *member_names.add(i as usize);
        let _member_name_length = *member_names_len.add(i as usize);
        // let member_slice = std::slice::from_raw_parts(member_name_ptr as *mut u8, member_name_length as usize);
        // let member_name = std::str::from_utf8_unchecked(member_slice);
        // println!("Member Name #{}: {}", i, member_name);
    }

    let id = flecs_core::flecs_component_create(
        component_name.as_ptr(),
        member_names,
        member_names_count,
        member_types,
        member_types_count,
    ) as i32;

    println!("COMPONENT ID: {}", id);
    id
}

#[no_mangle]
pub unsafe fn toxoid_entity_create() -> i32 {
    flecs_core::flecs_entity_create() as i32
}

// TODO: Change i32 to u64 for Rust functions
// This is a limitation of JS where 
// you can't pass u64 to JS functions
// from emscripten
// #[no_mangle]
// pub unsafe fn toxoid_entity_create() -> flecs_core::ecs_entity_t {
//     flecs_core::flecs_entity_create()
// }

#[no_mangle]
pub unsafe fn toxoid_entity_add_component(entity: u32, component: u32) -> *mut c_void {
    flecs_core::flecs_entity_add_component(entity, component)
}

#[no_mangle]
pub unsafe fn toxoid_entity_add_tag(entity: u32, tag: u32) {
    flecs_core::flecs_entity_add_tag(entity, tag)
}

#[no_mangle]
pub unsafe fn toxoid_query_create(
    ids: *mut i32,
    components_count: i32,
) -> *mut flecs_core::ecs_query_t {
    flecs_core::flecs_query_create(ids, components_count)
}

#[no_mangle]
pub unsafe fn toxoid_query_iter(
    query: *mut flecs_core::ecs_query_t,
) -> *mut flecs_core::ecs_iter_t {
    flecs_core::flecs_query_iter(query)
}

#[no_mangle]
pub unsafe fn toxoid_query_next(iter: *mut flecs_core::ecs_iter_t) -> bool {
    flecs_core::flecs_query_next(iter)
}

#[no_mangle]
pub unsafe fn toxoid_iter_count(iter: *mut flecs_core::ecs_iter_t) -> i32 {
    flecs_core::flecs_iter_count(iter)
}

#[no_mangle]
pub unsafe fn toxoid_query_field(
    iter: *mut flecs_core::ecs_iter_t,
    term_index: i32,
    count: u32,
    index: u32,
) -> *const c_void {
    flecs_core::flecs_query_field(iter, term_index, count, index)
}

// Function to convert your *mut u64 to a &[u64]
pub unsafe fn to_u64_slice(ptr: *mut u64, len: usize) -> &'static [u64] {
    let slice = core::slice::from_raw_parts(ptr, len);
    slice
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EntityId {
    id: i32,
}

#[no_mangle]
pub unsafe fn toxoid_query_entity_list(iter: *mut flecs_core::ecs_iter_t) -> &'static [EntityId] {
    let count = toxoid_iter_count(iter) as usize;
    let ptr = flecs_core::flecs_query_entity_list(iter) as *mut u64;
    let slice: &[u64] = core::slice::from_raw_parts(ptr, count);
    // Create a Vec<Entity> from the slice of entity IDs
    // grabbed raw from the flecs API
    let mut entities_vec: Vec<EntityId> = Vec::with_capacity(count);
    for &id in slice.iter() {
        entities_vec.push(EntityId { id: id as i32 });
    }
    // Here, Box::leak(entities_vec.into_boxed_slice()) creates a leak, intentionally not freeing the memory.
    // This is generally a bad practice, but sometimes it can be useful when interfacing with C or for certain kinds of low-level programming.
    Box::leak(entities_vec.into_boxed_slice()) as &'static [EntityId]
}

#[no_mangle]
pub unsafe fn toxoid_entity_get_component(entity: u32, component: u32) -> *mut c_void {
    flecs_core::flecs_entity_get_component(entity, component)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_cache_insert(
    type_id: core::any::TypeId,
    component_id: i32,
) {
    COMPONENT_ID_CACHE.with(|c| {
        let mut cache = c.borrow_mut();
        cache.insert(type_id, component_id);
    });
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_cache_get(type_id: core::any::TypeId) -> i32 {
    COMPONENT_ID_CACHE.with(|c| {
        let cache = c.borrow_mut();
        *cache.get(&type_id).unwrap_or(&0)
    })
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
pub unsafe fn toxoid_query_field_size(
    iter: *mut flecs_core::ecs_iter_t,
    term_index: i32
) -> usize {
    flecs_core::flecs_query_field_size(iter, term_index)
}

#[no_mangle]
pub unsafe fn toxoid_query_field_list(
    iter: *mut flecs_core::ecs_iter_t,
    term_index: i32,
    count: u32,
) -> &'static mut [*const c_void] {
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
) -> u64 {
    flecs_core::flecs_component_get_member_u64(component_ptr, offset)
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
    value: u64,
) {
    flecs_core::flecs_component_set_member_u64(component_ptr, offset, value);
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
pub unsafe extern "C" fn toxoid_add_system(
    system: toxoid_api::System
) {
    SYSTEMS.with(|systems| {
        systems.borrow_mut().push(system);
    });
}