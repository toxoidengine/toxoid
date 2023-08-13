use crate::toxoid_api::Entity;
use crate::components::{KeyboardInput, Position, Rect, Renderable};

pub fn init() {
    // Keyboard Input
    let mut keyboard_entity = Entity::new();
    keyboard_entity.add::<KeyboardInput>();

    let mut player_entity = Entity::new();
    player_entity.add::<Position>();

    let mut pos = player_entity.get::<Position>();
    pos.set_x(350);
    pos.set_y(50);
    
    let mut render_target = Entity::new();
    render_target.add::<Rect>();
    render_target.add::<Renderable>();
    render_target.child_of(player_entity);

    let mut rect = render_target.get::<Rect>();
    rect.set_width(25);
    rect.set_height(25);
    
    player_entity
        .children()
        .iter()
        .for_each(|child| {
            println!("Children: {}", child.get_id());
        });
}