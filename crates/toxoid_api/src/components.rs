use toxoid_api_macro::component;
use crate::*;
use crate::IsComponent;

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
    }
}

pub fn init() {    
    // Generic Components
    Position::register();
    Velocity::register();
    KeyboardInput::register();
    Rect::register();
    Color::register();
    Renderable::register();
    GameConfig::register();
}