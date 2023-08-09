#![allow(non_camel_case_types)]
#![allow(improper_ctypes)]
extern crate toxoid_ffi_macro;
use std::alloc::GlobalAlloc;

use toxoid_ffi::*;
use toxoid_ffi_macro::component;
pub mod local_ecs;
pub use local_ecs::*;

component! {
    Position {
        x: u32,
        y: u32,
    },
    // Velocity {
    //     dx: f32,
    //     dy: f32,
    // }
}

#[no_mangle]
pub unsafe extern "C" fn app_main() {
    let pos_id = Position::register();

    // Create a new entity.
    let mut player = Entity::new();
    // Add the component to the entity.
    player.add::<Position>();

    // Create a new entity.
    let mut player_2 = Entity::new();
    // Add the component to the entity.
    player_2.add::<Position>();
    
    let mut pos_component = player.get_component::<Position>();
    pos_component.set_x(420);
    pos_component.set_y(421);

    let mut pos_component_2 = player_2.get_component::<Position>();
    pos_component_2.set_x(777);
    pos_component_2.set_x(999);

    let mut query = Query::test::<(Position,)>();
    // // let mut query = Query::new(&mut [pos_id]);
    // let mut query = Query::new::<(Position,)>();
    // let query = query.iter();
    // while query.next() {
    //     let entities = query.entities();
    //     let test = entities.iter().map(|entity| {
    //         let pos = entity.get_component::<Position>();
    //         pos.get_ptr()
    //     });
    //     for ptr in test {
    //         print_i32(ptr as i32);
    //     }
    //     for entity in entities.iter() {
    //         let pos = entity.get_component::<Position>();
    //         print_i32(pos.get_x() as i32);
    //     }

    //     let pos = query.field::<Position>();
    //     pos
    //         .iter()
    //         .for_each(|pos| {
    //             print_string("Position X Value:");
    //             print_i32(pos.get_x() as i32);
    //             print_string("Position Y Value:");
    //             print_i32(pos.get_y() as i32);
    //         });
    //     // ALLOCATOR.dealloc(pos.as_ptr() as *mut u8, core::alloc::Layout::new::<Position>());
    // }
}
