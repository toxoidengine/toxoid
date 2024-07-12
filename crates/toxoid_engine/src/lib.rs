pub mod prefabs;
pub mod systems;
pub mod update;
pub mod utils;

pub use systems::*;
pub use update::*;
pub use utils::*;
pub use utils::rand::*;

#[no_mangle]
pub extern "C" fn toxoid_engine_init() {
    // Initialize sokol
    #[cfg(feature = "render")]
    toxoid_sokol::sokol_init();

    // Initialize default entities.
    prefabs::init();

    // Initialize default engine systems. Such as rendering, input, etc.
    systems::init();

    // Test WASM runtime
    // toxoid_wasm::wasm_init();

    #[cfg(target_os = "emscripten")]
    toxoid_ffi::emscripten::start_loop(game_loop);

    // #[cfg(not(target_os = "emscripten"))]
    // std::thread::spawn(move || {
    //     // 16 ms time step
    //     let mut last_time = std::time::Instant::now();
    //     let mut _delta_time = 0.0;
    //     loop {
    //         let current_time = std::time::Instant::now();
    //         _delta_time = current_time.duration_since(last_time).as_secs_f32();
    //         last_time = current_time;
    //         game_loop(std::ptr::null_mut());
    //         std::thread::sleep(std::time::Duration::from_millis(16));
    //     }
    // });
}

pub fn init() {
    // Set up ECS (Currently just threads configuration)
    toxoid_ffi::flecs_core::init();

    // Initialize default components.
    toxoid_api::components::init();

    // Bootstrap singletons
    toxoid_api::bootstrap::default();
    
    // Initialize network functionality.
    utils::network::init();

    // Initialize sokol
    // Add toxoid engine init to SOKOL init callback because
    // we need to initialize sokol on the main thread.
    // Many graphics APIS require that the window and rendering context be created and managed on the main thread. This is a common restriction due to how these APIs interact with the operating system's windowing system. Same with the browser unless offscreen canvas is properly supported in Emscripten.
    #[cfg(feature = "render")]
    #[cfg(target_os = "emscripten")]
    // Null mut as function pointer
    toxoid_sokol::init(toxoid_engine_init, game_loop, utils::sokol_event::sokol_event);

    // If not emscripten, just run the game loop for now.
    // TODO: Use Flecs staging and pipelines to run the game loop on a separate thread.
    #[cfg(feature = "render")]
    #[cfg(not(target_os = "emscripten"))]
    toxoid_sokol::init(toxoid_engine_init, game_loop, utils::sokol_event::sokol_event);

    // If serverside, we will not init using sokol
    #[cfg(not(feature = "render"))]
    toxoid_engine_init();
}