#![allow(non_camel_case_types)]

use toxoid_ffi_macro::Components;

pub type ecs_id_t = i32;
pub type ecs_entity_t = ecs_id_t;
pub type c_char = i8;
pub const MAX_ELEMENTS: usize = 100;

#[repr(u8)]
pub enum Type {
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
    Array,
    U32Array,
    F32Array,
}

extern "C" {
    pub fn toxoid_print_i32(v: i32);
    pub fn toxoid_print_string(v: *const i8, v_len: usize);
    pub fn toxoid_entity_get_name(id: i32);
    pub fn toxoid_register_tag(name: *const i8, name_len: usize) -> ecs_entity_t;
    pub fn toxoid_register_component(
        component_name: *const c_char,
        component_name_len: u8,
        member_names: *const *const c_char,
        member_names_count: u32,
        member_names_len: *const u8,
        // member_types: *const *const u8,
        // member_types_size: u32
    ) -> ecs_entity_t;
}

#[derive(Components)]
pub struct Position {
    x: u32,
    y: u32,
}

#[derive(Components)]
pub struct Velocity {
    dx: f32,
    dy: f32,
}

#[no_mangle]
pub unsafe extern "C" fn app_main() {
    let tag = register_tag("LocalPlayer");
    toxoid_entity_get_name(tag);

    let mut position = Position { x: 0, y: 0 };
    position.set_x(77);
    position.set_y(99);
    // position.x(10);
    // position.y(12);

    print_string("X:");
    print_i32(position.x as i32);
    print_string("Y:");
    print_i32(position.y as i32);
}

pub fn print_i32(v: i32) {
    unsafe {
        toxoid_print_i32(v);
    }
}

pub fn print_string(v: &str) {
    unsafe {
        toxoid_print_string(v.as_bytes().as_ptr() as *const i8, v.len());
    }
}

pub fn register_tag(name: &str) -> ecs_entity_t {
    unsafe {
        toxoid_register_tag(name.as_bytes().as_ptr() as *const i8, name.len())
    }
}

pub fn register_component(name: &str, member_names: &[&str], member_types: &[u8]) -> ecs_entity_t {
    unsafe {
        let mut c_member_names: [*const c_char; MAX_ELEMENTS] = [core::ptr::null(); MAX_ELEMENTS]; 
        let mut c_member_names_len: [u8; MAX_ELEMENTS] = [0; MAX_ELEMENTS];
        for (i, &s) in member_names.iter().enumerate() {
            c_member_names[i] = s.as_ptr() as *const c_char;
            c_member_names_len[i] = s.len() as u8;
        }
        toxoid_register_component(
            name.as_bytes().as_ptr() as *const c_char,
            name.len() as u8,
            c_member_names.as_ptr(),
            member_names.len() as u32,
            c_member_names_len.as_ptr(),
            // member_types.as_ptr(),
            // member_types.len() as u32
        )
    }
}