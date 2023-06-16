#![allow(non_camel_case_types)]

pub type ecs_id_t = i32;
pub type ecs_entity_t = ecs_id_t;
pub type c_char = i8;
pub const MAX_ELEMENTS: usize = 100;

extern "C" {
    pub fn toxoid_print_i32(v: i32);
    pub fn toxoid_print_string(v: *const i8);
    pub fn toxoid_entity_get_name(id: i32);
    pub fn toxoid_create_tag(name: *const i8, name_len: usize) -> ecs_entity_t;
    pub fn toxoid_create_component(
        component_name: *const c_char,
        member_names: *const *const c_char,
        member_names_count: u32,
        // member_types: *const *const u8,
        // member_types_size: u32
    ) -> ecs_entity_t;
}

#[no_mangle]
pub unsafe extern "C" fn app_main() {
    let tag = create_tag("PositionTag");
    toxoid_entity_get_name(tag);
    // create_component("Position", &["x", "y"], &["i32", "i32"]);
    create_component("Position", &["x", "y"], &[2, 2]);
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

pub fn create_tag(name: &str) -> ecs_entity_t {
    unsafe {
        toxoid_create_tag(name.as_bytes().as_ptr() as *const i8, name.len())
    }
}

pub fn create_component(name: &str, member_names: &[&str], member_types: &[u8]) -> ecs_entity_t {
    unsafe {
        let mut c_member_names: [*const c_char; MAX_ELEMENTS] = [core::ptr::null(); MAX_ELEMENTS]; 
        for (i, &s) in member_names.iter().enumerate() {
            c_member_names[i] = s.as_ptr() as *const c_char;
        }
        toxoid_create_component(
            name.as_bytes().as_ptr() as *const c_char,
            c_member_names.as_ptr(),
            member_names.len() as u32,
            // member_types.as_ptr(),
            // member_types.len() as u32
        )
    }
}

// pub fn convert_str_slice(input: &[&str]) -> *const *const c_char {
//     let c_strings: Vec<*const c_char> = input.iter().map(|&s| s.as_ptr() as *const c_char).collect();
//     let c_array = c_strings.as_ptr();
//     core::mem::forget(c_strings);
//     c_array
// }

// pub const MAX_ELEMENTS: usize = 100;
// pub fn convert_str_slice(input: &[&str; MAX_ELEMENTS]) -> *const *const c_char {
//     let mut c_strings: [*const c_char; MAX_ELEMENTS] = [core::ptr::null(); MAX_ELEMENTS];    
//     for (i, &s) in input.iter().enumerate() {
//         c_strings[i] = s.as_ptr() as *const c_char;
//     }
//     c_strings.as_ptr()
// }
