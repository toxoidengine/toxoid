use toxoid_ffi::{toxoid_add_system, toxoid_api::{print_string, Query, System}, KeyboardInput, Position};

pub fn movement_system_fn(query: &mut Query) {
    let query_iter = query.iter();
    while query_iter.next() {
        let entities = query_iter.entities();
        let entity = entities.get(0);
        if entity.is_some() {
            let keyboard_input = entity.unwrap().get::<KeyboardInput>();

            let mut player_query = Query::new::<(Position,)>();
            let player_query_iter = player_query.iter();
            while player_query_iter.next() {
                let player_entities = player_query_iter.entities();
                let player_entity = player_entities.get(0);
                if player_entity.is_some() {
                    let mut player_pos = player_entity.unwrap().get::<Position>();
                    if keyboard_input.get_left() {
                        player_pos.set_x(player_pos.get_x() - 3);
                    }
                    if keyboard_input.get_right() {
                        player_pos.set_x(player_pos.get_x() + 3);
                    }
                    if keyboard_input.get_up() {
                        player_pos.set_y(player_pos.get_y() - 3);
                    }
                    if keyboard_input.get_down() {
                        player_pos.set_y(player_pos.get_y() + 3);
                    }
                }
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