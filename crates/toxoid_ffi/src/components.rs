use toxoid_api_macro::component;
use toxoid_api::{IsComponent};
use crate::ecs::*;

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
        x: u32,
        y: u32,
        w: u32,
        h: u32,
    },
    Color {
        r: u8,
        g: u8,
        b: u8,
    },
    Renderable {
        x: u32,
        y: u32,
    }
}

pub fn init() {
    Position::register();
    Velocity::register();
    KeyboardInput::register();
    Rect::register();
    Color::register();
    Renderable::register();
}