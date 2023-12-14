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

    // Create a new entity.
    let player = Entity::new();

    unsafe { 
        // Add the component to the entity.
        (*player).add::<Position>();

        // Get the component from the entity.
        let mut pos_component = (*player).get::<Position>();
        pos_component.set_x(420);
        pos_component.set_y(421);

        // Get component data.
        let x = pos_component.get_x();
        let y = pos_component.get_y();
        let query = Query::new::<(Position,)>();
        let query = (*query).iter();
        while query.next() {
            let pos = query.field::<Position>();
            pos.iter().for_each(|pos| {
                print_string("Position X Value:");
                print_i32(pos.get_x() as i32);
                print_string("Position Y Value:");
                print_i32(pos.get_y() as i32);
            });
        //     ALLOCATOR.dealloc(pos.as_ptr() as *mut u8, core::alloc::Layout::new::<Position>());
    }
        
        // Print the component data.
        print_i32(x as i32);
        print_i32(y as i32);

        print_string("HELLO WORLD 123!");
    };
}