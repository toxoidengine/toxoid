use toxoid_ffi::{toxoid_add_system, toxoid_api::{print_string, Query, System}, KeyboardInput};

pub fn movement_system_fn(query: &mut Query) {
    let query_iter = query.iter();
    while query_iter.next() {
        let entities = query_iter.entities();
        let entity = entities.get(0);
        if entity.is_some() {
            let keyboard_input = entity.unwrap().get::<KeyboardInput>();
            if keyboard_input.get_left() {
                print_string("Left");
            }
            if keyboard_input.get_right() {
                print_string("Right");
            }
            if keyboard_input.get_up() {
                print_string("Up");
            }
            if keyboard_input.get_down() {
                print_string("Down");
            }
        }
        
    }
} 

pub fn init() {
    let movement_system = System::new::<(KeyboardInput,)>(movement_system_fn);
    unsafe {
        toxoid_add_system(movement_system);
    }
}