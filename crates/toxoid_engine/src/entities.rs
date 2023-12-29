use toxoid_api::*;
use crate::components::*;

pub fn init() {
    World::add_singleton::<GameConfig>();
    let mut game_config = World::get_singleton::<GameConfig>();
    game_config.set_resolution_width(1280);
    game_config.set_resolution_height(720);

    World::add_singleton::<KeyboardInput>();

    let mut entity = Entity::new();
    entity.add::<Sprite>();
    let sprite = entity.get::<Sprite>();

    sprite.set_sprite()
    

}