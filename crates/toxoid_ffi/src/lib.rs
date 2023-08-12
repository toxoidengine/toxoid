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

pub fn init() {
    // Initialize FLECS ECS.
    flecs_core::init();
    // Initialize default components.
    components::init();
    // Initialize default engine systems. Such as rendering, input, etc.
    systems::init();
}

// Start update loop / game loop / render loop.
pub fn start() {
    #[cfg(target_os = "emscripten")]
    emscripten::start_loop();
    #[cfg(not(target_os = "emscripten"))]
    loop {}
}