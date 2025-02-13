use toxoid_api::*;

pub fn init() {
    // Game settings
    let game_config = World::get_singleton::<GameConfig>();
    game_config.set_window_width(1280);
    game_config.set_window_height(720);
    game_config.set_game_width(1280);
    game_config.set_game_height(720);
}