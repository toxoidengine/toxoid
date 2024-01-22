pub mod prefabs;
pub mod systems;
pub mod update;
pub mod utils;

pub use systems::*;
pub use update::*;
pub use utils::*;
pub use utils::rand::*;

pub fn init() {
    // Initialize FLECS ECS.
    toxoid_ffi::flecs_core::init();

    // Initialize default components.
    toxoid_api::components::init();

    // Initialize default entities.
    prefabs::init();
    
    // Initialize network functionality.
    utils::network::init();

    // Initialize input functionality.
    utils::input::init();

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
}