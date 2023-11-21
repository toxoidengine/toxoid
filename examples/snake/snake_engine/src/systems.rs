use core::cell::RefCell;

use toxoid_ffi::{toxoid_add_system, toxoid_api::{Query, System}, KeyboardInput, Position, Direction, DirectionEnum, Player, Food, Rect};
use toxoid_ffi::toxoid_api::*;
use toxoid_ffi::*;

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

// TODO: Possibly replace after we implement flecs filters
thread_local! {
    static PLAYER_QUERY: RefCell<Query> = RefCell::new(Query::new::<(Position, Direction, Player)>());
}

// Create AABB function in Rust
pub fn input_system_fn(query: &mut Query) {
    let query_iter = query.iter();
    while query_iter.next() {
        let entities = query_iter.entities();
        // TODO: Make this a FLECS singleton
        let entity = entities.get(0);
        if entity.is_some() {
            let keyboard_input = entity.unwrap().get::<KeyboardInput>();
            PLAYER_QUERY.with(|player_query_cell| {
                let mut player_query = player_query_cell.borrow_mut();
                let player_query_iter = player_query.iter();
                while player_query_iter.next() {
                    let player_entities = player_query_iter.entities();
                    player_entities
                        .iter()
                        .for_each(|player_entity| {
                            let mut player_dir = player_entity.get::<Direction>();
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
                        });
                }
            });
        }
    }
}

// For a 60 FPS rate, 6 frames is approximately 100ms
// For a 60 FPS rate, 30 frames is approximately 500ms
const FRAMES_PER_MOVE: u32 = 15;  
thread_local! {
    static FRAMES_SINCE_LAST_MOVE: std::cell::Cell<u32> = std::cell::Cell::new(0);
}

pub fn create_player_block(x: u32, y: u32, direction: u8, child: ecs_entity_t) {
    let mut player_entity = Entity::new();
    player_entity.add::<Position>();
    player_entity.add::<Direction>();
    player_entity.add::<Player>();
    let mut pos = player_entity.get::<Position>();
    pos.set_x(x);
    pos.set_y(y);
    let mut dir = player_entity.get::<Direction>();
    dir.set_direction(direction);
    player_entity.add::<Head>();
    player_entity.parent_of(Entity { id: child, children: &mut [] });
    // Child Entity
    let mut render_target = Entity::new();
    render_target.add::<Rect>();
    render_target.add::<Renderable>();
    render_target.add::<Color>();
    render_target.add::<Position>();
    render_target.child_of(player_entity);
    let mut rect = render_target.get::<Rect>();
    rect.set_width(50);
    rect.set_height(50);
    let mut color = render_target.get::<Color>();
    color.set_r(0);
    color.set_g(200);
    color.set_b(0);
    let mut render_pos = render_target.get::<Position>();
    render_pos.set_x(pos.get_x());
    render_pos.set_y(pos.get_y());
}

pub fn tail_last_recurse_delete(index: &mut u32, length: u32, entity: &mut Entity) {
    entity.children_each(|mut child_entity| {
        // Check if player because there is also a "renderable" child entity
        // That contains only render components rather than a player entity
        // That contains player info
        if child_entity.has::<Player>() {
            *index += 1;
            if *index == length {
                child_entity.add::<Despawn>();
            } else {
                tail_last_recurse_delete(index, length, &mut child_entity);
            }
        }
    });
}

thread_local! {
    static TAIL_LENGTH: std::cell::Cell<u32> = std::cell::Cell::new(2);
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
            // There is only one player entity with
            // component head (in the query)
            let player_entity = entities.get_mut(0).unwrap();
            let player_dir = player_entity.get::<Direction>();
            let player_pos = player_entity.get::<Position>();
            let direction = player_dir.get_direction();

            if player_dir.get_direction() == DirectionEnum::Down as u8 {
                create_player_block(player_pos.get_x(), player_pos.get_y() + 50, direction, player_entity.get_id());
            } else if player_dir.get_direction() == DirectionEnum::Up as u8 {
                create_player_block(player_pos.get_x(), player_pos.get_y() - 50, direction, player_entity.get_id());
            } else if player_dir.get_direction() == DirectionEnum::Left as u8 {
                create_player_block(player_pos.get_x() - 50, player_pos.get_y(), direction, player_entity.get_id());
            } else if player_dir.get_direction() == DirectionEnum::Right as u8 {
                create_player_block(player_pos.get_x() + 50, player_pos.get_y(), direction, player_entity.get_id());
            }
            // Recursively go through player entity children and delete the
            // last child entity, which is the tail.
            let mut index = 0;
            TAIL_LENGTH.with(|tail_length_cell| {
                let tail_length = tail_length_cell.get();
                tail_last_recurse_delete(&mut index, tail_length, player_entity);
            });
            player_entity.remove::<Head>();
        }

        // Reset the counter after moving.
        frames_cell.set(0);
    });
} 

