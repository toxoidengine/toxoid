use toxoid_api::*;
use crate::components::*;

pub fn create_player_block(x: u32, y: u32, child: u64) {
    let mut player_entity = toxoid_api::Entity::new();
    player_entity.add::<Player>();
    player_entity.add::<Position>();
    player_entity.add::<Direction>();

    let mut pos = player_entity.get::<Position>();
    pos.set_x(x);
    pos.set_y(y);
    
    player_entity.add::<Head>();
    if child != 0 {
        player_entity.parent_of(Entity { id: child, children: &mut [] });
    }

    let mut renderable_entity = toxoid_api::Entity::new();
    renderable_entity.add::<Rect>();
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
    color.set_r(0);
    color.set_g(255);
    color.set_b(0);
    color.set_a(255);

    renderable_entity.child_of(player_entity);
}

pub fn create_food_block() -> u64 {
    let (random_x, random_y) =  unsafe { gen_rng_grid_pos() };

    let mut food_entity = Entity::new();
    let id = food_entity.get_id();
    food_entity.add::<Position>();
    food_entity.add::<Food>();

    let mut pos = food_entity.get::<Position>();
    pos.set_x(random_x as u32);
    pos.set_y(random_y as u32);

    let mut renderable_entity = Entity::new();
    renderable_entity.add::<Rect>();
    renderable_entity.add::<Renderable>();
    renderable_entity.add::<Size>();
    renderable_entity.add::<Color>();
    renderable_entity.add::<Position>();
    renderable_entity.child_of(food_entity);

    let mut rect = renderable_entity.get::<Size>();
    rect.set_width(50);
    rect.set_height(50);

    let mut render_pos = renderable_entity.get::<Position>();
    render_pos.set_x(random_x as u32);
    render_pos.set_y(random_y as u32);

    let mut color = renderable_entity.get::<Color>();
    color.set_r(255);
    color.set_g(0);
    color.set_b(0);

    id
}

pub fn init() {
    create_player_block(0, 0, 0);
    create_food_block();
    
    World::add_singleton::<Direction>();
    let mut direction = World::get_singleton::<Direction>();
    direction.set_direction(DirectionEnum::Down as u8);
}

