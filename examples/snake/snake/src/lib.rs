#![allow(non_camel_case_types)]
#![allow(improper_ctypes)]
extern crate toxoid_ffi_macro;
use toxoid_ffi::*;
use toxoid_ffi_macro::component;
pub mod ecs;
pub use ecs::*;

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
    player.add(pos_id);

    // Create a new entity.
    let mut player_2 = Entity::new();
    // Add the component to the entity.
    player_2.add(pos_id);
    
    let mut pos_component = player.get_component::<Position>();
    print_i32(pos_component.get_x() as i32);
    pos_component.set_x(420);

    let mut pos_component_2 = player_2.get_component::<Position>();
    print_i32(pos_component_2.get_x() as i32);
    pos_component_2.set_x(777);

    let mut query = Query::new(&mut [pos_id]);
    let query = query.iter();
    while query.next() {
        let entities = query.entities();
        for entity in entities.iter() {
            let pos = entity.get_component::<Position>();
            print_i32(entity.get_id());
            print_i32(pos.get_x() as i32);
        }
        
        // let field = query.field::<Position>();
        // print_string("Field length:");
        // print_i32(field.as_slice().len() as i32);
    }
}
