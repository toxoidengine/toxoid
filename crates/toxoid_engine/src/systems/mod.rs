pub mod input;
pub mod load;
pub mod render;
pub mod network;

pub use input::*;
pub use load::*;
pub use render::*;
pub use network::*;
use toxoid_api::*;

#[cfg(target_os = "emscripten")]
pub fn init() {
    // Loaders
    System::new(load_sprite_system)
        .with::<(Loadable, Sprite, Size, Position)>()
        .build();
    System::new(load_bone_animation_system)
        .with::<(Loadable, Atlas, Skeleton, Images)>()
        .build();

    // Renderers
    System::new(render_rect_system)
        .with::<(Sprite, Renderable, Size, Position)>()
        .build();
    System::new(render_sprite_system)
        .with::<(Rect, Renderable, Color, Size, Position)>()
        .build();
    System::new(draw_bone_animation)
        .with::<(Position, BoneAnimation, SpineInstance)>()
        .build();

    // Network
    System::new(network_event_system)
        .with::<(Updated, Networked, Local, Player)>()
        .build();

    // TODO: Remove
    System::new(input_system)
        .with::<(Position, BoneAnimation, SpineInstance, Local)>()
        .build();
}

#[cfg(not(target_os = "emscripten"))]
pub fn init() {}