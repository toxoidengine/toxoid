use toxoid_api::*;
use crate::prefabs::*;

pub fn init() {
    // Create sprite
    create_sprite("assets/sprite.png");
    // create_sprite("assets/sprite.png");
    // create_sprite("assets/sprite.png");
    // create_sprite("assets/sprite.png");
    // create_sprite("assets/sprite.png");
    // create_sprite("assets/sprite.png");
    // create_sprite("assets/sprite.png");
    // create_sprite("assets/sprite.png");
    let mut entity = Entity::new(None);
    entity.add::<Position>();
    entity.add::<Size>();
    entity.add::<Sprite>();
    entity.add::<Blittable>();
    entity.add::<RenderTarget>();
    
}
