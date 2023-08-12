#![allow(improper_ctypes_definitions)]
mod local_ecs;
mod entities;
mod systems;

extern "C" {
    // Main function of the dynamically linked library / Toxoid App.
    pub fn app_main();
}

fn main() {
    // Initialize Flecs ECS and Toxoid ECS initializers + default components.
    toxoid_ffi::ecs_init();

    // // Initialize all engine default entities. Such as rendering, input, etc.
    entities::init();

    // Initialize default engine systems. Such as rendering, input, etc.
    toxoid_ffi::systems_init();

    // Initialize all engine default engine systems. Such as rendering, input, etc.
    systems::init();
}

#[no_mangle]
pub unsafe extern "C" fn app_init() {
    // Main function of the dynamically linked library / Toxoid App.
    app_main();
    
    // Start update loop / game loop / render loop.
    toxoid_ffi::start();
}
