use std::ffi::c_char;

extern "C" {
    pub fn app_main();
}

#[no_mangle]
pub unsafe extern "C" fn app_init() {
    app_main();
    // Initialize SDL2
    toxoid_sdl::create_sdl_loop();
}

fn main() {
    flecs_core::init();
    unsafe {
        println!("World: {:?}", flecs_core::WORLD);
    }
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_print_i32(v: i32) {
    println!("Printing from Toxoid Engine: {}", v);
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_print_string(v: *const i8) {
    let v = std::ffi::CStr::from_ptr(v as *const i8);
    let v = v.to_string_lossy().into_owned();
    println!("Printing from Toxoid Engine: {}", v);
}

#[no_mangle]
pub fn toxoid_entity_get_name(id: i32) {
    unsafe {
        let world = *flecs_core::WORLD.as_mut().unwrap_unchecked();
        let tag_name = flecs_core::bindings::ecs_get_name(world, id as u64);

        // Convert to Rust string
        let tag_name = std::ffi::CStr::from_ptr(tag_name as *const i8);
        let tag_name = tag_name.to_string_lossy().into_owned();
        println!("Found tag name: {:?}", tag_name);
    }
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_create_tag(name: *const i8, name_len: usize) -> i32 {
    let slice = std::slice::from_raw_parts(name as *mut u8, name_len);
    let rust_string = std::str::from_utf8_unchecked(slice);
    println!("Created tag named: {}", rust_string);
    let c_string = std::ffi::CString::new(rust_string).expect("Failed to convert to CString");
    flecs_core::flecs_tag_create(c_string.as_ptr()) as i32
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_create_component(
        component_name: *const c_char,
        member_names: *const *const c_char,
        member_names_count: u32,
        // member_types: *const *const u8,
        // member_types_size: u32
) -> i32 {
    use std::ffi::CStr;

    // Convert the component name to a Rust string
    let component_name = CStr::from_ptr(component_name).to_string_lossy().into_owned();
    println!("Component Name: {}", component_name);

    // Iterate over the member names
    for i in 0..member_names_count {
        let member_name_ptr = *member_names.add(i as usize);
        let member_name = CStr::from_ptr(member_name_ptr).to_string_lossy().into_owned();
        println!("Member Name #{}: {}", i, member_name);
    }
    
    // flecs_core::flecs_component_create(
    //     component_name,
    //     member_names,
    //     member_names_count,
    //     member_types,
    //     member_types_size,
    // ) as i32
    0
}