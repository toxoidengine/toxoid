use toxoid_api::*;
use crate::components::*;
use crate::entities::*;

// pub fn create_player_block(x: u32, y: u32, direction: u8, child: ecs_entity_t) {
//     let mut player_entity = Entity::new();
//     player_entity.add::<Position>();
//     player_entity.add::<Direction>();
//     player_entity.add::<Player>();
//     let mut pos = player_entity.get::<Position>();
//     pos.set_x(x);
//     pos.set_y(y);
//     let mut dir = player_entity.get::<Direction>();
//     dir.set_direction(direction);
//     player_entity.add::<Head>();
//     player_entity.parent_of(Entity { id: child, children: &mut [] });
//     // Child Entity
//     let mut render_target = Entity::new();
//     render_target.add::<Size>();
//     render_target.add::<Renderable>();
//     render_target.add::<Color>();
//     render_target.add::<Position>();
//     render_target.child_of(player_entity);
//     let mut rect = render_target.get::<Size>();
//     rect.set_width(50);
//     rect.set_height(50);
//     let mut color = render_target.get::<Color>();
//     color.set_r(0);
//     color.set_g(200);
//     color.set_b(0);
//     let mut render_pos = render_target.get::<Position>();
//     render_pos.set_x(pos.get_x());
//     render_pos.set_y(pos.get_y());
// }

// Basic Movement
// pub fn movement_system_fn(query: &mut Query) {
//     FRAMES_SINCE_LAST_MOVE.with(|frames_cell| {
//         let current_frames = frames_cell.get();
//         if current_frames < FRAMES_PER_MOVE {
//             frames_cell.set(current_frames + 1);
//             return;
//         } 

//         let query_iter = query.iter();
//         while query_iter.next() {
//             let entities = query_iter.entities();
//             // There is only one player entity with
//             // component head (in the query)
//             let player_entity = entities.get_mut(0).unwrap();
//             let player_dir = player_entity.get::<Direction>();
//             let player_pos = player_entity.get::<Position>();
//             let direction = player_dir.get_direction();

//             if player_dir.get_direction() == DirectionEnum::Down as u8 {
//                 create_player_block(player_pos.get_x(), player_pos.get_y() + 50, direction, player_entity.get_id());
//             } else if player_dir.get_direction() == DirectionEnum::Up as u8 {
//                 create_player_block(player_pos.get_x(), player_pos.get_y() - 50, direction, player_entity.get_id());
//             } else if player_dir.get_direction() == DirectionEnum::Left as u8 {
//                 create_player_block(player_pos.get_x() - 50, player_pos.get_y(), direction, player_entity.get_id());
//             } else if player_dir.get_direction() == DirectionEnum::Right as u8 {
//                 create_player_block(player_pos.get_x() + 50, player_pos.get_y(), direction, player_entity.get_id());
//             }
//             // Recursively go through player entity children and delete the
//             // last child entity, which is the tail.
//             let mut index = 0;
//             TAIL_LENGTH.with(|tail_length_cell| {
//                 let tail_length = tail_length_cell.get();
//                 tail_last_recurse_delete(&mut index, tail_length, player_entity);
//             });
//             player_entity.remove::<Head>();
//         }

//         // Reset the counter after moving.
//         frames_cell.set(0);
//     });
// } 

// For a 60 FPS rate, 6 frames is approximately 100ms
// For a 60 FPS rate, 30 frames is approximately 500ms
const FRAMES_PER_MOVE: u32 = 15;  
static mut FRAMES_SINCE_LAST_MOVE: u32 = 0;

pub fn movement_system(query: &mut Query) { 
    unsafe {
        if FRAMES_SINCE_LAST_MOVE < FRAMES_PER_MOVE {
            FRAMES_SINCE_LAST_MOVE += 1;
            return;
        } 
    }
    let query = query.iter();
    while query.next() {
        let entities = query.entities();
        entities
            .iter_mut()
            .for_each(|entity| {
                // print_string("Moving");
                let position = entity.get::<Position>();
                let direction = entity.get::<Direction>();
                let x = position.get_x();
                let y = position.get_y();
                let direction = direction.get_direction();

                if direction == DirectionEnum::Down as u8 {
                    create_player_block(x, y + 50, direction, entity.get_id());
                } else if direction == DirectionEnum::Up as u8 {
                    create_player_block(x, y - 50, direction, entity.get_id());
                } else if direction == DirectionEnum::Left as u8 {
                    create_player_block(x - 50, y, direction, entity.get_id());
                } else if direction == DirectionEnum::Right as u8 {
                    create_player_block(x + 50, y, direction, entity.get_id());
                }

                entity.remove::<Head>();
                entity.add::<Tail>();

                let length = entity.children.len();
                entity.children_each(|mut child_entity| {
                    // print_i32(child_entity.get_id() as i32);
                    if child_entity.has::<Tail>() && length > 1 {
                        // print_i32(child_entity.get_id() as i32);
                        // unsafe { toxoid_delete_entity(child_entity.get_id()) };
                    }
                });
            });
    }
    unsafe {
        FRAMES_SINCE_LAST_MOVE = 0;
    }
} 

pub fn init() {
    let movement_system = System::new::<(Head, Player, Position, Direction)>(movement_system);
    World::add_system(movement_system);
}