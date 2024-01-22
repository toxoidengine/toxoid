
use toxoid_api::*;

pub fn init() {
    // Game Config
    World::add_singleton::<GameConfig>();
    let mut game_config = World::get_singleton::<GameConfig>();
    game_config.set_resolution_width(1280);
    game_config.set_resolution_height(720);

    // Keyboard Input
    World::add_singleton::<KeyboardInput>();

    // Local Player
    World::add_singleton::<NetworkEntity>();
    
    let player_animation_entity = crate::utils::loader::load_animation("assets/player_spine.atlas", "assets/player_spine.json");
    // let mut position = player_animation_entity.get::<Position>();
    player_animation_entity.add::<Local>();
}