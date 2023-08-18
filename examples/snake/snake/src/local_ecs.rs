use core::any::TypeId;
use core::alloc::Layout;
use core::alloc::GlobalAlloc;
use toxoid_api::*;

// Have to define at top level as a workaround to maintain context in toxoid_api_macro
pub fn register_component_ecs(
    name: &str,
    member_names: &[&str],
    member_types: &[u8],
) -> ecs_entity_t {
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

        toxoid_register_component(
            name.as_bytes().as_ptr() as *const c_char,
            name.len() as u8,
            member_names_ptr as *const *const c_char,
            member_names.len() as u32,
            member_names_len_ptr,
            member_types_ptr,
            member_types.len() as u32,
        )
    }
}

pub fn cache_component_ecs(type_id: TypeId, component_id: ecs_entity_t) {
    unsafe {
        toxoid_component_cache_insert(type_id, component_id);
    }
}
