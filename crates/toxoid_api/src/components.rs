use toxoid_api_macro::component;
use crate::*;
use crate::IsComponent;

pub enum KeyCode {
    Up = 38,
    Down = 40,
    Left = 37,
    Right = 39
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
    Size {
        width: u32,
        height: u32,
    },
    Color {
        r: u8,
        g: u8,
        b: u8,
        a: u8
    },
    Renderable {
        x: u32,
        y: u32,
    },
    GameConfig {
        resolution_width: u32,
        resolution_height: u32,
        scale_factor: f32,
    },
}

pub fn init() {    
    // Generic Components
    Position::register();
    Velocity::register();
    KeyboardInput::register();
    Size::register();
    Color::register();
    Renderable::register();
    GameConfig::register();
}