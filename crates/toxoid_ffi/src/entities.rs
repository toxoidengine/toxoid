use crate::DirectionEnum;
use crate::toxoid_api::Entity;
use crate::components::{KeyboardInput, Position, Rect, Renderable, Direction};

pub fn init() {
    // Keyboard Input
    let mut keyboard_entity = Entity::new();
    keyboard_entity.add::<KeyboardInput>();

    let mut player_entity = Entity::new();
    player_entity.add::<Position>();
    player_entity.add::<Direction>();

    let mut pos = player_entity.get::<Position>();
    pos.set_x(350);
    pos.set_y(50);

    let mut dir = player_entity.get::<Direction>();
    dir.set_direction(DirectionEnum::Down as u8);
    
    let mut render_target = Entity::new();
    render_target.add::<Rect>();
    render_target.add::<Renderable>();
    render_target.child_of(player_entity);

    let mut rect = render_target.get::<Rect>();
    rect.set_width(25);
    rect.set_height(25);
}