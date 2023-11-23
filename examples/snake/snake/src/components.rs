use toxoid_api_macro::component;
use toxoid_api::IsComponent;
use toxoid_api::bindings::*;
use toxoid_api::*;

pub enum DirectionEnum {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

component! {
    Position {
        x: u32,
        y: u32,
    },
    Velocity {
        dx: f32,
        dy: f32,
    },
    KeyboardInput {
        up: bool,
        down: bool,
        left: bool,
        right: bool, 
    },
    Rect {
        width: u32,
        height: u32,
    },
    Color {
          r: u8,
        g: u8,
        b: u8,
    },
    Renderable {
        x: u32,
        y: u32,
    },
    Direction {
        direction: u8
    },
    Player {
        player: bool
    },
    Food {
        food: bool
    },
    Despawn {
        despawn: bool
    },
    Spawn {
        spawn: bool
    },
    TailLength {
        length: u32
    },
    Head {
        previous_head: u64
    },
}

pub fn init() {    
    // Generic Components
    Position::register();
    Velocity::register();
    KeyboardInput::register();
    Rect::register();
    Color::register();
    Renderable::register();
    // Snake Specific
    Direction::register();
    Player::register();
    Food::register();
    Despawn::register();
    Spawn::register();
    TailLength::register();
    Head::register();

    print_string("Hello bruh!");
    // Create a new entity.
    let mut player = Entity::new();
    print_string("Hello bruh 2!");
    // Add the component to the entity.
    unsafe { 
        (*player).add::<Position>() 
    };
    print_string("Hello bruh 3!");

    // let mut pos_component = player.get::<Position>();
    // pos_component.set_x(420);
    // pos_component.set_y(421);
}