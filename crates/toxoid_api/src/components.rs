use serde::{Deserialize, Serialize};
use toxoid_api_macro::component;
use crate::*;

pub enum DirectionEnum {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

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
    SpineInstance {
        instance: Pointer,
        instantiated: bool
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
    Direction {
        direction: u8
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
    RenderTarget {
        render_target: Pointer
    },
    Networked {
        network_id: u64,
        entity_id: u64,
        message: Pointer
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
    TiledWorldComponent {
        world: Pointer
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
    Player {},
    Map {},
    Updated {},
}

pub fn init() {    
    // Singletons
    GameConfig::register();
    KeyboardInput::register();
    WebSocket::register();
    SpineInstance::register();
    
    // Components
    Position::register();
    Velocity::register();
    Size::register();
    Color::register();
    Sprite::register();
    RenderTarget::register();
    Networked::register();
    Atlas::register();
    Skeleton::register();
    Images::register();
    BoneAnimation::register();
    Direction::register();
    TiledWorldComponent::register();

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
    Player::register();
    Map::register();
    Updated::register();
}