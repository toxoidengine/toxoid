use crate::DirectionEnum;
use crate::toxoid_api::Entity;
use crate::components::{KeyboardInput, Position, Rect, Renderable, Direction, Color, Player, Food};

pub fn create_player_block(x: u32, y: u32, head: bool) {
    let mut player_entity = Entity::new();
    player_entity.add::<Position>();
    player_entity.add::<Direction>();
    player_entity.add::<Player>();
    let mut pos = player_entity.get::<Position>();
    pos.set_x(x);
    pos.set_y(y);
    let mut dir = player_entity.get::<Direction>();
    dir.set_direction(DirectionEnum::Down as u8);
    let mut player = player_entity.get::<Player>();
    player.set_head(head);

    
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
}

pub fn init() {
    // Keyboard Input Entity
    // TODO: Make this Singleton
    {
        let mut keyboard_entity = Entity::new();
        keyboard_entity.add::<KeyboardInput>();
    }
    
    // Player Entity
    // TODO: Move this out of lib
    {
        // Parent entity
        
    }

    create_player_block(350, 150, true);
    create_player_block(350, 100, false);
    create_player_block(350, 50, false);

    // Food Entity
    // TODO: Move this out of lib
    {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let random_x = rng.gen_range(0..16) * 50; 
        let random_y = rng.gen_range(0..12) * 50; 

        let mut food_entity = Entity::new();
        food_entity.add::<Position>();
        food_entity.add::<Food>();

        let mut pos = food_entity.get::<Position>();
        pos.set_x(random_x);
        pos.set_y(random_y);

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
        render_pos.set_x(random_x);
        render_pos.set_y(random_y);
    }
}