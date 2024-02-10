
use toxoid_api::*;
use toxoid_sokol::SokolRenderer2D;

pub fn init() {
    // Game Config
    World::add_singleton::<GameConfig>();
    let mut game_config = World::get_singleton::<GameConfig>();
    game_config.set_resolution_width(1280);
    game_config.set_resolution_height(720);

    // Keyboard Input
    World::add_singleton::<KeyboardInput>();

    // Local Player
    World::add_singleton::<Networked>();
    
    // Parent entity
    let mut player_entity = Entity::new();
    player_entity.add::<Local>();
    player_entity.add::<Player>();
    player_entity.add::<Networked>();
    let mut local_player = World::get_singleton::<Networked>();
    local_player.set_entity_id(player_entity.get_id());

    // TODO: Make this a child entity later
    
    #[cfg(target_os = "emscripten")]
    // let player_animation_entity = crate::utils::load::load_animation("assets/player_spine.atlas", "assets/player_spine.json");
    // let mut position = player_animation_entity.get::<Position>();
    #[cfg(target_os = "emscripten")]
    // player_animation_entity.add::<Local>();


    crate::utils::load::load_image("assets/character.png");
}