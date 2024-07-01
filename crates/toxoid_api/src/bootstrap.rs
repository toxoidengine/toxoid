use crate::*;

pub fn bootstrap_game_config() {
    World::add_singleton::<GameConfig>();
    let mut game_config = World::get_singleton::<GameConfig>();
    game_config.set_resolution_width(1280);
    game_config.set_resolution_height(720);

    print_string("Bootstrapped!")
}

pub fn bootstrap_input() {
    World::add_singleton::<KeyboardInput>();
}

#[cfg(feature = "net")]
pub fn bootstrap_network() {
    World::add_singleton::<Networked>();
}

pub fn bootstrap_player() {
    // #[cfg(feature = "net")]
    // let mut networked = World::get_singleton::<Networked>();

    // // Player Entity
    // let mut player_entity = Entity::new();
    // player_entity.add::<Player>();
    // player_entity.add::<Local>();
    // #[cfg(feature = "net")]
    // player_entity.add::<Networked>();

    // #[cfg(feature = "net")]
    // networked.set_entity_id(player_entity.get_id());
}

pub fn default() {
    // Game Config
    bootstrap_game_config();

    // Keyboard Input
    bootstrap_input();

    // Bootstrap network
    #[cfg(feature = "net")]
    bootstrap_network();

    // Bootstrap player
    bootstrap_player();
}

