use toxoid_api::*;
use crate::components::*;
use crate::entities::*;

// Use ECS hierarchy to recursively go through player entity children and delete the
// last child entity, which is the tail.
pub fn tail_last_recurse_delete(index: &mut u32, length: u32, entity: &mut Entity) {
    entity.children_each(|mut child_entity| {
        // Check if player because there is also a "renderable" child entity
        // That contains only render components rather than a player entity
        // That contains player info
        if child_entity.has::<Player>() {
            *index += 1;
            if *index == length {
                // child_entity.add::<Despawn>();
                World::delete_entity(child_entity);
            } else {
                tail_last_recurse_delete(index, length, &mut child_entity);
            }
        }
    });
}

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
                let position = entity.get::<Position>();
                let direction = World::get_singleton::<Direction>();
                let x = position.get_x();
                let y = position.get_y();
                let direction = direction.get_direction();

                if direction == DirectionEnum::Down as u8 {
                    create_player_block(x, y + 50, entity.get_id());
                } else if direction == DirectionEnum::Up as u8 {
                    create_player_block(x, y - 50, entity.get_id());
                } else if direction == DirectionEnum::Left as u8 {
                    create_player_block(x - 50, y, entity.get_id());
                } else if direction == DirectionEnum::Right as u8 {
                    create_player_block(x + 50, y, entity.get_id());
                }

                entity.remove::<Head>();
                entity.add::<Tail>();

                // Use ECS hierarchy to recursively go through player entity children and delete the
                // last child entity, which is the tail.
                let mut index = 0;
                tail_last_recurse_delete(&mut index, 3, entity);
            });
    }
    unsafe {
        FRAMES_SINCE_LAST_MOVE = 0;
    }
} 

fn aabb(a: Position, a2: Size, b: Position, b2: Size) -> bool {
    a.get_x() + a2.get_width() > b.get_x() &&
    a.get_x() < b.get_x() + b2.get_width() &&
    a.get_y() + a2.get_height() > b.get_y() &&
    a.get_y() < b.get_y() + b2.get_height()
}

pub fn input_system(query: &mut Query) {
    let query = query.iter();
    while query.next() {
        let entities = query.entities();
        entities
            .iter_mut()
            .for_each(|entity| {
                let mut direction = World::get_singleton::<Direction>();
                let mut keyboard_input = entity.get::<KeyboardInput>();
                if keyboard_input.get_up() {
                    direction.set_direction(DirectionEnum::Up as u8);
                    print_string("Hello world");
                    keyboard_input.set_up(false);
                }
                if keyboard_input.get_down() {
                    direction.set_direction(DirectionEnum::Down as u8);
                    keyboard_input.set_down(false);
                }
                if keyboard_input.get_left() {
                    direction.set_direction(DirectionEnum::Left as u8);
                    keyboard_input.set_left(false);
                }
                if keyboard_input.get_right() {
                    direction.set_direction(DirectionEnum::Right as u8);
                    keyboard_input.set_right(false);
                }
            });
    }
}

pub fn eat_system(query: &mut Query) {
    let query_iter = query.iter();
        while query_iter.next() {
            let entities = query_iter.entities();
            entities
                .iter_mut()
                .for_each(|player_entity| {
                    player_entity
                        .children_each(|player_child| {
                            if player_child.has::<Size>() {

                            }
                        });
                });
        }
}


pub fn init() {
    let mut movement_system = System::new(movement_system);
    let mut input_system = System::new(input_system);
    // let mut eat_system = System::new(eat_system);
    movement_system
        .with::<(Head, Player, Position, Direction)>()
        .build();
    input_system
        .with::<(KeyboardInput,)>()
        .build();
    // eat_system
    //     .with::<(Head, Player)>()
    //     .build();

    World::add_system(movement_system);
    World::add_system(input_system);
    // World::add_system(eat_system);
}