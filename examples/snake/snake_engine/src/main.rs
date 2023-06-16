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
pub unsafe extern "C" fn toxoid_register_tag(name: *const i8, name_len: usize) -> i32 {
    let c_slice = std::slice::from_raw_parts(name as *mut u8, name_len);
    let c_string = std::str::from_utf8_unchecked(c_slice);
    println!("Created tag named: {}", c_string);
    flecs_core::flecs_tag_create(c_string.as_bytes().as_ptr() as *const i8) as i32
}

#[no_mangle]
pub fn toxoid_entity_get_name(id: i32) {
    unsafe {
        let world = *flecs_core::WORLD.as_mut().unwrap_unchecked();
        let tag_name = flecs_core::bindings::ecs_get_name(world, id as u64);
        // let v = std::ffi::CStr::from_ptr(tag_name as *const i8);
        // let v = v.to_string_lossy().into_owned();
        let c_slice = std::slice::from_raw_parts(tag_name as *mut u8, 10);
        let c_string = std::str::from_utf8_unchecked(c_slice);
        println!("Found tag name: {:?}", c_string);
    }
}