use toxoid_api::ecs_entity_t;

use crate::{DirectionEnum, utils};
use crate::toxoid_api::Entity;
use crate::components::{KeyboardInput, Position, Rect, Renderable, Direction, Color, Player, Food, Head};

pub fn create_player_block(x: u32, y: u32, head: bool, parent_entity: ecs_entity_t) -> ecs_entity_t {
    let mut player_entity = Entity::new();
    player_entity.add::<Position>();
    player_entity.add::<Direction>();
    player_entity.add::<Player>();
    let mut pos = player_entity.get::<Position>();
    pos.set_x(x);
    pos.set_y(y);
    let mut dir = player_entity.get::<Direction>();
    dir.set_direction(DirectionEnum::Down as u8);
    if head {
        player_entity.add::<Head>();
    } else {
        player_entity.child_of(Entity { id: parent_entity, children: &mut [] });
    }
    let id = player_entity.get_id();

    // Child Entity
    let mut render_target = Entity::new();
    render_target.add::<Rect>();
    render_target.add::<Renderable>();
    render_target.add::<Color>();
    render_target.add::<Position>();
    render_target.child_of(player_entity);
    let mut rect = render_target.get::<Rect>();
    rect.set_width(50);
    rect.set_height(50);
    let mut color = render_target.get::<Color>();
    color.set_r(0);
    color.set_g(200);
    color.set_b(0);
    let mut render_pos = render_target.get::<Position>();
    render_pos.set_x(pos.get_x());
    render_pos.set_y(pos.get_y());

    id
}

pub fn init() {
    // Keyboard Input Entity
    // TODO: Make this Singleton
    {
        let mut keyboard_entity = Entity::new();
        keyboard_entity.add::<KeyboardInput>();
    }
    
    // Create 3 default snake blocks, on head, 2 body / tail
    let parent_id = create_player_block(350, 150, true, 0);
    let parent_id = create_player_block(350, 100, false, parent_id);
    create_player_block(350, 50, false, parent_id);

    {
        let (random_x, random_y) = utils::gen_rng_grid_pos();

        let mut food_entity = Entity::new();
        food_entity.add::<Position>();
        food_entity.add::<Food>();

        let mut pos = food_entity.get::<Position>();
        pos.set_x(random_x as u32);
        pos.set_y(random_y as u32);

        let mut render_target = Entity::new();
        render_target.add::<Rect>();
        render_target.add::<Renderable>();
        render_target.add::<Color>();
        render_target.add::<Position>();
        render_target.child_of(food_entity);
        let mut rect = render_target.get::<Rect>();
        rect.set_width(50);
        rect.set_height(50);
        let mut color = render_target.get::<Color>();
        color.set_r(255);
        color.set_g(0);
        color.set_b(0);
        let mut render_pos = render_target.get::<Position>();
        render_pos.set_x(random_x as u32);
        render_pos.set_y(random_y as u32);
    }
}