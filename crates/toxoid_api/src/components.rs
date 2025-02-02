use crate::*;

component! {
    // -- Singletons --
    KeyboardInput {
        up: bool,
        down: bool,
        left: bool,
        right: bool, 
    },
    RenderSystems {
        entity: u64
    },

    // -- Components --
    // Space
    Position {
        x: u32,
        y: u32
    },
    Size {
        width: u32,
        height: u32
    },
    // Rendering
    Color {
        r: f32,
        g: f32,
        b: f32,
        a: f32
    },
    Sprite {
        sprite: u64
    },
    // Fetch
    FetchRequest {
        path: String,
        data: Vec::<u8>,
        data_type: u8
    },

    // -- Tags --
    // Rendering
    Rect {},
    Renderable {},
    // UI
    Window {},
    Text {},
    Button {},
    // General
    Loading {},
    Loaded {}
}

pub fn init() {
    // Register singletons
    KeyboardInput::register();
    RenderSystems::register();
    // Register components
    Position::register();
    Size::register();
    Color::register();
    Sprite::register();
    FetchRequest::register();
    
    // Register tags
    Rect::register();
    Renderable::register();
    Loading::register();
    Loaded::register();
    
    // Add singletons
    World::add_singleton::<KeyboardInput>();
    World::add_singleton::<RenderSystems>();
}