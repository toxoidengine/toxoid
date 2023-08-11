pub mod allocator;
pub mod ecs;
pub mod emscripten;

pub use allocator::*;
pub use ecs::*;
pub use emscripten::*;

pub fn init() {
    flecs_core::init();
    ecs::init();
}