pub mod allocator;
#[cfg(target_os = "emscripten")]
pub mod emscripten;
pub mod utils;
pub mod ecs;
pub use flecs_core;