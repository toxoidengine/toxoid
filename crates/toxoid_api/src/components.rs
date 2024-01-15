use serde::{Deserialize, Serialize};
use toxoid_api_macro::component;
use crate::*;

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
        filename: StringPtr,
        sprite: Pointer
    },
    NetworkEntity {
        id: u64,
        entity_id: u64,
    },
    WebSocket {
        socket: Pointer
    },
    Atlas {
        atlas_filename: StringPtr,
        atlas: Pointer,
        data_size: u64,
        loaded: bool,
    },
    Skeleton {
        skeleton_filename: StringPtr,
        skeleton: Pointer,
        data_size: u64,
        loaded: bool,
    },
    Images {
        images: Pointer,
        loaded: bool,
    },
    BoneAnimation {
        animation_state: StringPtr,
        animation: StringPtr
    },

    // Tags
    Rect {},
    Loadable {},
    Renderable {},
    Initializable {},
    Constructable {},
    Connected {},
    Disconnected {},
    Local {},
    Remote {},
}

pub fn init() {    
    // Singletons
    GameConfig::register();
    KeyboardInput::register();
    WebSocket::register();
    
    // Components
    Position::register();
    Velocity::register();
    Size::register();
    Color::register();
    Sprite::register();
    NetworkEntity::register();
    Atlas::register();
    Skeleton::register();
    Images::register();
    BoneAnimation::register();

    // Tags
    Rect::register();
    Loadable::register();
    Renderable::register();
    Initializable::register();
    Constructable::register();
    Connected::register();
    Disconnected::register();
    Local::register();
    Remote::register();
}