use toxoid_api::*;
use crate::prefabs::*;

pub fn init() {
    // Create sprite
    create_sprite("assets/character.png");
    toxoid_api::load_animation("assets/animations/player.atlas", "assets/animations/player.json");
}
