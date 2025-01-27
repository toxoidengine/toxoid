mod renderer;
mod input;
mod systems;
mod entities;
#[cfg(not(target_arch = "wasm32"))]
mod watch;

#[no_mangle]
pub extern "C" fn init_bootstrap(user_data: *mut core::ffi::c_void) {
    // Initialize ECS
    toxoid_api::components::init();
    entities::init();
    systems::init();

    // Call user data "init_host" as function
    let init_host = unsafe {
        std::mem::transmute::<*mut core::ffi::c_void, extern "C" fn() -> ()>(user_data)
    };
    init_host();

    // Initialize renderer
    toxoid_sokol::sokol_init();

    // WASM Runtime
    // Watch for file changes for WASM scripts
    #[cfg(not(target_arch = "wasm32"))]
    watch::init();
}

pub fn init(init_host: extern "C" fn()) {
    // TODO: Possibly change this when we have Flecs system phases
    // so that we can dynamically enable / disable systems and the ECS
    // so initialization order vs the decoupled renderer is not 
    // nessecarily an issue.
    // Also want to make sure we're running on different loops / threads 
    // for gameplay logic (ECS) vs rendering (Sokol + ECS with renderer phase tagged systems)
    renderer::init(init_host);
}
