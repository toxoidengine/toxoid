use toxoid_api::*;

pub fn init() {
    // Game settings
    let game_config = World::get_singleton::<GameConfig>();
    
    // Set minimum window dimensions (720p)
    game_config.set_min_window_width(1280);
    game_config.set_min_window_height(720);
    
    // Initialize with minimum dimensions
    game_config.set_window_width(1280);
    game_config.set_window_height(720);
    game_config.set_game_width(1280);
    game_config.set_game_height(720);
}