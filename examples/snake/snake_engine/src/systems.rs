use toxoid_ffi::{toxoid_add_system, toxoid_api::{Query, System}, KeyboardInput, Position, Direction, DirectionEnum};

// Basic Movement
// pub fn movement_system_fn(query: &mut Query) {
//     let query_iter = query.iter();
//     while query_iter.next() {
//         let entities = query_iter.entities();
//         let entity = entities.get(0);
//         if entity.is_some() {
//             let keyboard_input = entity.unwrap().get::<KeyboardInput>();

//             let mut player_query = Query::new::<(Position,)>();
//             let player_query_iter = player_query.iter();
//             while player_query_iter.next() {
//                 let player_entities = player_query_iter.entities();
//                 let player_entity = player_entities.get(0);
//                 if player_entity.is_some() {
//                     let mut player_pos = player_entity.unwrap().get::<Position>();
//                     if keyboard_input.get_left() {
//                         player_pos.set_x(player_pos.get_x() - 3);
//                     }
//                     if keyboard_input.get_right() {
//                         player_pos.set_x(player_pos.get_x() + 3);
//                     }
//                     if keyboard_input.get_up() {
//                         player_pos.set_y(player_pos.get_y() - 3);
//                     }
//                     if keyboard_input.get_down() {
//                         player_pos.set_y(player_pos.get_y() + 3);
//                     }
//                 }
//             }
//         }
//     }
// } 

pub fn input_system_fn(query: &mut Query) {
    let query_iter = query.iter();
    while query_iter.next() {
    let entities = query_iter.entities();
    let entity = entities.get(0);
    if entity.is_some() {
        let keyboard_input = entity.unwrap().get::<KeyboardInput>();
        let mut player_query = Query::new::<(Position, Direction)>();
        let player_query_iter = player_query.iter();
        while player_query_iter.next() {
            let player_entities = player_query_iter.entities();
            let player_entity = player_entities.get(0);
                let mut player_dir = player_entity.unwrap().get::<Direction>();
                if keyboard_input.get_up() {
                    if player_dir.get_direction() == DirectionEnum::Left as u8 || player_dir.get_direction() == DirectionEnum::Right as u8 {
                        player_dir.set_direction(DirectionEnum::Up as u8);
                    }
                }
                if keyboard_input.get_down() {
                    if player_dir.get_direction() == DirectionEnum::Left as u8 || player_dir.get_direction() == DirectionEnum::Right as u8 {
                        player_dir.set_direction(DirectionEnum::Down as u8);
                    }
                }
                if keyboard_input.get_left() {
                    if player_dir.get_direction() == DirectionEnum::Up as u8 || player_dir.get_direction() == DirectionEnum::Down as u8 {
                        player_dir.set_direction(DirectionEnum::Left as u8);
                    }
                }
                if keyboard_input.get_right() {
                    if player_dir.get_direction() == DirectionEnum::Up as u8 || player_dir.get_direction() == DirectionEnum::Down as u8 {
                        player_dir.set_direction(DirectionEnum::Right as u8);
                    }
                }
            }
        }
    }
}

// For a 60 FPS rate, 6 frames is approximately 100ms
// For a 60 FPS rate, 30 frames is approximately 500ms
const FRAMES_PER_MOVE: u32 = 30;  
thread_local! {
    static FRAMES_SINCE_LAST_MOVE: std::cell::Cell<u32> = std::cell::Cell::new(0);
}
pub fn movement_system_fn(query: &mut Query) {
    FRAMES_SINCE_LAST_MOVE.with(|frames_cell| {
        let current_frames = frames_cell.get();
        if current_frames < FRAMES_PER_MOVE {
            frames_cell.set(current_frames + 1);
            return;
        } 

        let query_iter = query.iter();
        while query_iter.next() {
            let entities = query_iter.entities();
            let player_entity = entities.get(0);
            if player_entity.is_some() {
                let player_entity = player_entity.unwrap();
                let mut player_pos = player_entity.get::<Position>();
                let player_dir = player_entity.get::<Direction>();
                let children = player_entity.children();
                if player_dir.get_direction() == DirectionEnum::Up as u8 {
                    player_pos.set_y(player_pos.get_y() - 50);
                    children.iter().for_each(|child| {
                        let mut child_pos = child.get::<Position>();
                        child_pos.set_y(child_pos.get_y() - 50);
                    });
                } else if player_dir.get_direction() == DirectionEnum::Down as u8 {
                    player_pos.set_y(player_pos.get_y() + 50);
                    children.iter().for_each(|child| {
                        let mut child_pos = child.get::<Position>();
                        child_pos.set_y(child_pos.get_y() + 50);
                    });
                } else if player_dir.get_direction() == DirectionEnum::Left as u8 {
                    player_pos.set_x(player_pos.get_x() - 50);
                    children.iter().for_each(|child| {
                        let mut child_pos = child.get::<Position>();
                        child_pos.set_x(child_pos.get_x() - 50);
                    });
                } else if player_dir.get_direction() == DirectionEnum::Right as u8 {
                    player_pos.set_x(player_pos.get_x() + 50);
                    children.iter().for_each(|child| {
                        let mut child_pos = child.get::<Position>();
                        child_pos.set_x(child_pos.get_x() + 50);
                    });
                }
            }
        }

        // Reset the counter after moving.
        frames_cell.set(0);
    });
} 

pub fn init() {
    let input_system = System::new::<(KeyboardInput,)>(input_system_fn);
    let movement_system = System::new::<(Position, Direction)>(movement_system_fn);
    unsafe {
        toxoid_add_system(input_system);
        toxoid_add_system(movement_system);
    }
}