fn despawn_system_fn(query: &mut Query) {
    let query_iter = query.iter();
    while query_iter.next() {
        let entities = query_iter.entities();
        entities
            .iter_mut()
            .for_each(|entity| {
                // println!("DELETED!");
                delete_entity_mut(entity);
            });
    }

}

// TODO: Possibly replace after we implement flecs filters
thread_local! {
    static FOOD_QUERY: RefCell<Query> = RefCell::new(Query::new::<(Food,)>());  
}

fn aabb(a: Position, a2: Rect, b: Position, b2: Rect) -> bool {
    a.get_x() + a2.get_width() > b.get_x() &&
    a.get_x() < b.get_x() + b2.get_width() &&
    a.get_y() + a2.get_height() > b.get_y() &&
    a.get_y() < b.get_y() + b2.get_height()
}

pub fn create_food_block() {
    let (random_x, random_y) = utils::gen_rng_grid_pos();

    let mut food_entity = Entity::new();
    food_entity.add::<Position>();
    food_entity.add::<Food>();

    let mut pos = food_entity.get::<Position>();
    pos.set_x(random_x as u32);
    pos.set_y(random_y as u32);

    let mut render_target = Entity::new();
    render_target.add::<Rect>();
    render_target.add::<Renderable>();
    render_target.add::<Color>();
    render_target.add::<Position>();
    render_target.child_of(food_entity);
    let mut rect = render_target.get::<Rect>();
    rect.set_width(50);
    rect.set_height(50);
    let mut color = render_target.get::<Color>();
    color.set_r(255);
    color.set_g(0);
    color.set_b(0);
    let mut render_pos = render_target.get::<Position>();
    render_pos.set_x(random_x as u32);
    render_pos.set_y(random_y as u32);
}

pub fn eat_system_fn(query: &mut Query) {
    let query_iter = query.iter();
        while query_iter.next() {
            let entities = query_iter.entities();
            entities
                .iter_mut()
                .for_each(|player_entity| {
                    player_entity
                        .children_each(|player_child| {
                            FOOD_QUERY.with(|food_query_cell| {
                                let mut food_query = food_query_cell.borrow_mut();
                                let food_query_iter = food_query.iter();
                                while food_query_iter.next() {
                                    food_query_iter
                                        .entities()
                                        .iter_mut()
                                        .for_each(|food_entity| {
                                            let food_entity_id = food_entity.get_id();
                                            food_entity.children_each(|food_child| {
                                                    let food_pos = food_child.get::<Position>();
                                                    let food_rect = food_child.get::<Rect>();
                                                    let player_child_pos = player_child.get::<Position>();
                                                    let player_child_rect = player_child.get::<Rect>();
                                                    if aabb(player_child_pos, player_child_rect, food_pos, food_rect) {
                                                        delete_entity(Entity { id: food_entity_id, children: &mut [] });
                                                        create_food_block();
                                                        TAIL_LENGTH.with(|tail_length_cell| {
                                                            let tail_length = tail_length_cell.get();
                                                            tail_length_cell.set(tail_length + 1);
                                                        });
                                                    }
                                            });
                                        });
                                }
                            });
                        });
                });
        }
}


pub fn init() {
    let input_system = System::new::<(KeyboardInput,)>(input_system_fn);
    let movement_system = System::new::<(Head, Player)>(movement_system_fn);
    let eat_system = System::new::<(Head, Player)>(eat_system_fn);
    let despawn_system = System::new::<(Despawn,)>(despawn_system_fn);
    unsafe {
        toxoid_add_system(input_system);
        toxoid_add_system(movement_system);
        toxoid_add_system(eat_system);
        toxoid_add_system(despawn_system);
    }
}