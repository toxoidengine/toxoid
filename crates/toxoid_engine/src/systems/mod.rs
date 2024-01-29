pub mod input;
pub mod load;
pub mod render;
pub mod network;

pub use input::*;
pub use load::*;
pub use render::*;
pub use network::*;

use toxoid_api::*;

use std::sync::Mutex;

pub static mut GAMEPLAY_SYSTEMS: Mutex<Vec<toxoid_api::System>> = Mutex::new(Vec::new());
pub static mut RENDER_SYSTEMS: Mutex<Vec<toxoid_api::System>> = Mutex::new(Vec::new());

#[no_mangle]
pub unsafe extern "C" fn toxoid_add_system(
    system: toxoid_api::System
) {
    let render_systems = unsafe { &mut *RENDER_SYSTEMS.lock().unwrap() };
    render_systems.push(system);
}

#[cfg(target_os = "emscripten")]
pub fn init() {
    // Loaders
    let mut load_sprite_system = System::new(load_sprite_system);
    let mut load_bone_animation_system = System::new(load_bone_animation_system);
    load_sprite_system
        .with::<(Loadable, Sprite, Size, Position)>()
        .build();
    load_bone_animation_system
        .with::<(Loadable, Atlas, Skeleton, Images)>()
        .build();
    World::add_system(load_sprite_system);
    World::add_system(load_bone_animation_system);

    // Renderers
    let mut render_rect_system = System::new(render_rect_system);
    let mut render_sprite_system = System::new(render_sprite_system);
    let mut draw_bone_animation = System::new(draw_bone_animation);
    render_sprite_system
        .with::<(Sprite, Renderable, Size, Position)>()
        .build();
    render_rect_system
        .with::<(Rect, Renderable, Color, Size, Position)>()
        .build();
    draw_bone_animation
        .with::<(Position, BoneAnimation, SpineInstance)>()
        .build();
    World::add_system(render_rect_system);
    World::add_system(render_sprite_system);
    World::add_system(draw_bone_animation);

    // Network
    let mut network_event_system = System::new(network_event_system);
    network_event_system
        .with::<(Updated, Networked, Local, Player)>()
        .build();
    World::add_system(network_event_system);

    // TODO: Remove
    let mut input_system = System::new(input_system);
    input_system
        .with::<(Position, BoneAnimation, SpineInstance, Local)>()
        .build();
    World::add_system(input_system);
}

#[cfg(not(target_os = "emscripten"))]
pub fn init() {
    
}