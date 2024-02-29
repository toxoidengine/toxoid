pub mod input;
pub mod load;
pub mod render;
pub mod network;

#[cfg(feature = "client")]
pub use input::*;
pub use load::*;
pub use render::*;
pub use network::*;
use toxoid_api::*;

pub fn network_input_system(iter: &mut Iter) {
    let keyboard_input = World::get_singleton::<KeyboardInput>();
    let entities = iter.entities();
    entities
        .iter_mut()
        .for_each(|entity| {
            let direction = entity.get::<Direction>();
            if keyboard_input.get_up() {
                toxoid_net::send_components(entity.get_id(), &[direction.clone()], "PlayerMove".to_string());
            }
            if keyboard_input.get_down() {
                toxoid_net::send_components(entity.get_id(), &[direction.clone()], "PlayerMove".to_string());
            }
            if keyboard_input.get_left() {
                toxoid_net::send_components(entity.get_id(), &[direction.clone()], "PlayerMove".to_string());
            }
            if keyboard_input.get_right() {
                toxoid_net::send_components(entity.get_id(), &[direction], "PlayerMove".to_string());
            }
        });
}

pub fn init() {
    // // Loaders
    // System::new(load_sprite_system)
    //     .with::<(Loadable, Sprite, Size, Position)>()
    //     .build();
    // System::new(load_bone_animation_system)
    //     .with::<(Loadable, Atlas, Skeleton, Images)>()
    //     .build();

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
        System::new(draw_bone_animation)
            .with::<(Position, BoneAnimation, SpineInstance)>()
            .build();
    }
    
    // Network
    // #[cfg(feature = "net")]
    // System::new(network_event_system)
    //     .with::<(Updated, Networked, Local, Player)>()
    //     .build();

    System::new(network_input_system)
        .with::<(Player, Sprite, Renderable, Size, Position)>()
        .build();
}