use toxoid_api::*;

pub fn init() {
    World::add_singleton::<GameConfig>();
    let mut game_config = World::get_singleton::<GameConfig>();
    game_config.set_resolution_width(1280);
    game_config.set_resolution_height(720);

    World::add_singleton::<KeyboardInput>();

    crate::utils::load_image("assets/character.png");
}