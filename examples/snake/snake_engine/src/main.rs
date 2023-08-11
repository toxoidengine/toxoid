#![allow(improper_ctypes_definitions)]
mod local_ecs;
use toxoid_ffi::*;

extern "C" {
    // Main function of the dynamically linked library / Toxoid App.
    pub fn app_main();
}

#[no_mangle]
pub unsafe extern "C" fn app_init() {
    app_main();
    // Initialize SDL2
    // toxoid_sdl::create_sdl_loop();
}

fn main() {
    init();
}
