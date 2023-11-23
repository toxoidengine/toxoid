#![allow(non_camel_case_types)]
#![allow(improper_ctypes)]
extern crate toxoid_api_macro;

use toxoid_api::*;
// use toxoid_api_macro::component;
// pub mod local_ecs;
// pub use local_ecs::*;
mod components;

pub fn update_fn(_query: &mut Query) {
    print_string("Hello from snake loop!");
    // let query = query.iter();
    // while query.next() {
    //     let pos = query.field::<Position>();
    //     let vel = query.field::<Velocity>();
    //     // pos.iter().for_each(|pos| {
    //     //     print_string("Position X Value:");
    //     //     print_i32(pos.get_x() as i32);
    //     //     print_string("Position Y Value:");
    //     //     print_i32(pos.get_y() as i32);
    //     // });
    //     // vel.iter().for_each(|vel| {
    //     //     print_string("Velocity X Value:");
    //     //     print_i32(vel.get_dx() as i32);
    //     //     print_string("Velocity Y Value:");
    //     //     print_i32(vel.get_dy() as i32);
    //     // });
    //     // ALLOCATOR.dealloc(pos.as_ptr() as *mut u8, core::alloc::Layout::new::<Position>());
    // }
} 

#[no_mangle]
pub unsafe extern "C" fn app_main() {
    components::init();
    // Position::register();
    // Velocity::register();

    // // Create a new entity.
    // let mut player = Entity::new();
    // // Add the component to the entity.
    // player.add::<Position>();
    // player.add::<Velocity>();

    // // Create a new entity.
    // let mut player_2 = Entity::new();
    // // Add the component to the entity.
    // player_2.add::<Position>();
    // player_2.add::<Velocity>();
    
    // let mut pos_component = player.get::<Position>();
    // pos_component.set_x(420);
    // pos_component.set_y(421);

    // let mut pos_component_2 = player_2.get::<Position>();
    // pos_component_2.set_x(777);
    // pos_component_2.set_y(999);

    // let mut vel_component = player.get::<Velocity>();
    // vel_component.set_dx(1.0);
    // vel_component.set_dy(2.0);

    // let mut vel_component_2 = player_2.get::<Velocity>();
    // vel_component_2.set_dx(3.0);
    // vel_component_2.set_dy(4.0);

    // let mut query = Query::new::<(Position, Velocity)>();
    // let query = query.iter();
    // while query.next() {
    //     let pos = query.field::<Position>();
    //     let vel = query.field::<Velocity>();
    //     // pos.iter().for_each(|pos| {
    //     //     print_string("Position X Value:");
    //     //     print_i32(pos.get_x() as i32);
    //     //     print_string("Position Y Value:");
    //     //     print_i32(pos.get_y() as i32);
    //     // });
    //     // vel.iter().for_each(|vel| {
    //     //     print_string("Velocity X Value:");
    //     //     print_i32(vel.get_dx() as i32);
    //     //     print_string("Velocity Y Value:");
    //     //     print_i32(vel.get_dy() as i32);
    //     // });
    //     // ALLOCATOR.dealloc(pos.as_ptr() as *mut u8, core::alloc::Layout::new::<Position>());
    // }

    // let system = System::new::<(Position, Velocity)>(update_fn);
    // toxoid_add_system(system);
}
