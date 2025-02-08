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
    Image {
        info: u64
    },
    // Fetch
    FetchRequest {
        data_type: u8,
        path: String,
        data: Vec::<u8>,
        user_data: u64
    },
    // Animation
    Atlas {
        atlas: u64,
        filename: String,
        data: Vec::<u8>,
        loaded: bool,
    },
    Skeleton {
        skeleton: u64,
        filename: String,
        data: Vec::<u8>,
        loaded: bool,
    },
    Images {
        images: Vec::<u64>,
        loaded: bool,
    },
    BoneAnimation {
        animation_state: String,
        animation: String
    },
    SpineInstance {
        instance: u64,
        ctx: u64,
        instantiated: bool
    },

    // -- Tags --
    // Rendering
    Rect {},
    Blittable {},
    Renderable {},
    // UI
    Window {},
    Text {},
    Button {},
    // General
    Loading {},
    Loaded {},
    Connected {},
    Disconnected {},

    TestComponent {
        test_4: String,
        test_3: Vec::<u8>,
        test_2: u8,
        test: u64
    }
}

pub fn init() {
    // Register singletons
    KeyboardInput::register();
    RenderSystems::register();
    // Register components
    // Space
    Position::register();
    Size::register();
    // Rendering
    Color::register();
    Sprite::register();
    Image::register();
    // Fetch
    FetchRequest::register();
    // Animation
    Atlas::register();
    Skeleton::register();
    Images::register();
    BoneAnimation::register();
    SpineInstance::register();

    // Register tags
    // Rendering
    Rect::register();
    Blittable::register();
    Renderable::register();
    // UI
    Window::register();
    Text::register();
    Button::register();
    // General
    Loading::register();
    Loaded::register();
    Connected::register();
    Disconnected::register();

    // Add singletons
    World::add_singleton::<KeyboardInput>();
    World::add_singleton::<RenderSystems>();

    // Add test component
    TestComponent::register();
}