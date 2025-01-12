#[cfg(target_arch = "wasm32")]
mod wasm;
mod components;
mod entities;
mod systems;

pub fn init() {
    components::init();
    entities::init();
    systems::init();
}