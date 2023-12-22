use toxoid_api::components::*;
use toxoid_api::Entity;
use crate::components::*;

pub fn create_player_block(x: u32, y: u32, direction: u8, child: u64) {
    let mut player_entity = toxoid_api::Entity::new();
    player_entity.add::<Player>();
    player_entity.add::<Position>();
    player_entity.add::<Direction>();

    let mut pos = player_entity.get::<Position>();
    pos.set_x(x);
    pos.set_y(y);

    let mut dir = player_entity.get::<Direction>();
    dir.set_direction(direction);
    
    player_entity.add::<Head>();
    if child != 0 {
        player_entity.parent_of(Entity { id: child, children: &mut [] });
    }

    let mut renderable_entity = toxoid_api::Entity::new();
    renderable_entity.add::<Renderable>();
    renderable_entity.add::<Size>();
    renderable_entity.add::<Color>();
    renderable_entity.add::<Position>();
    
    let mut rect = renderable_entity.get::<Size>();
    rect.set_width(50);
    rect.set_height(50);

    let mut pos = renderable_entity.get::<Position>();
    pos.set_x(x);
    pos.set_y(y);

    let mut color = renderable_entity.get::<Color>();
    color.set_r(255);
    color.set_g(0);
    color.set_b(0);
    color.set_a(255);
}

pub fn init() {
    create_player_block(0, 0, DirectionEnum::Down as u8, 0);
}