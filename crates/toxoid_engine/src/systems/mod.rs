pub mod input;
pub mod load;
pub mod render;
pub mod network;

pub use input::*;
pub use load::*;
pub use render::*;
pub use network::*;
use toxoid_api::*;

// pub fn test_system(query_iter: *mut core::ffi::c_void) {
//     let mut query_iter = Iter::from(query_iter);
//     while query_iter.next() {
//         let entities = query_iter.entities();
//         entities
//             .iter_mut()
//             .for_each(|entity| {
//                 let mut position = entity.get::<Position>();
//                 position.set_x(5);
//                 position.set_y(5);
//             });
//     }
// }

// pub fn test_system_2(query_iter: *mut core::ffi::c_void) {
//     let query_iter = Iter::from(query_iter);
//     let positions = query_iter.field_mut::<Position>(1); 
//     for i in 0..query_iter.count() {
//         let p = positions.get_mut(i as usize).unwrap();
//         p.set_x(99);
//         p.get_x();
//     }
// }

#[cfg(target_os = "emscripten")]
pub fn init() {
    // let mut player_entity = Entity::new();
    // player_entity.add::<Position>();
    // System::new(test_system)
    //     .with::<(Position,)>()
    //     .build();
    // System::new(test_system_2)
    //     .with::<(Position,)>()
    //     .build();
    
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
pub fn init() {
}