#![allow(improper_ctypes)]
#![allow(improper_ctypes_definitions)]

use core::ffi::{c_char, c_void};
use std::{collections::HashMap, cell::RefCell, any::TypeId};
use core::alloc::{GlobalAlloc, Layout};

use flecs_core::ecs_iter_t;
use toxoid_api::{ALLOCATOR, ecs_entity_t};

thread_local! {
    pub static SYSTEMS: RefCell<Vec<toxoid_api::System>> = {
        let systems = Vec::new();
        RefCell::new(systems)
    };
    pub static COMPONENT_ID_CACHE: RefCell<HashMap<core::any::TypeId, ecs_entity_t>> = {
        let cache = HashMap::new();
        RefCell::new(cache)
    };
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
        let world = flecs_core::WORLD.lock().unwrap().world.as_mut().unwrap();
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

    println!("Created tag named: {}", rust_string);

    // Convert back to C string with specific length
    let c_string = std::ffi::CString::new(rust_string).expect("Failed to convert to CString");
    flecs_core::flecs_tag_create(c_string.as_ptr())
}

#[repr(C)]
pub struct SplitU64 {
    high: u32,
    low: u32,
}

pub fn split_u64(value: u64) -> SplitU64 {
    SplitU64 {
        high: (value >> 32) as u32,
        low: (value & 0xFFFFFFFF) as u32,
    }
}

// Have to define high level workaround to maintain context in toxoid_api_macro
#[no_mangle]
pub unsafe extern "C" fn register_component_ecs(
    name: &str,
    member_names: &[&str],
    member_types: &[u8],
) -> SplitU64 {
    unsafe {
        let member_names_layout = Layout::array::<*mut c_char>(member_names.len() as usize).unwrap();
        let member_names_ptr = ALLOCATOR.alloc(member_names_layout) as *mut *mut c_char;
        let member_names_len_layout = Layout::array::<u8>(member_names.len() as usize).unwrap();
        let member_names_len_ptr = ALLOCATOR.alloc(member_names_len_layout) as *mut u8;
        member_names
            .iter()
            .enumerate()
            .for_each(|(i, &member_name)| {
                member_names_ptr.add(i).write(member_name.as_ptr() as *mut i8);
                member_names_len_ptr.add(i).write(member_name.len() as u8);
            });

        let member_types_layout = Layout::array::<u8>(member_types.len() as usize).unwrap();
        let member_types_ptr = ALLOCATOR.alloc(member_types_layout) as *mut u8;
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

    flecs_core::flecs_component_create(
        component_name.as_ptr(),
        member_names_pointers.as_ptr(),
        member_names_count,
        member_types,
        member_types_count,
    )
}

#[no_mangle]
pub unsafe fn toxoid_entity_create() -> u64 {
    flecs_core::flecs_entity_create()
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
pub unsafe fn toxoid_entity_add_component(entity: ecs_entity_t, component: ecs_entity_t){
    flecs_core::flecs_entity_add_component(entity, component)
}

#[no_mangle]
pub unsafe fn toxoid_entity_add_tag(entity: ecs_entity_t, tag: ecs_entity_t) {
    flecs_core::flecs_entity_add_tag(entity, tag)
}

#[no_mangle]
pub unsafe fn toxoid_entity_child_of(parent: ecs_entity_t, child: ecs_entity_t) {
    flecs_core::flecs_entity_child_of(parent, child)
}

#[no_mangle]
pub unsafe fn toxoid_entity_children(parent: ecs_entity_t) -> *mut ecs_iter_t {
    flecs_core::flecs_entity_children(parent)
}

#[no_mangle]
pub unsafe fn toxoid_child_entities(iter: *mut ecs_iter_t) -> *mut u64 {
    flecs_core::flecs_child_entities(iter)
}

#[no_mangle]
pub unsafe fn toxoid_term_next(iter: *mut flecs_core::ecs_iter_t) -> bool {
    flecs_core::flecs_term_next(iter)
}

#[no_mangle]
pub unsafe fn toxoid_query_create(
    ids: *mut ecs_entity_t,
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
pub unsafe fn toxoid_iter_entity_id(iter: *mut flecs_core::ecs_iter_t) -> i32 {
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
pub struct EntityId {
    id: ecs_entity_t,
    children: &'static mut [EntityId]
}

#[no_mangle]
pub unsafe fn toxoid_query_entity_list(iter: *mut flecs_core::ecs_iter_t) -> &'static [EntityId] {
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
pub unsafe fn toxoid_entity_get_component(entity: ecs_entity_t, component: ecs_entity_t) -> *mut c_void {
    flecs_core::flecs_entity_get_component(entity, component)
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_cache_insert(
    type_id: core::any::TypeId,
    component_id: ecs_entity_t
) {
    COMPONENT_ID_CACHE.with(|c| {
        let mut cache = c.borrow_mut();
        cache.insert(type_id, component_id);
    });
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_component_cache_get(type_id: core::any::TypeId) -> ecs_entity_t {
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

#[no_mangle]
pub unsafe fn toxoid_progress(delta_time: f32) -> bool {
    flecs_core::flecs_progress(delta_time)
}

#[no_mangle]
pub unsafe fn toxoid_filter_children_init(
    parent: ecs_entity_t
) -> *mut flecs_core::ecs_filter_t {
    flecs_core::flecs_filter_children_init(parent.into())
}

#[no_mangle]
pub unsafe fn toxoid_filter_iter(
    filter: *mut flecs_core::ecs_filter_t
) -> *mut flecs_core::ecs_iter_t {
    flecs_core::flecs_filter_iter(filter)
}

#[no_mangle]
pub unsafe fn toxoid_iter_entities(
    iter: *mut flecs_core::ecs_iter_t
) -> &'static [u64] {
    flecs_core::flecs_iter_entities(iter)
}

#[no_mangle]
pub unsafe fn toxoid_delete_entity(
    entity: ecs_entity_t
) {
    flecs_core::flecs_delete_entity(entity)
}

#[no_mangle]
pub unsafe fn toxoid_entity_remove_component(
    entity: ecs_entity_t,
    component: ecs_entity_t
) {
    flecs_core::flecs_entity_remove_component(entity, component)
}

#[no_mangle]
pub unsafe fn toxoid_filter_next(
    iter: *mut flecs_core::ecs_iter_t
) -> bool {
    flecs_core::flecs_filter_next(iter)
}

#[no_mangle]
pub unsafe fn toxoid_is_valid(
    entity: ecs_entity_t
) -> bool {
    flecs_core::flecs_is_valid(entity)
}

#[no_mangle]
pub unsafe fn toxoid_entity_has_component(
    entity: ecs_entity_t,
    component: ecs_entity_t
) -> bool {
    flecs_core::flecs_entity_has_component(entity, component)
}