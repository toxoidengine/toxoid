#![allow(non_camel_case_types)]
#![allow(improper_ctypes)]
extern crate toxoid_ffi_macro;
// use toxoid_ffi_macro::Component;
use toxoid_ffi_macro::component;
use toxoid_ffi::*;
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

    Position::register();


    let type_id = core::any::TypeId::of::<Position>();
    toxoid_component_cache_insert(type_id, 420);
    let final_id = toxoid_component_cache_get(type_id);
    print_i32(final_id);


    // let mut map = HashMap::new();
    // map.insert(1, "one");
    // print_string(map.get(&1).unwrap());
    // let mut map = HashMap::new();
    // map.insert("Hello", "Hello TYPEID!");
    // map.insert(test, "Hello TYPEID!");
    // let value = map.get(&test).unwrap();
    // print_string(value);

    // let cache = crate::COMPONENT_ID_CACHE.as_mut().unwrap_unchecked();
        // cache.insert(type_id, component_id);
    
    // let ptr = ALLOCATOR.alloc(Layout::from_size_align(1, 1).unwrap());
    // ALLOCATOR.dealloc(ptr, Layout::from_size_align(1, 1).unwrap());

    // let mut vec = Vec::new();

    // for i in 1..10 {
    //     vec.push(i);
    // }

    // for item in vec.iter() {
    //     print_string("HELLO ITEMS!");
    // }

    // let mut map = HashMap::new();
    // map.insert(1, "one");
    // print_string(map.get(&1).unwrap());

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