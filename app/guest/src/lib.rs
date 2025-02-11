mod components;
mod entities;
mod systems;
// target arch wasm32 but not emscripten target os
#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
mod wasm;

pub fn init() {
    // components::init();
    // entities::init();
    // systems::init();

    // Create sprite
    toxoid_api::load_sprite("assets/character.png");
    toxoid_api::load_animation("assets/animations/player.atlas", "assets/animations/player.json");
    toxoid_api::load_worldmap("assets/world_0.world");
}