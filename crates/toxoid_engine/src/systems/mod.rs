pub mod input;
pub mod load;
pub mod render;
pub mod network;

pub use input::*;
pub use load::*;
pub use render::*;
pub use network::*;

use toxoid_api::*;

pub fn test_system(query_iter: *mut core::ffi::c_void) {
    let mut query_iter = Iter::from(query_iter);
    while query_iter.next() {
        println!("Hello from system!");
        let entities = query_iter.entities();
        entities
            .iter_mut()
            .for_each(|entity| {
                let mut position = entity.get::<Position>();
                position.set_x(5);
                position.set_y(5);

                let pos = position.get_y();
                println!("Position test: {}", pos);
            });
    }
}

#[cfg(target_os = "emscripten")]
pub fn init() {
    let mut player_entity = Entity::new();
    player_entity.add::<Position>();
    System::new(test_system)
        .with::<(Position,)>()
        .build();
    // // Loaders
    // let mut load_sprite_system = System::new(load_sprite_system);
    // let mut load_bone_animation_system = System::new(load_bone_animation_system);
    // load_sprite_system
    //     .with::<(Loadable, Sprite, Size, Position)>()
    //     .build();
    // load_bone_animation_system
    //     .with::<(Loadable, Atlas, Skeleton, Images)>()
    //     .build();
    // World::add_system(load_sprite_system);
    // World::add_system(load_bone_animation_system);

    // // Renderers
    // let mut render_rect_system = System::new(render_rect_system);
    // let mut render_sprite_system = System::new(render_sprite_system);
    // let mut draw_bone_animation = System::new(draw_bone_animation);
    // render_sprite_system
    //     .with::<(Sprite, Renderable, Size, Position)>()
    //     .build();
    // render_rect_system
    //     .with::<(Rect, Renderable, Color, Size, Position)>()
    //     .build();
    // draw_bone_animation
    //     .with::<(Position, BoneAnimation, SpineInstance)>()
    //     .build();
    // World::add_system(render_rect_system);
    // World::add_system(render_sprite_system);
    // World::add_system(draw_bone_animation);

    // // Network
    // let mut network_event_system = System::new(network_event_system);
    // network_event_system
    //     .with::<(Updated, Networked, Local, Player)>()
    //     .build();
    // World::add_system(network_event_system);

    // // TODO: Remove
    // let mut input_system = System::new(input_system);
    // input_system
    //     .with::<(Position, BoneAnimation, SpineInstance, Local)>()
    //     .build();
    // World::add_system(input_system);
}

#[cfg(not(target_os = "emscripten"))]
pub fn init() {
}