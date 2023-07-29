use toxoid_ffi::*;
use core::any::TypeId;

// Have to define at top level as a workaround to maintain context in toxoid_ffi_macro
pub fn register_component_ecs(name: &str, member_names: &[&str], member_types: &[u8]) -> ecs_entity_t {
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
            c_member_types.len() as u32
        )
    }
}

pub fn cache_component_ecs(type_id: TypeId, component_id: i32) {
    unsafe {
        toxoid_component_cache_insert(type_id, component_id);
    }
}