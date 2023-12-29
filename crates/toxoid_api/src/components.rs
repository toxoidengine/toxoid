use toxoid_api_macro::component;
use crate::*;
use crate::IsComponent;
use core::ffi::c_void;

pub enum KeyCode {
    Up = 38,
    Down = 40,
    Left = 37,
    Right = 39
}

component! {
    // Singletons
    GameConfig {
        resolution_width: u32,
        resolution_height: u32,
        scale_factor: f32,
    },
    KeyboardInput {
        up: bool,
        down: bool,
        left: bool,
        right: bool, 
    },

    // Components
    Position {
        x: u32,
        y: u32,
    },
    Velocity {  
        dx: f32,
        dy: f32,
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
    Sprite {
        // filename: *const u8,
        sprite: Pointer
    },

    // Tags
    Renderable {},
    Rect {},
    Loadable {},
}

pub fn init() {    
    // Singletons
    GameConfig::register();
    KeyboardInput::register();
    
    // Components
    Position::register();
    Velocity::register();
    Size::register();
    Color::register();
    Sprite::register();

    // Tags
    Renderable::register();
    Rect::register();
}