#![allow(non_camel_case_types)]
#![allow(improper_ctypes)]
extern crate toxoid_ffi_macro;
// use toxoid_ffi_macro::Component;
use toxoid_ffi::*;
use toxoid_ffi_macro::component;
pub mod ecs;
pub use ecs::*;

// #[derive(Component)]
// pub struct Position
//     x: u32,
//     y: u32,
// }

//     dy: f32,
// }

component! {
    Position {
        x: u32,
        y: u32,
    },
    Velocity {
        dx: f32,
        dy: f32,
    }
}

#[no_mangle]
pub unsafe extern "C" fn app_main() {
    let vel = Velocity::default();
    let pos = Position::default();

    let pos_id = Position::register();

    // Create a new entity.
    let mut player = Entity::new();
    // Add the component to the entity.
    player.add(pos_id);
    let pos_component = player.get_component::<Position>();
    print_i32(pos_component.ptr as i32);

    let type_id = core::any::TypeId::of::<Position>();
    let pos_id = toxoid_component_cache_get(type_id);
    print_i32(pos_id);

    // // Create a new tag.
    // let tag = register_tag("LocalPlayer");
    // // Print the name of the tag.
    // toxoid_entity_get_name(tag);

    // // Create a new component.
    // let mut position = Position { x: 0, y: 0 };
    // // Set the values of the component.
    // position.set_x(77);
    // position.set_y(99);
    // // Print the values of the component.
    // print_string("X:");
    // print_i32(position.x as i32);
    // print_string("Y:");
    // print_i32(position.y as i32);

    // // // Register the component.
    // let pos_id = Position::register();
    // let vel_id = Velocity::register();

    // // Print the name of the component.
    // toxoid_entity_get_name(pos_id);
    // toxoid_entity_get_name(vel_id);

    // // Create a new entity.
    // let mut player = Entity::new();
    // // Add the component to the entity.
    // player.add(pos_id);
    // player.add(vel_id);
    // player.add_tag(tag);

    // let mut player_2 = Entity::new();
    // // Add the component to the entity.
    // player_2.add(pos_id);
    // player_2.add(vel_id);
    // player_2.add_tag(tag);

    // let mut player_3 = Entity::new();
    // // Add the component to the entity.
    // player_3.add(pos_id);
    // player_3.add(vel_id);
    // player_3.add_tag(tag);

    // let mut query = Query::new(&mut [pos_id, vel_id]);
    // let query = query.iter();
    // while query.next() {
    //     let _field = query.field();
    //     let entities = query.entities();
    //     for entity in entities.iter() {
    //         print_i32(entity.get_id());

    //         let pos = entity.get_component::<Position>();
    //     }
    // }
}
