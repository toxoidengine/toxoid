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

    print_string("Original Pointer value:");
    print_i32(pos_component.ptr as i32);

    let mut query = Query::new(&mut [pos_id]);
    let query = query.iter();
    while query.next() {
        let entities = query.entities();
        for entity in entities.iter() {
            let pos = entity.get_component::<Position>();
            print_i32(entity.get_id());
            print_i32(pos.get_x() as i32);
        }
        
        let (field_ptr, count) = query.field::<Position>();
        let field = core::slice::from_raw_parts(*field_ptr as *mut Position, count as usize);
        for i in 0..count {
            print_string("Field iterator pointer value:");
            print_i32(field[i as usize].ptr as i32);
            print_string("X value:");
            print_i32(field[i as usize].get_x() as i32);
        }

        // Iterate over field
        // print_i32(field[0 as usize].get_x() as i32);

        // let mut components = Vec::<i32>::new();
        // for i in 0..30 {
        //     components.push(i);
        //     print_i32(components.as_slice()[i as usize]);
        // }
    }
}
