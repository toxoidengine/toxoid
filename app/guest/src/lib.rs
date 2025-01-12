mod components;
mod entities;
mod systems;
#[cfg(target_arch = "wasm32")]
mod wasm;

pub fn init() {
    components::init();
    entities::init();
    systems::init();
}