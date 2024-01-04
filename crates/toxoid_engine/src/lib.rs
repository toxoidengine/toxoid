pub mod entities;
pub mod systems;
pub mod update;
pub mod utils;

pub use systems::*;
pub use update::*;
pub use utils::*;

pub fn init() {
    // Initialize FLECS ECS.
    toxoid_ffi::flecs_core::init();

    // Initialize default components.
    toxoid_api::components::init();

    // Initialize default entities.
    // TODO: Turn into prefabs
    entities::init();
    // Initialize default engine systems. Such as rendering, input, etc.
    systems::init();

    // Initialize renderer
    // TODO: Renderer backend feature flags
    #[cfg(target_os = "emscripten")]
    toxoid_sokol::init(render_loop);
    #[cfg(not(target_os = "emscripten"))]
    toxoid_sokol::init(render_loop);

    #[cfg(target_os = "emscripten")]
    toxoid_ffi::emscripten::start_loop(gameplay_loop);
    #[cfg(not(target_os = "emscripten"))]
    loop {
        // Update gameplay systems
        gameplay_loop(std::ptr::null_mut());
    }
    
    // Audio test
    // audio::init();
}