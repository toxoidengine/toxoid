#![allow(improper_ctypes_definitions)]
mod local_ecs;

extern "C" {
    // Main function of the dynamically linked library / Toxoid App.
    pub fn app_main();
}

fn main() {
    // Initialize Flecs ECS and Toxoid ECS initializers.
    toxoid_ffi::init();
}

#[no_mangle]
pub unsafe extern "C" fn app_init() {
    // Main function of the dynamically linked library / Toxoid App.
    app_main();
    
    // Start update loop / game loop / render loop.
    toxoid_ffi::start();
}
