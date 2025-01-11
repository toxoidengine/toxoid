mod renderer;
mod input;
mod systems;
mod entities;
#[cfg(not(target_arch = "wasm32"))]
mod watch;

pub fn init() {
    toxoid_api::components::init();
    entities::init();
    systems::init();
    #[cfg(not(target_arch = "wasm32"))]
    watch::init();
    renderer::init();
}
