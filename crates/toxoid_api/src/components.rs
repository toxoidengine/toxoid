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
        sprite: Pointer,
        data: Pointer,
        data_size: i32, 
        renderable: bool
    },
    RenderTarget {
        render_target: Pointer
    },
    Networked {
        network_id: u64,
        entity_id: u64
    },
    WebSocket {
        socket: Pointer,
        connected: bool
    },
    Atlas {
        atlas: Pointer,
        filename: StringPtr,
        data: Pointer,
        data_size: i32,
        loaded: bool,
    },
    Skeleton {
        skeleton: Pointer,
        filename: StringPtr,
        data: Pointer,
        data_size: i32,
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
    TiledCellComponent {
        cell: Pointer,
        index: u32
    },
    TilesetComponent {},
    Callback {
        callback: Pointer
    },

    // Tags
    Rect {},
    Local {},
    Remote {},
    Player {},

    // States
    Loadable {},
    Blittable {},
    Renderable {},
    Connected {},
    Disconnected {},
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
    RenderTarget::register();
    Networked::register();
    Atlas::register();
    Skeleton::register();
    Images::register();
    BoneAnimation::register();
    Direction::register();
    TiledWorldComponent::register();
    TiledCellComponent::register();
    TilesetComponent::register();
    SpineInstance::register();
    Callback::register();

    // Tags
    Rect::register();
    Loadable::register();
    Blittable::register();
    Renderable::register();
    Connected::register();
    Disconnected::register();
    Local::register();
    Remote::register();
    Player::register();
}