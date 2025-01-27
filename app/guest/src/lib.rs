mod components;
mod entities;
mod systems;
// target arch wasm32 but not emscripten target os
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
mod wasm;

pub fn init() {
    components::init();
    entities::init();
    systems::init();
    toxoid_api::fetch("sprite.png")
}