pub mod input;
pub mod render;
pub mod network;

#[cfg(feature = "client")]
pub use input::*;
pub use load::*;
#[cfg(feature = "render")]
pub use render::*;
pub use network::*;
use toxoid_api::*;

#[cfg(feature = "render")]
static mut CURRENT_ANIMATION: &str = "idle_down";

// TODO: Remove from engine, and make animation name and position update through flecs Observables (To be implemented)
#[cfg(feature = "render")]
pub fn animation_input_system(iter: &mut Iter) {
    let entities = iter.entities();
    entities
        .iter_mut()
        .for_each(|entity| {
            use toxoid_sokol::bindings::*;
            let spine_instance = entity.get::<SpineInstance>();
            if spine_instance.get_instantiated() {
                let instance = spine_instance.get_instance().ptr as *mut sspine_instance;
                let spine_instance = entity.get::<Skeleton>();
                let spine_skeleton = spine_instance.get_skeleton().ptr as *mut sspine_skeleton;

                let keyboard_input = World::get_singleton::<KeyboardInput>();

                if !keyboard_input.get_up() && !keyboard_input.get_down() && !keyboard_input.get_left() && !keyboard_input.get_right() {
                    let direction = entity.get::<Direction>();
                    unsafe {
                        if direction.get_direction() == DirectionEnum::Up as u8 && CURRENT_ANIMATION != "idle_up" {
                            // configure a simple animation sequence
                            sspine_set_animation(*instance, sspine_anim_by_name(*spine_skeleton, make_c_string("idle_up")), 0, true);
                            CURRENT_ANIMATION = "idle_up";
                        }
                        if direction.get_direction() == DirectionEnum::Down as u8 && CURRENT_ANIMATION != "idle_down"{
                            sspine_set_animation(*instance, sspine_anim_by_name(*spine_skeleton, make_c_string("idle_down")), 0, true);
                            CURRENT_ANIMATION = "idle_down";
                        }
                        if direction.get_direction() == DirectionEnum::Left as u8 && CURRENT_ANIMATION != "idle_left" {
                            sspine_set_animation(*instance, sspine_anim_by_name(*spine_skeleton, make_c_string("idle_left")), 0, true);
                            CURRENT_ANIMATION = "idle_left";
                        }
                        if direction.get_direction() == DirectionEnum::Right as u8 && CURRENT_ANIMATION != "idle_right" {
                            sspine_set_animation(*instance, sspine_anim_by_name(*spine_skeleton, make_c_string("idle_right")), 0, true);
                            CURRENT_ANIMATION = "idle_right";
                        }
                    }
                }
                unsafe {
                    if keyboard_input.get_up() && CURRENT_ANIMATION != "walk_up" { 
                        sspine_set_animation(*instance, sspine_anim_by_name(*spine_skeleton, make_c_string("walk_up")), 0, true);
                        CURRENT_ANIMATION = "walk_up";
                    }
                    if keyboard_input.get_down() && CURRENT_ANIMATION != "walk_down" {
                        sspine_set_animation(*instance, sspine_anim_by_name(*spine_skeleton, make_c_string("walk_down")), 0, true);
                        CURRENT_ANIMATION = "walk_down";
                    }
                    if keyboard_input.get_left() && CURRENT_ANIMATION != "walk_left" {
                        sspine_set_animation(*instance, sspine_anim_by_name(*spine_skeleton, make_c_string("walk_left")), 0, true);
                        CURRENT_ANIMATION = "walk_left";
                    }
                    if keyboard_input.get_right() && CURRENT_ANIMATION != "walk_right" {
                        sspine_set_animation(*instance, sspine_anim_by_name(*spine_skeleton, make_c_string("walk_right")), 0, true);
                        CURRENT_ANIMATION = "walk_right";
                    }
                }
            }
        });
}

pub fn init() {
    render::init();

    #[cfg(feature = "render")]
    System::new(animation_input_system)
            .with::<(SpineInstance, Position)>()
            .build();

    // Network
    // #[cfg(feature = "net")]
    // System::new(network_event_system)
    //     .with::<(Updated, Networked, Local, Player)>()
    //     .build();
}