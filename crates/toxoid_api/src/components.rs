use crate::*;

component! {
    // -- Singletons --
    KeyboardInput {
        up: bool,
        down: bool,
        left: bool,
        right: bool, 
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
    // Fetch
    FetchRequest {
        path: String,
        // TODO: Make this a Vec<u8> after we implement that in toxoid_api_macro
        data: Vec::<u64>
    },

    // -- Tags --
    // Rendering
    Rect {},
    Renderable {},
    // General
    Loading {},
    Loaded {}
}

pub fn init() {
    // Register singletons
    KeyboardInput::register();

    // Register components
    Position::register();
    Size::register();
    Color::register();
    FetchRequest::register();
    
    // Register tags
    Rect::register();
    Renderable::register();
    Loading::register();
    Loaded::register();
    
    // Add singletons
    World::add_singleton::<KeyboardInput>();
}