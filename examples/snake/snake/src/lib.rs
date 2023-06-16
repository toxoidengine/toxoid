#![allow(non_camel_case_types)]
pub type ecs_id_t = i32;
pub type ecs_entity_t = ecs_id_t;
pub type c_char = u8;

extern "C" {
    pub fn toxoid_print_i32(v: i32);
    pub fn toxoid_print_string(v: *const i8);
    pub fn toxoid_register_tag(name: *const i8, name_len: usize) -> ecs_entity_t;
    pub fn toxoid_entity_get_name(id: i32);
}

#[no_mangle]
pub unsafe extern "C" fn app_main() {
    let tag = register_tag("PositionTag");
    let tag_2 = register_tag("PositionTagTwo");
    toxoid_entity_get_name(tag);
    toxoid_entity_get_name(tag_2);
}

pub fn print_i32(v: i32) {
    unsafe {
        toxoid_print_i32(v);
    }
}

pub fn print_string(v: &str) {
    unsafe {
        toxoid_print_string(v.as_bytes().as_ptr() as *const i8);
    }
}

pub fn register_component() {}

pub fn register_tag(name: &str) -> ecs_entity_t {
    unsafe {
        toxoid_register_tag(name.as_bytes().as_ptr() as *const i8, name.len())
    }
}
