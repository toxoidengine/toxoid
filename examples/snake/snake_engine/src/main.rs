extern "C" {
    pub fn app_main();
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_print_i32(v: i32) {
    println!("Printing from Toxoid Engine: {}", v);
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_print_string(v: *const i8) {
    let v = unsafe { std::ffi::CStr::from_ptr(v as *const i8) };
    let v = v.to_string_lossy().into_owned();
    println!("Printing from Toxoid Engine: {}", v);
}

#[no_mangle]
pub unsafe extern "C" fn toxoid_register_tag(name: *const i8, name_len: usize) -> i32 {
    let slice = std::slice::from_raw_parts(name as *mut u8, name_len);
    let string = std::str::from_utf8_unchecked(slice);
    println!("Created tag named: {}", string);
    flecs_core::flecs_tag_create(name) as i32
}



#[no_mangle]
pub unsafe extern "C" fn app_init() {
    app_main();
    toxoid_sdl::create_sdl_loop();
}

fn main() {
    println!("Hello world!");
    flecs_core::init();
    println!("Flecs Initialized!");
    unsafe {
        println!("World: {:?}", flecs_core::WORLD);
    }
}
