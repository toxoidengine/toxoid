pub mod allocator;
pub mod ecs;
pub mod components;
pub mod systems;
pub mod emscripten;
pub mod update_loop;

pub use allocator::*;
pub use ecs::*;
pub use emscripten::*;
pub use update_loop::*;
pub use components::*;
pub use toxoid_api;
pub use flecs_core;

// Initialize Flecs ECS and Toxoid ECS initializers.
pub fn ecs_init() {
    flecs_core::init();
    components::init();
}

pub fn systems_init() {
    systems::init();
}

// Start update loop / game loop / render loop.
pub fn start() {
    #[cfg(target_os = "emscripten")]
    emscripten::start_loop();
    #[cfg(not(target_os = "emscripten"))]
    loop {}
}