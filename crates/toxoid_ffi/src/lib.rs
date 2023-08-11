pub mod allocator;
pub mod ecs;
pub mod emscripten;

pub use allocator::*;
pub use ecs::*;
pub use emscripten::*;

// Initialize Flecs ECS and Toxoid ECS initializers.
pub fn init() {
    flecs_core::init();
    ecs::init();
}

// Start update loop / game loop / render loop.
pub fn start() {
    #[cfg(target_os = "emscripten")]
    emscripten::start_loop();
    #[cfg(not(target_os = "emscripten"))]
    loop {}
}