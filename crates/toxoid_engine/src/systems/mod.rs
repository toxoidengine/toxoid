pub mod input;
pub mod render;
pub mod network;

#[cfg(feature = "client")]
pub use input::*;
pub use load::*;
pub use render::*;
pub use network::*;
use toxoid_api::*;

pub fn init() {
    #[cfg(feature = "render")] {
        // Renderers
        System::new(render_rect_system)
            .with::<(Rect, Renderable, Color, Size, Position)>()
            .build();
        System::new(render_sprite_system)
            .with::<(Sprite, Renderable, Size, Position)>()
            .build();
        System::new(render_rt_system)
            .with::<(RenderTarget, Renderable, Size, Position)>()
            .build();
        System::new(render_bone_animation)
            .with::<(SpineInstance, Position, BoneAnimation)>()
            .build();
    }
    
    // Network
    // #[cfg(feature = "net")]
    // System::new(network_event_system)
    //     .with::<(Updated, Networked, Local, Player)>()
    //     .build();
}