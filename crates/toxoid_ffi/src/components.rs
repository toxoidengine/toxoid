use toxoid_api_macro::component;
use toxoid_api::IsComponent;
use crate::ecs::*;

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
    //Generic Components
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
}