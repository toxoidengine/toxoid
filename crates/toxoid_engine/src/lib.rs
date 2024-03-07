#![feature(thread_id_value)]
pub mod prefabs;
pub mod systems;
pub mod update;
pub mod utils;
pub mod events;

pub use systems::*;
pub use update::*;
pub use utils::*;
pub use utils::rand::*;
pub use toxoid_ffi;

pub fn init() {
    println!("Hello world!");
    
    // Set ECS threads
    unsafe { toxoid_ffi::flecs_core::ecs_set_threads(*toxoid_ffi::flecs_core::WORLD, 12) };

    // Initialize default components.
    toxoid_api::components::init();

    // Initialize sokol
    // TODO: Renderer backend feature flags
    // Check if emscripten but also check if renderer feature is enabled
    #[cfg(feature = "render")]
    toxoid_sokol::init(render_loop);

    // Initialize default entities.
    prefabs::init();

    // Initialize default events.
    events::init();
    
    // Initialize network functionality.
    // utils::network::init();

    // Initialize input functionality.
    utils::input::init();

    // Initialize default engine systems. Such as rendering, input, etc.
    systems::init();
    
    #[cfg(target_os = "emscripten")]
    toxoid_ffi::emscripten::start_loop(game_loop);
}