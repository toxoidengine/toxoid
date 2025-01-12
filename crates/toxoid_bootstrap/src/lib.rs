mod renderer;
mod input;
mod systems;
mod entities;
#[cfg(not(target_arch = "wasm32"))]
mod watch;

pub fn init_ecs() {
    toxoid_api::components::init();
    entities::init();
    systems::init();
    // #[cfg(not(target_arch = "wasm32"))]
    // watch::init();
}

pub fn init_renderer() {
    renderer::init();
}
