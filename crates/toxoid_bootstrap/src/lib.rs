mod renderer;
mod input;
mod systems;
mod entities;
mod prefabs;
#[cfg(not(target_arch = "wasm32"))]
mod watch;

#[no_mangle]
pub extern "C" fn init_bootstrap(user_data: *mut core::ffi::c_void) {
    // Initialize renderer
    toxoid_sokol::sokol_init();
    
    // Enable render systems
    let render_systems = toxoid_api::World::get_singleton::<toxoid_api::RenderSystems>();
    let mut entity = toxoid_api::Entity::from_id(render_systems.get_entity());
    entity.enable();

    // Initialize engine bootstrap entities
    entities::init();

    // Initialize app host after engine is bootstrapped
    // Call user data "init_host" as function
    let init_host = unsafe {
        std::mem::transmute::<*mut core::ffi::c_void, extern "C" fn() -> ()>(user_data)
    };
    init_host();

    // WASM Runtime
    // Watch for file changes for WASM scripts
    #[cfg(not(target_arch = "wasm32"))]
    #[cfg(not(feature = "static-linking"))]
    watch::init();
}

pub fn init(init_host: extern "C" fn()) {
    // Initialize ECS
    toxoid_api::components::init();
    // Game settings
    let mut game_config = toxoid_api::World::get_singleton::<toxoid_api::GameConfig>();
    game_config.set_width(1280);
    game_config.set_height(720);
    // Initialize systems
    systems::init();

    // TODO: Possibly change this when we have Flecs system phases
    // so that we can dynamically enable / disable systems and the ECS
    // so initialization order vs the decoupled renderer is not 
    // nessecarily an issue. (This is already implmented, but should now be implemented in a threaded way with decoupling).
    // Also want to make sure we're running on different loops / threads 
    // for gameplay logic (ECS) vs rendering (Sokol + ECS with renderer phase tagged systems)
    renderer::init(init_host);
}
